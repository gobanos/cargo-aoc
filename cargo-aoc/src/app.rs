use aoc_runner_internal::Day;
use aoc_runner_internal::Part;
use clap::ArgMatches;
use credentials::CredentialsManager;
use date::AOCDate;
use project::ProjectManager;
use reqwest::header::{COOKIE, USER_AGENT};
use reqwest::blocking::Client;
use reqwest::StatusCode;
use std::error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process;

const CARGO_AOC_USER_AGENT: &str = "github.com/gobanos/cargo-aoc by gregory.obanos@gmail.com";

pub struct AOCApp {}

impl AOCApp {
    /// Creates a new instance of the application
    pub fn new() -> Self {
        AOCApp {}
    }

    /// Executes the "credientals" subcommand of the app
    pub fn execute_credentials(&self, sub_args: &ArgMatches) {
        let mut creds_manager = CredentialsManager::new();

        if let Some(new_session) = sub_args.value_of("set") {
            // Tries to set the session token
            match creds_manager.set_session_token(new_session.into()) {
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
    pub fn execute_input(&self, sub_args: &ArgMatches) {
        // Gets the token or exit if it's not referenced.
        let token = CredentialsManager::new().get_session_token().expect(
            "Error: you need to setup your AOC token using \"cargo aoc credentials -s {token}\"",
        );

        // Creates the AOCDate struct from the arguments (defaults to today...)
        let date: AOCDate = AOCDate::new(sub_args);
        println!(
            "Requesting input for year {}, day {} ...",
            date.year, date.day
        );

        // Creates an HTTP Client
        let client = Client::new();
        // Cookie formatting ...
        let formated_token = format!("session={}", token);
        // Sends the query to the right URL, with the user token
        let res = client
            .get(&date.request_url())
            .header(USER_AGENT, CARGO_AOC_USER_AGENT)
            .header(COOKIE, formated_token)
            .send();

        // Depending on the StatusCode of the request, we'll write errors or try to write
        // the result of the HTTP Request to a file
        match res {
            Ok(response) => match response.status() {
                StatusCode::OK => {
                    let filename = date.filename();
                    let dir = date.directory();
                    // Creates the file-tree to store inputs
                    // TODO: Maybe use crate's infos to get its root in the filesystem ? 
                    fs::create_dir_all(&dir).unwrap_or_else(|_| panic!("Could not create input directory: {}", dir));

                    // Gets the body from the response and outputs everything to a file
                    let body = response.text().expect("Could not read content from input");
                    let mut file = File::create(&filename).unwrap_or_else(|_| panic!("Could not create file {}", filename));
                    file.write_all(body.as_bytes()).unwrap_or_else(|_| panic!("Could not write to {}", filename));
                }
                sc => println!(
                    "Could not find corresponding input. Are the day, year, and token correctly set ? Status: {}\
                    Message: {}", sc, response.text().unwrap_or_else(|_| String::new())
                ),
            },
            Err(e) => println!("Failed to get a response: {}", e),
        }
    }

    fn download_input(&self, day: Day, year: u32) -> Result<(), Box<dyn error::Error>> {
        let date = AOCDate {
            day: u32::from(day.0),
            year: year as i32,
        };

        let filename = date.filename();
        let filename = Path::new(&filename);

        if filename.exists() {
            return Ok(());
        }

        let token = CredentialsManager::new().get_session_token()?;
        // Creates an HTTP Client
        let client = Client::new();
        // Cookie formatting ...
        let formated_token = format!("session={}", token);

        let response = client
            .get(&date.request_url())
            .header(USER_AGENT, CARGO_AOC_USER_AGENT)
            .header(COOKIE, formated_token)
            .send()?;

        match response.status() {
            StatusCode::OK => {
                let dir = date.directory();
                // Creates the file-tree to store inputs
                // TODO: Maybe use crate's infos to get its root in the filesystem ?
                fs::create_dir_all(&dir)?;

                // Gets the body from the response and outputs everything to a file
                let body = response.text()?;
                let mut file = File::create(&filename)?;
                file.write_all(body.as_bytes())?;
            }
            sc => return Err(format!(
                "Could not find corresponding input. Are the day, year, and token correctly set ? Status: {}\
                 Message: {}", sc, response.text().unwrap_or_else(|_| String::new())
            ).into()),
        }

        Ok(())
    }

    pub fn execute_default(&self, args: &ArgMatches) -> Result<(), Box<dyn error::Error>> {
        let day: Option<Day> = args
            .value_of("day")
            .map(|d| d.parse().expect("Failed to parse day"));

        let part: Option<Part> = args
            .value_of("part")
            .map(|p| p.parse().expect("Failed to parse part"));

        let pm = ProjectManager::new()?;

        let day_parts = pm.build_project()?;

        let day = day.unwrap_or_else(|| day_parts.last().expect("No implementation found").day);
        let year = day_parts.year;

        let cargo_content = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/template/Cargo-run.toml.tpl"
        ))
        .replace("{CRATE_NAME}", &pm.name)
        .replace(
            "{PROFILE}",
            if args.is_present("profile") {
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

        self.download_input(day, year)?;

        let main_content = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/template/src/main.rs.tpl"
        ))
        .replace("{CRATE_SLUG}", &pm.slug)
        .replace("{YEAR}", &day_parts.year.to_string())
        .replace(
            "{INPUT}",
            &template_input(day, year, args.value_of("input")),
        )
        .replace("{BODY}", &body);

        fs::create_dir_all("target/aoc/aoc-autobuild/src")
            .expect("failed to create autobuild directory");
        fs::write("target/aoc/aoc-autobuild/Cargo.toml", &cargo_content)
            .expect("failed to write Cargo.toml");
        fs::write("target/aoc/aoc-autobuild/src/main.rs", &main_content)
            .expect("failed to write src/main.rs");

        let status = process::Command::new("cargo")
            .args(&["run", "--release"])
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

    pub fn execute_bench(&self, args: &ArgMatches) -> Result<(), Box<dyn error::Error>> {
        let day: Option<Day> = args
            .value_of("day")
            .map(|d| d.parse().expect("Failed to parse day"));

        let part: Option<Part> = args
            .value_of("part")
            .map(|p| p.parse().expect("Failed to parse part"));

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
            if args.is_present("profile") {
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
                                            &n
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

        let gens = if args.is_present("generator") {
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
                                                &n
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

        self.download_input(day, year)?;

        let main_content = bench_tpl
            .replace("{CRATE_SLUG}", &pm.slug)
            .replace("{PARTS}", &body)
            .replace("{GENS}", &gens)
            .replace(
                "{BENCHMARKS}",
                if args.is_present("generator") {
                    "aoc_benchmark, input_benchmark"
                } else {
                    "aoc_benchmark"
                },
            )
            .replace(
                "{INPUTS}",
                &template_input(day, year, args.value_of("input")),
            );

        fs::create_dir_all("target/aoc/aoc-autobench/benches")
            .expect("failed to create autobench directory");
        fs::write("target/aoc/aoc-autobench/Cargo.toml", &cargo_content)
            .expect("failed to write Cargo.toml");
        fs::write(
            "target/aoc/aoc-autobench/benches/aoc_benchmark.rs",
            &main_content,
        )
        .expect("failed to write src/aoc_benchmark.rs");

        let status = process::Command::new("cargo")
            .args(&["bench"])
            .current_dir("target/aoc/aoc-autobench")
            .spawn()
            .expect("Failed to run cargo")
            .wait()
            .expect("Failed to wait for cargo");

        if !status.success() {
            process::exit(status.code().unwrap_or(-1));
        }

        if args.is_present("open") {
            let index = "target/aoc/aoc-autobench/target/criterion/report/index.html";

            if !Path::new(index).exists() {
                return Err("Report is missing, perhaps gnuplot is missing ?".into());
            }
            webbrowser::open(index)?;
        }

        Ok(())
    }
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
