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

use clap::{App, Arg, SubCommand};

use app::AOCApp;

fn main() {
    // Parses
    let matches = App::new("aoc")
        .version("1.0")
        .about("Cargo helper for Advent of Code")
        .author("gobanos <gregory.obanos@gmail.com>")
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

    // If subcommand "credentials" is called
    if let Some(args) = matches.subcommand_matches("credentials") {
        app.execute_credentials(args);
        return;
    }

    // If subcommand "input" is called
    if let Some(args) = matches.subcommand_matches("input") {
        app.execute_input(args);
        return;
    }

    // Else, let's use the default command and let the runner do the rest
    app.execute_default();
}
