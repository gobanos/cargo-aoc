extern crate dotenv;
extern crate aoc_runner_internal;
extern crate toml;

use std::env;
use std::process;
use std::fs;
use aoc_runner_internal::DayParts;

fn main() {
    let _ = dotenv::dotenv();

    let session = env::var("AOC_SESSION").expect("no session found");

    println!("SESSION: {}", session);

    let args = vec!["build", "--release", "--color=always"];

    let status = process::Command::new("cargo")
        .args(&args)
        .spawn()
        .expect("Failed to run cargo")
        .wait()
        .expect("Failed to wait for cargo");

    if !status.success() {
        process::exit(status.code().unwrap_or(-1));
    }

    let day_parts = DayParts::load().expect("Failed to load AOC infos");

    println!("{:#?}", day_parts);

    let cargo : toml::Value = fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml").parse().expect("Invalid toml value");

    let crate_name = cargo.get("package").unwrap().get("name").unwrap().as_str().unwrap();
    let crate_slug = crate_name.replace('-', "_");

    let cargo_content = include_str!("../template/Cargo.toml").replace("{CRATE_NAME}", &crate_name);
    let main_content = include_str!("../template/src/main.rs").replace("{CRATE_SLUG}", &crate_slug);

    fs::create_dir_all("target/aoc/aoc-autobuild/src").expect("failed to create autobuild directory");
    fs::write("target/aoc/aoc-autobuild/Cargo.toml", &cargo_content).expect("failed to write Cargo.toml");
    fs::write("target/aoc/aoc-autobuild/src/main.rs", &main_content).expect("failed to write src/main.rs");

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
}
