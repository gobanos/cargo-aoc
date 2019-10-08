use chrono::prelude::*;
use chrono_tz::EST;
use clap::ArgMatches;

pub struct AOCDate {
    /// The day of the input to retrieve
    pub day: u32,
    /// The year of the input to retrieve
    pub year: i32,
}

impl AOCDate {
    /// Creates a date struct for the given `ArgMatches`, defaulting to today's day and year
    pub fn new(matches: &ArgMatches) -> Self {
        // Get the current date in the EST timezone, which is used by advent of code to
        // release new puzzles.
        let utc_today = Utc::now().naive_utc();
        let today = EST.from_utc_datetime(&utc_today);
        let day: u32 = matches
            .value_of("day")
            .map(|d| d.parse::<u32>().expect("Day not formatted correctly"))
            .unwrap_or(today.day());

        let year: i32 = matches
            .value_of("year")
            .map(|d| d.parse::<i32>().expect("Year not formatted correctly"))
            .unwrap_or(today.year());

        AOCDate { day, year }
    }

    pub fn directory(&self) -> String {
        format!("input/{}", self.year)
    }

    pub fn filename(&self) -> String {
        format!("input/{}/day{}.txt", self.year, self.day)
    }

    /// Consumes the date to get an URL
    pub fn request_url(&self) -> String {
        format!(
            "https://adventofcode.com/{}/day/{}/input",
            self.year, self.day
        )
    }
}
