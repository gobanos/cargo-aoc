use aoc_runner_internal::Day;
use aoc_runner_internal::Part;
use clap::ArgMatches;
use credentials::CredentialsManager;
use date::AOCDate;
use project::ProjectManager;
use reqwest::header::COOKIE;
use reqwest::Client;
use reqwest::StatusCode;
use std::error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process;

pub struct AOCApp {}

impl AOCApp {
    /// Creates a new instance of the application
    pub fn new() -> Self {
        AOCApp {}
    }

    /// Executes the "credientals" subcommand of the app
    pub fn execute_credentials(&self, sub_args: &ArgMatches) {
        let mut creds_manager = CredentialsManager::new();

        match sub_args.value_of("set") {
            Some(new_session) => {
                // Tries to set the session token
                match creds_manager.set_session_token(new_session.into()) {
                    Ok(()) => println!("Credentials sucessfully changed!"),
                    Err(e) => println!("Error changing credentials: {}", e),
                }
            }
            _ => {}
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
            .header(COOKIE, formated_token)
            .send();

        // Depending on the StatusCode of the request, we'll write errors or try to write
        // the result of the HTTP Request to a file
        match res {
            Ok(mut response) => match response.status() {
                StatusCode::OK => {
                    let filename = date.filename();
                    let dir = date.directory();
                    // Creates the file-tree to store inputs
                    // TODO: Maybe use crate's infos to get its root in the filesystem ? 
                    fs::create_dir("input").expect("Could not create input directory");
                    fs::create_dir(&dir).expect(&format!("Could not create input directory: {}", dir));

                    // Gets the body from the response and outputs everything to a file
                    let body = response.text().expect("Could not read content from input");
                    let mut file = File::create(&filename).expect(&format!("Could not create file {}", filename));
                    file.write(body.as_bytes()).expect(&format!("Could not write to {}", filename));
                }
                sc => println!(
                    "Could not find corresponding input. Are the day, year, and token correctly set ? Status: {}", sc
                ),
            },
            Err(e) => println!("Failed to get a response: {}", e),
        }
    }

    fn download_input(&self, day: Day, year: u32) -> Result<(), Box<error::Error>> {
        let date = AOCDate {
            day: day.0 as u32,
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

        let mut response = client
            .get(&date.request_url())
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
                file.write(body.as_bytes())?;
            }
            sc => Err(format!(
                "Could not find corresponding input. Are the day, year, and token correctly set ? Status: {}\
                Message: {}", sc, response.text().unwrap_or_else(|_| String::new())
            ))?,
        }

        Ok(())
    }

    pub fn execute_default(&self, args: &ArgMatches) -> Result<(), Box<error::Error>> {
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

        let cargo_content =
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/template/Cargo.toml.tpl")).replace("{CRATE_NAME}", &pm.name);
        let template = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/template/src/runner.rs.tpl"));

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

            let input = format!("{}/day{}.txt", year, dp.day.0);

            body += &template
                .replace("{RUNNER_NAME}", &name)
                .replace("{INPUT}", &input)
                .replace("{RUNNER_DISPLAY}", &display);
        }

        if body.is_empty() {
            Err("No matching day & part found")?;
        }

        self.download_input(day, year)?;

        let main_content = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/template/src/main.rs.tpl"))
            .replace("{CRATE_SLUG}", &pm.slug)
            .replace("{YEAR}", &day_parts.year.to_string())
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
}
