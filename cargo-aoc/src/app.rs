use crate::{
    credentials::CredentialsManager, date, project::ProjectManager, Bench, Credentials, Input,
};
use aoc_runner_internal::{Day, Part};
use date::AOCDate;
use reqwest::{
    header::{HeaderMap, COOKIE, USER_AGENT},
    StatusCode,
};
use std::io::Write;
use std::path::Path;
use std::process;
use std::{error, sync::Arc};
use std::{
    error::Error,
    fs::{self, File},
};

use crate::Cli;

const CARGO_AOC_USER_AGENT: &str = "github.com/gobanos/cargo-aoc by gregory.obanos@gmail.com";

pub fn execute_credentials(args: &Credentials) {
    let mut creds_manager = CredentialsManager::new();

    if let Some(new_session) = &args.set {
        // Tries to set the session token
        match creds_manager.set_session_token(new_session.to_owned()) {
            Ok(()) => println!("Credentials sucessfully changed!"),
            Err(e) => println!("Error changing credentials: {}", e),
        }
    }

    // Displays the stored session token
    match creds_manager.get_session_token() {
        Ok(cred) => println!("Current credentials: {}", cred),
        Err(e) => println!("Error: {}", e),
    }
}

/// Executes the "input" subcommand of the app
pub fn execute_input(args: &Input) -> Result<(), Box<dyn Error>> {
    // Gets the token or exit if it's not referenced.
    let token = CredentialsManager::new().get_session_token().expect(
        "Error: you need to setup your AOC token using \"cargo aoc credentials {token}\"",
    );

    let pm = ProjectManager::new()?;

    let formated_token = format!("session={}", token);
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, CARGO_AOC_USER_AGENT.parse().unwrap());
    headers.insert(COOKIE, formated_token.parse().unwrap());

    let generate = args.generate;
    if args.all {
        let year = args
            .year
            .expect("Need to specify a year to run cargo-aoc input --all");
        {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let client = reqwest::Client::builder()
                    .default_headers(headers)
                    .build()
                    .unwrap();
                let client = Arc::new(client);
                let mut tasks = Vec::new();
                for day in 1..26u32 {
                    let client = client.clone();
                    let pm = pm.clone();
                    tasks.push(tokio::spawn(async move {
                        let date = AOCDate { day, year };
                        match download_input_async(date, &client).await {
                            Ok(_) => println!("Successfully downloaded day {day}"),
                            Err(e) => eprintln!("{e}"),
                        };
                        if generate {
                            match codegen(day, &pm) {
                                Ok(_) => {
                                    println!("Successfully generated boilerplate for day {day}")
                                }
                                Err(e) => eprintln!("{e}"),
                            }
                        }
                        day
                    }));
                }
                let mut results = Vec::new();
                for task in tasks {
                    let r = task.await;
                    if let Ok(r) = r {
                        results.push(r);
                    }
                }
                results.sort_unstable();
                for i in results {
                    let _ =
                        update_lib_rs(i, &pm).map_err(|e| eprintln!("Couldn't update lib.rs: {e}"));
                }
            });
        };
        return Ok(());
    }

    // Creates the AOCDate struct from the arguments (defaults to today...)
    let date: AOCDate = AOCDate::new(args);
    download_input(date)?;

    if generate {
        update_lib_rs(date.day, &pm)?;
        codegen(date.day, &pm)?;
        println!("Successfully generated boilerplate for {}", date.day);
    }
    Ok(())
}

fn update_lib_rs(day: u32, pm: &ProjectManager) -> Result<(), Box<dyn Error>> {
    let lib_rs_path = Path::new(pm.lib_path.as_deref().unwrap_or("src/lib.rs"));
    if !lib_rs_path.exists() {
        Err("lib.rs does not exist!")?
    }

    let lib_rs = fs::read_to_string(lib_rs_path)?;

    let str = format!("mod day{day};");
    if !lib_rs.contains(&str) {
        let lib_rs = format!("{str}\n{lib_rs}");
        fs::write(lib_rs_path, lib_rs)?;
    } else {
        eprintln!("lib.rs already contains {str}. Skipping...");
    }
    Ok(())
}

