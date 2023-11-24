mod app;
mod credentials;
mod date;
mod project;

use aoc_runner_internal::{Day, Part};
use app::{execute_default, execute_bench, execute_credentials, execute_input};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "cargo-aoc", version = "0.3.0", author = "gobanos <gregory.obanos@gmail.com>", about = "Cargo helper for Advent of Code")]
pub struct Cli {
    #[clap(help = "Specifies the day. Defaults to last implemented.")]
    day: Option<Day>,

    #[clap(short, long, help = "Specifies the part. Defaults to both parts.")]
    part: Option<Part>,

    #[clap(short, long, help = "Use an alternate input file.")]
    input: Option<String>,

    #[clap(short, long, help = "Add debug info for profiling tools.")]
    profile: bool,

    #[clap(subcommand)]
    subcmd: Option<SubCommands>,
}

#[derive(Parser, Debug)]
enum SubCommands {
    Bench(Bench),
    Credentials(Credentials),
    Input(Input),
}

#[derive(Parser, Debug)]
pub struct Bench {
    #[clap(short, long, help = "Specifies the day. Defaults to last implemented.")]
    day: Option<Day>,

    #[clap(short, long, help = "Specifies the part. Defaults to both parts.")]
    part: Option<Part>,

    #[clap(short, long, help = "Use an alternate input file.")]
    input: Option<String>,

    #[clap(short, long, help = "Opens the benchmark information in the browser")]
    open: bool,

    #[clap(short, long, help = "Also benchmark generator functions.")]
    generator: bool,

    #[clap(short, long, help = "Add debug info for profiling tools.")]
    profile: bool,
}

#[derive(Parser, Debug)]
pub struct Credentials {
    #[clap(short, long, help = "Sets the session cookie")]
    set: Option<String>,
}

#[derive(Parser, Debug)]
pub struct Input {
    #[clap(short, long, help = "Specifies the day. Defaults to today's date.")]
    day: Option<u32>,

    #[clap(short, long, help = "Specifies the year. Defaults to the current year.")]
    year: Option<i32>,
}

fn main() {
    let cli = Cli::parse();
    
    let Some(subcommand) = cli.subcmd else {
        return execute_default(&cli).unwrap();
    };

    let err = match subcommand {
        SubCommands::Bench(arg) => execute_bench(&arg),
        SubCommands::Credentials(arg) => Ok(execute_credentials(&arg)),
        SubCommands::Input(arg) => Ok(execute_input(&arg)),
    };
    err.unwrap()
}
