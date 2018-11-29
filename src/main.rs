/// TODO: refactor this. As of Rust 2018 Edition, extern crate is no longer required.
extern crate aoc_runner_internal;
extern crate chrono;
extern crate clap;
/// TODO: Do we actually need to rely on reqwest ?
/// ... Tokio is overkill for the scope of this project.
extern crate reqwest;
extern crate toml;

mod app;
mod credentials;
mod date;
mod project;

use clap::{App, Arg, SubCommand};

use app::AOCApp;

fn main() {
    // Parses
    let matches = App::new("cargo-aoc")
        .version("1.0")
        .about("Cargo helper for Advent of Code")
        .author("gobanos <gregory.obanos@gmail.com>")
        .arg(Arg::with_name("dummy").hidden(true).possible_value("aoc"))
        .subcommand(
            SubCommand::with_name("credentials")
                .about("Manage your AOC credentials information")
                .arg(
                    Arg::with_name("set")
                        .short("s")
                        .help("Sets the session cookie")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("input")
                .about("Get the input for a specified date")
                .arg(
                    Arg::with_name("day")
                        .short("d")
                        .help("Specifies the day. Defaults to today's date.")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("year")
                        .short("y")
                        .help("Specifies the year. Defaults to the current year.")
                        .takes_value(true),
                ),
        )
        .get_matches();

    // Creates an AOCApp that we'll use to launch actions (commands)
    let app = AOCApp::new();

    match matches.subcommand() {
        ("credentials", Some(m)) => app.execute_credentials(&m),
        ("input", Some(m)) => app.execute_input(&m),
        (_, Some(_)) => panic!("Unknown command"),
        _ => app.execute_default().unwrap(),
    }
}