fn codegen(day: u32, pm: &ProjectManager) -> Result<(), Box<dyn Error>> {
    let src_dir = pm
        .lib_path
        .as_deref()
        .map(Path::new)
        .and_then(|lib_path| lib_path.parent())
        .unwrap_or(Path::new("src"));
    let filename = src_dir.join(format!("day{day}.rs"));
    if filename.exists() {
        eprintln!("{filename:?} already exists. Skipping...");
        return Ok(());
    }
    let code = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/template/src/day.rs.tpl"
    ))
    .replace("{DAY}", &format!("day{day}"));
    fs::write(filename, code)?;
    Ok(())
}

// The client should have the appropriate headers set
async fn download_input_async(
    date: AOCDate,
    client: &reqwest::Client,
) -> Result<(), Box<dyn error::Error>> {
    let filename = date.filename();
    let filename = Path::new(&filename);

    if filename.exists() {
        return Ok(());
    }

    let response = client.get(&date.request_url()).send().await?;

    match response.status() {
        StatusCode::OK => {
            let dir = date.directory();
            // Creates the file-tree to store inputs
            // TODO: Maybe use crate's infos to get its root in the filesystem ?
            fs::create_dir_all(&dir)?;

            // Gets the body from the response and outputs everything to a file
            let body = response
                .text()
                .await
                .map_err(|e| format!("Can't convert response to text: {e:?}"))?;
            let mut file = File::create(filename)?;
            file.write_all(body.as_bytes())?;
            Ok(())
        }
        StatusCode::NOT_FOUND => Err(format!("Day {} not yet ready", date.day))?,
        sc => Err(format!(
            "Could not find corresponding input. Is the token correct?\n\
                Status: {}\n\n\
                Message: {}",
            sc,
            response.text().await.unwrap_or_else(|_| String::new())
        ))?,
    }
}

fn download_input(date: AOCDate) -> Result<(), Box<dyn error::Error>> {
    let filename = date.filename();
    let filename = Path::new(&filename);

    if filename.exists() {
        return Ok(());
    }

    let token = CredentialsManager::new().get_session_token()?;
    // Creates an HTTP Client
    let client = reqwest::blocking::Client::new();
    // Cookie formatting ...
    let formated_token = format!("session={}", token);

    let response = client
        .get(date.request_url())
        .header(USER_AGENT, CARGO_AOC_USER_AGENT)
        .header(COOKIE, formated_token)
        .send()?;

    match response.status() {
            StatusCode::OK => {
                let dir = date.directory();
                // Creates the file-tree to store inputs
                // TODO: Maybe use crate's infos to get its root in the filesystem ?
                fs::create_dir_all(dir)?;

                // Gets the body from the response and outputs everything to a file
                let body = response.text()?;
                let mut file = File::create(filename)?;
                file.write_all(body.as_bytes())?;
            }
            sc => return Err(format!(
                "Could not find corresponding input. Are the day, year, and token correctly set ? Status: {}\
                 Message: {}", sc, response.text().unwrap_or_else(|_| String::new())
            ).into()),
        }

    Ok(())
}

