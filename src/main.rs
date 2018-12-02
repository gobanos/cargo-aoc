/// TODO: refactor this. As of Rust 2018 Edition, extern crate is no longer required.
extern crate aoc_runner_internal;
extern crate chrono;
extern crate chrono_tz;
extern crate clap;
/// TODO: Do we actually need to rely on reqwest ?
/// ... Tokio is overkill for the scope of this project.
extern crate reqwest;
extern crate toml;
extern crate webbrowser;

mod app;
mod credentials;
mod date;
mod project;

use clap::{App, Arg, SubCommand};

use app::AOCApp;

fn main() {
    // Parses the attributes (CLAP)
    let matches = App::new("cargo-aoc")
        .version("1.0")
        .about("Cargo helper for Advent of Code")
        .author("gobanos <gregory.obanos@gmail.com>")
        .arg(Arg::with_name("dummy").hidden(true).possible_value("aoc"))
        .arg(
            Arg::with_name("day")
                .short("d")
                .help("Specifies the day. Defaults to last implemented.")
                .takes_value(true),
        ).arg(
            Arg::with_name("part")
                .short("p")
                .help("Specifies the part. Defaults to both parts.")
                .takes_value(true),
        ).subcommand(
            SubCommand::with_name("bench")
                .about("Benchmark your solutions")
                .arg(
                    Arg::with_name("day")
                        .short("d")
                        .help("Specifies the day. Defaults to last implemented.")
                        .takes_value(true),
                ).arg(
                    Arg::with_name("part")
                        .short("p")
                        .help("Specifies the part. Defaults to both parts.")
                        .takes_value(true),
                ).arg(
                    Arg::with_name("open")
                        .short("o")
                        .help("Opens the benchmark information in the browser"),
                ),
        ).subcommand(
            SubCommand::with_name("credentials")
                .about("Manage your AOC credentials information")
                .arg(
                    Arg::with_name("set")
                        .short("s")
                        .help("Sets the session cookie")
                        .takes_value(true),
                ),
        ).subcommand(
            SubCommand::with_name("input")
                .about("Get the input for a specified date")
                .arg(
                    Arg::with_name("day")
                        .short("d")
                        .help("Specifies the day. Defaults to today's date.")
                        .takes_value(true),
                ).arg(
                    Arg::with_name("year")
                        .short("y")
                        .help("Specifies the year. Defaults to the current year.")
                        .takes_value(true),
                ),
        ).get_matches();

    // Creates an AOCApp that we'll use to launch actions (commands)
    let app = AOCApp::new();

    match matches.subcommand() {
        ("credentials", Some(m)) => app.execute_credentials(&m),
        ("input", Some(m)) => app.execute_input(&m),
        ("bench", Some(m)) => if let Err(e) = app.execute_bench(&m) {
            eprintln!("An error occurs : {}", e.description());
            std::process::exit(-1);
        },
        (c, Some(_)) => panic!("Unknown command `{}`", c),
        _ => if let Err(e) = app.execute_default(&matches) {
            eprintln!("An error occurs : {}", e.description());
            std::process::exit(-1);
        },
    }
}
