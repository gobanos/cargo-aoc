use aoc_runner_derive::{aoc, aoc_generator};
use chrono::NaiveDateTime;
use chrono::ParseError;
use chrono::Timelike;
use std::cmp;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt::{self, Display};
use std::str::FromStr;
use std::time::Duration;

#[derive(Debug, Eq, PartialEq)]
pub struct Record {
    date: NaiveDateTime,
    action: Action,
}

impl Ord for Record {
    fn cmp(&self, other: &Record) -> cmp::Ordering {
        self.date.cmp(&other.date)
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Record) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Action {
    BeginShift(u32),
    FallAsleep,
    WakeUp,
}

#[derive(Debug)]
enum Error {
    ParseDateError(ParseError),
    ParseActionError(&'static str),
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::ParseDateError(e)
    }
}

impl From<&'static str> for Error {
    fn from(e: &'static str) -> Self {
        Error::ParseActionError(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "{:?}", self)
    }
}
impl StdError for Error {}

impl FromStr for Action {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Action, &'static str> {
        match s {
            "falls asleep" => Ok(Action::FallAsleep),
            "wakes up" => Ok(Action::WakeUp),
            _ if s.starts_with("Guard #") && s.ends_with(" begins shift") => {
                Ok(Action::BeginShift(
                    s[7..s.len() - 13]
                        .parse()
                        .map_err(|_| "failed to parse guard id")?,
                ))
            }
            _ => Err("failed to parse action"),
        }
    }
}

#[aoc_generator(day = "4")]
fn parse(input: &str) -> Result<Vec<Record>, Error> {
    let mut records = input
        .lines()
        .map(|l| {
            let date = NaiveDateTime::parse_from_str(&l[1..17], "%Y-%m-%d %H:%M")?;
            let action = l[19..].parse()?;

            Ok(Record { date, action })
        })
        .collect::<Result<Vec<_>, Error>>()?;

    records.sort();

    Ok(records)
}

type GuardId = u32;
type GuardRecord = (Duration, Vec<(NaiveDateTime, NaiveDateTime)>);

fn build_map(records: &[Record]) -> Result<HashMap<GuardId, GuardRecord>, &'static str> {
    let mut guard = None;
    let mut start_sleeping = None;

    let mut map: HashMap<u32, (Duration, Vec<(NaiveDateTime, NaiveDateTime)>)> = HashMap::new();

    for record in records {
        match record.action {
            Action::BeginShift(g) => guard = Some(g),
            Action::FallAsleep => start_sleeping = Some(record.date),
            Action::WakeUp => {
                let start = start_sleeping
                    .take()
                    .ok_or("wake up before start sleeping")?;
                let sleep_duration = (record.date - start)
                    .to_std()
                    .map_err(|_| "failed to convert chrono duration to std duration")?;
                let entry = map
                    .entry(guard.ok_or("wake up with no guard")?)
                    .or_default();
                entry.0 += sleep_duration;
                entry.1.push((start, record.date));
            }
        }
    }

    Ok(map)
}

#[aoc(day = "4", part = "1")]
fn part1(records: &[Record]) -> Result<u32, &'static str> {
    let map = build_map(records)?;

    let (guard, (_, sessions)) = map
        .into_iter()
        .max_by_key(|(_, d)| d.0)
        .ok_or("maximum sleeping session not found")?;

    let (min, _) = (0..60)
        .map(|m| {
            (
                m,
                sessions
                    .iter()
                    .filter(|&(start, stop)| m >= start.minute() && m < stop.minute())
                    .count(),
            )
        })
        .max_by_key(|&(_, t)| t)
        .ok_or("maximum sleeping minute not found")?;

    Ok(guard * min)
}

#[aoc(day = "4", part = "2")]
fn part2(records: &[Record]) -> Result<u32, &'static str> {
    let map = build_map(records)?;

    let (guard, min, _) = map
        .into_iter()
        .map(|(guard, (_, sessions))| {
            let (min, count) = (0..60)
                .map(|m| {
                    (
                        m,
                        sessions
                            .iter()
                            .filter(|&(start, stop)| m >= start.minute() && m < stop.minute())
                            .count(),
                    )
                })
                .max_by_key(|&(_, t)| t)
                .ok_or("maximum sleeping session not found")?;

            Ok((guard, min, count))
        })
        .fold(None, |acc, res| match (acc, res) {
            (None, _) | (Some(Ok(_)), Err(_)) => Some(res),
            (Some(Err(_)), _) => acc,
            (Some(Ok((_, _, a))), Ok((_, _, b))) if b > a => Some(res),
            _ => acc,
        })
        .ok_or("maximum sleeping min not found")??;

    Ok(guard * min)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    #[test]
    fn parse_example() {
        assert!(parse(INPUT).is_ok());
    }
}