pub fn execute_default(args: &Cli) -> Result<(), Box<dyn error::Error>> {
    let pm = ProjectManager::new()?;

    let mut day_parts = pm.build_project()?;

    let part = args.part;
    let day = args
        .day
        .unwrap_or_else(|| day_parts.last().expect("No implementation found").day);
    let year = day_parts.year;

    let date = AOCDate {
        day: u32::from(day.0),
        year: year as i32,
    };

    if args.generate {
        update_lib_rs(date.day, &pm)?;
        codegen(date.day, &pm)?;
        println!("Successfully generated boilerplate for {}", date.day);
        // Rebuild to include newly generated day
        day_parts = pm.build_project()?;
    }

    let cargo_content = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/template/Cargo-run.toml.tpl"
    ))
    .replace("{CRATE_NAME}", &pm.name)
    .replace(
        "{PROFILE}",
        if args.profile {
            "[profile.release]\ndebug = true"
        } else {
            ""
        },
    );

    let template = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/template/src/runner.rs.tpl"
    ));

    let mut body = String::new();
    for dp in day_parts.iter().filter(|dp| dp.day == day).filter(|dp| {
        if let Some(p) = part {
            dp.part == p
        } else {
            true
        }
    }) {
        let (name, display) = if let Some(n) = &dp.name {
            (
                format!("day{}_part{}_{}", dp.day.0, dp.part.0, n.to_lowercase()),
                format!("Day {} - Part {} - {}", dp.day.0, dp.part.0, n),
            )
        } else {
            (
                format!("day{}_part{}", dp.day.0, dp.part.0),
                format!("Day {} - Part {}", dp.day.0, dp.part.0),
            )
        };

        body += &template
            .replace("{DAY}", &day.0.to_string())
            .replace("{RUNNER_NAME}", &name)
            .replace("{RUNNER_DISPLAY}", &display);
    }

    if body.is_empty() {
        return Err("No matching day & part found".into());
    }

    download_input(date)?;

    let main_content = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/template/src/main.rs.tpl"
    ))
    .replace("{CRATE_SLUG}", &pm.slug)
    .replace("{YEAR}", &day_parts.year.to_string())
    .replace("{INPUT}", &template_input(day, year, args.input.as_deref()))
    .replace("{BODY}", &body);

    fs::create_dir_all("target/aoc/aoc-autobuild/src")
        .expect("failed to create autobuild directory");
    fs::write("target/aoc/aoc-autobuild/Cargo.toml", cargo_content)
        .expect("failed to write Cargo.toml");
    fs::write("target/aoc/aoc-autobuild/src/main.rs", main_content)
        .expect("failed to write src/main.rs");

    let status = process::Command::new("cargo")
        .args(["run", "--release"])
        .current_dir("target/aoc/aoc-autobuild")
        .spawn()
        .expect("Failed to run cargo")
        .wait()
        .expect("Failed to wait for cargo");

    if !status.success() {
        process::exit(status.code().unwrap_or(-1));
    }
    Ok(())
}

