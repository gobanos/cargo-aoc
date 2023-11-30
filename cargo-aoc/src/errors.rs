use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct CouldNotLoadDayParts(pub Box<dyn Error>);

impl Debug for CouldNotLoadDayParts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

impl Display for CouldNotLoadDayParts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Could not load aoc-runner metadata.\n")?;
        f.write_str("Have you setup your project?\n")?;
        f.write_str("Getting started: https://github.com/gobanos/cargo-aoc/tree/v0.3/aoc-runner#getting-started\n")?;
        f.write_fmt(format_args!("Error: {}", self.0))
    }
}

impl Error for CouldNotLoadDayParts {

}
