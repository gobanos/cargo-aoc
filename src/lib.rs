extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use std::str::FromStr;
use serde_derive::*;
use std::error;
use std::fs;
use std::ops::Deref;
use std::iter::FromIterator;
use std::cmp::Ordering;
use std::ops::DerefMut;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Day(pub u8);

impl FromStr for Day {
    type Err = String;

    fn from_str(day: &str) -> Result<Self, Self::Err> {
        if day.len() < 4 || &day[..3] != "day" {
            return Err(format!("Failed to parse day: {}", day));
        }

        day[3..]
            .parse()
            .map_err(|e| format!("Failed to parse {}: {:?}", day, e))
            .and_then(|d| {
                if d == 0 || d > 25 {
                    Err(format!("day {} is not between 0 and 25", d))
                } else {
                    Ok(Day(d))
                }
            })
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Part(pub u8);

impl FromStr for Part {
    type Err = String;

    fn from_str(part: &str) -> Result<Self, Self::Err> {
        Ok(match part {
            "part1" => Part(1),
            "part2" => Part(2),
            _ => return Err(format!("Failed to parse part: {}", part)),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
pub struct DayPart {
    pub day: Day,
    pub part: Part,
}

impl PartialOrd for DayPart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for DayPart {
    fn cmp(&self, other: &Self) -> Ordering {
        self.day.cmp(&other.day).then(self.part.cmp(&other.part))
    }
}

#[derive(Debug)]
pub struct DayParts {
    inner: Vec<DayPart>,
}

impl DayParts {
    pub fn save(&self) -> Result<(), Box<error::Error>> {
        fs::create_dir_all("target/aoc")?;
        let f = fs::File::create("target/aoc/completed.json")?;

        serde_json::to_writer_pretty(f, &self.inner)?;

        Ok(())
    }

    pub fn load() -> Result<Self, Box<error::Error>> {
        let f = fs::File::open("target/aoc/completed.json")?;

        let inner: Vec<DayPart> = serde_json::from_reader(f)?;

        Ok(DayParts { inner })
    }
}

impl Deref for DayParts {
    type Target = [DayPart];

    fn deref(&self) -> &[DayPart] {
        &self.inner
    }
}

impl DerefMut for DayParts {
    fn deref_mut(&mut self) -> &mut [DayPart] {
        &mut self.inner
    }
}

impl FromIterator<DayPart> for DayParts {
    fn from_iter<T: IntoIterator<Item=DayPart>>(iter: T) -> Self {
        let inner = iter.into_iter().collect();
        DayParts { inner }
    }
}