pub fn execute_bench(args: &Bench) -> Result<(), Box<dyn error::Error>> {
    let day: Option<Day> = args.day;
    let part: Option<Part> = args.part;

    let pm = ProjectManager::new()?;

    let day_parts = pm.build_project()?;

    let day = day.unwrap_or_else(|| day_parts.last().expect("No implementation found").day);
    let year = day_parts.year;

    let cargo_content = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/template/Cargo-bench.toml.tpl"
    ))
    .replace("{CRATE_NAME}", &pm.name)
    .replace(
        "{PROFILE}",
        if args.profile {
            "[profile.release]\ndebug = true"
        } else {
            ""
        },
    );

    let bench_tpl = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/template/benches/aoc_benchmark.rs.tpl"
    ));

    let part_tpl = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/template/benches/part.rs.tpl"
    ));

    let gen_tpl = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/template/benches/gen.rs.tpl"
    ));

    let impl_tpl = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/template/benches/impl.rs.tpl"
    ));

    let gen_impl_tpl = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/template/benches/gen_impl.rs.tpl"
    ));

    let matching_parts = day_parts.iter().filter(|dp| dp.day == day).filter(|dp| {
        if let Some(p) = part {
            dp.part == p
        } else {
            true
        }
    });

    let mut parts: Vec<_> = matching_parts.clone().map(|dp| dp.part).collect();
    parts.sort();
    parts.dedup();

    let body: String = parts
        .into_iter()
        .map(|p| {
            let part_name = format!("day{}_part{}", day.0, p.0);
            part_tpl
                .replace("{PART_NAME}", &part_name)
                .replace("{DAY}", &day.0.to_string())
                .replace("{PART}", &p.0.to_string())
                .replace(
                    "{IMPLS}",
                    &matching_parts
                        .clone()
                        .filter(|dp| dp.part == p)
                        .map(|dp| {
                            impl_tpl
                                .replace(
                                    "{RUNNER_NAME}",
                                    &if let Some(n) = &dp.name {
                                        format!(
                                            "day{}_part{}_{}",
                                            dp.day.0,
                                            dp.part.0,
                                            n.to_lowercase()
                                        )
                                    } else {
                                        format!("day{}_part{}", dp.day.0, dp.part.0)
                                    },
                                )
                                .replace("{DAY}", &dp.day.0.to_string())
                                .replace(
                                    "{NAME}",
                                    if let Some(n) = &dp.name {
                                        n
                                    } else {
                                        "(default)"
                                    },
                                )
                                .replace("{PART_NAME}", &part_name)
                        })
                        .collect::<String>(),
                )
        })
        .collect();

    if body.is_empty() {
        return Err("No matching day & part found".into());
    }

    let gens = if args.generator {
        let mut parts: Vec<_> = matching_parts.clone().map(|dp| dp.part).collect();
        parts.sort();
        parts.dedup();

        parts
            .into_iter()
            .map(|p| {
                let gen_name = format!("day{}", day.0);
                gen_tpl
                    .replace("{GEN_NAME}", &gen_name)
                    .replace("{DAY}", &day.0.to_string())
                    .replace("{PART}", &p.0.to_string())
                    .replace(
                        "{IMPLS}",
                        &matching_parts
                            .clone()
                            .filter(|dp| dp.part == p)
                            .map(|dp| {
                                gen_impl_tpl
                                    .replace(
                                        "{RUNNER_NAME}",
                                        &if let Some(n) = &dp.name {
                                            format!(
                                                "day{}_part{}_{}",
                                                dp.day.0,
                                                dp.part.0,
                                                n.to_lowercase()
                                            )
                                        } else {
                                            format!("day{}_part{}", dp.day.0, dp.part.0)
                                        },
                                    )
                                    .replace("{DAY}", &dp.day.0.to_string())
                                    .replace(
                                        "{NAME}",
                                        if let Some(n) = &dp.name {
                                            n
                                        } else {
                                            "(default)"
                                        },
                                    )
                                    .replace("{GEN_NAME}", &gen_name)
                            })
                            .collect::<String>(),
                    )
            })
            .collect()
    } else {
        String::new()
    };

    let date = AOCDate {
        day: u32::from(day.0),
        year: year as i32,
    };
    download_input(date)?;

    let main_content = bench_tpl
        .replace("{CRATE_SLUG}", &pm.slug)
        .replace("{PARTS}", &body)
        .replace("{GENS}", &gens)
        .replace(
            "{BENCHMARKS}",
            if args.generator {
                "aoc_benchmark, input_benchmark"
            } else {
                "aoc_benchmark"
            },
        )
        .replace(
            "{INPUTS}",
            &template_input(day, year, args.input.as_deref()),
        );

    fs::create_dir_all("target/aoc/aoc-autobench/benches")
        .expect("failed to create autobench directory");
    fs::write("target/aoc/aoc-autobench/Cargo.toml", cargo_content)
        .expect("failed to write Cargo.toml");
    fs::write(
        "target/aoc/aoc-autobench/benches/aoc_benchmark.rs",
        main_content,
    )
    .expect("failed to write src/aoc_benchmark.rs");

    let status = process::Command::new("cargo")
        .args(["bench"])
        .current_dir("target/aoc/aoc-autobench")
        .spawn()
        .expect("Failed to run cargo")
        .wait()
        .expect("Failed to wait for cargo");

    if !status.success() {
        process::exit(status.code().unwrap_or(-1));
    }

    if args.open {
        let index = "target/aoc/aoc-autobench/target/criterion/report/index.html";

        if !Path::new(index).exists() {
            return Err("Report is missing, perhaps gnuplot is missing ?".into());
        }
        webbrowser::open(index)?;
    }

    Ok(())
}

fn template_input(day: Day, year: u32, input: Option<&str>) -> String {
    let day = day.0.to_string();
    let path = input
        .map(|p| {
            if Path::new(p).is_relative() {
                format!("../../../../{}", p)
            } else {
                p.to_string()
            }
        })
        .unwrap_or_else(|| format!("../../../../input/{}/day{}.txt", year, &day));
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/template/input.rs.tpl"
    ))
    .replace("{PATH}", &path)
    .replace("{DAY}", &day)
}
