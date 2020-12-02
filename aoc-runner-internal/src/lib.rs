use serde::export::fmt::Error;
use serde::export::Formatter;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::Display;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum Day {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

impl Day {
    pub fn as_u8(self) -> u8 {
        <Self as Into<u8>>::into(self)
    }
}

impl FromStr for Day {
    type Err = &'static str;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        Ok(match val {
            "Day01" | "day01" | "Day1" | "day1" | "D1" | "d1" => Day::Day1,
            "Day02" | "day02" | "Day2" | "day2" | "D2" | "d2" => Day::Day2,
            "Day03" | "day03" | "Day3" | "day3" | "D3" | "d3" => Day::Day3,
            "Day04" | "day04" | "Day4" | "day4" | "D4" | "d4" => Day::Day4,
            "Day05" | "day05" | "Day5" | "day5" | "D5" | "d5" => Day::Day5,
            "Day06" | "day06" | "Day6" | "day6" | "D6" | "d6" => Day::Day6,
            "Day07" | "day07" | "Day7" | "day7" | "D7" | "d7" => Day::Day7,
            "Day08" | "day08" | "Day8" | "day8" | "D8" | "d8" => Day::Day8,
            "Day09" | "day09" | "Day9" | "day9" | "D9" | "d9" => Day::Day9,
            "Day10" | "day10" | "D10" | "d10" => Day::Day10,
            "Day11" | "day11" | "D11" | "d11" => Day::Day11,
            "Day12" | "day12" | "D12" | "d12" => Day::Day12,
            "Day13" | "day13" | "D13" | "d13" => Day::Day13,
            "Day14" | "day14" | "D14" | "d14" => Day::Day14,
            "Day15" | "day15" | "D15" | "d15" => Day::Day15,
            "Day16" | "day16" | "D16" | "d16" => Day::Day16,
            "Day17" | "day17" | "D17" | "d17" => Day::Day17,
            "Day18" | "day18" | "D18" | "d18" => Day::Day18,
            "Day19" | "day19" | "D19" | "d19" => Day::Day19,
            "Day20" | "day20" | "D20" | "d20" => Day::Day20,
            "Day21" | "day21" | "D21" | "d21" => Day::Day21,
            "Day22" | "day22" | "D22" | "d22" => Day::Day22,
            "Day23" | "day23" | "D23" | "d23" => Day::Day23,
            "Day24" | "day24" | "D24" | "d24" => Day::Day24,
            "Day25" | "day25" | "D25" | "d25" => Day::Day25,
            _ => {
                return Err(
                    "Failed to parse day, allowed patterns: DayX, dayX, DX, dX with X in 1..=25",
                )
            }
        })
    }
}

impl TryFrom<u8> for Day {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        Ok(match val {
            1 => Day::Day1,
            2 => Day::Day2,
            3 => Day::Day3,
            4 => Day::Day4,
            5 => Day::Day5,
            6 => Day::Day6,
            7 => Day::Day7,
            8 => Day::Day8,
            9 => Day::Day9,
            10 => Day::Day10,
            11 => Day::Day11,
            12 => Day::Day12,
            13 => Day::Day13,
            14 => Day::Day14,
            15 => Day::Day15,
            16 => Day::Day16,
            17 => Day::Day17,
            18 => Day::Day18,
            19 => Day::Day19,
            20 => Day::Day20,
            21 => Day::Day21,
            22 => Day::Day22,
            23 => Day::Day23,
            24 => Day::Day24,
            25 => Day::Day25,
            _ => return Err("Day must be in range 1..=25"),
        })
    }
}

impl Into<u8> for Day {
    fn into(self) -> u8 {
        match self {
            Day::Day1 => 1,
            Day::Day2 => 2,
            Day::Day3 => 3,
            Day::Day4 => 4,
            Day::Day5 => 5,
            Day::Day6 => 6,
            Day::Day7 => 7,
            Day::Day8 => 8,
            Day::Day9 => 9,
            Day::Day10 => 10,
            Day::Day11 => 11,
            Day::Day12 => 12,
            Day::Day13 => 13,
            Day::Day14 => 14,
            Day::Day15 => 15,
            Day::Day16 => 16,
            Day::Day17 => 17,
            Day::Day18 => 18,
            Day::Day19 => 19,
            Day::Day20 => 20,
            Day::Day21 => 21,
            Day::Day22 => 22,
            Day::Day23 => 23,
            Day::Day24 => 24,
            Day::Day25 => 25,
        }
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "day{}", self.as_u8())
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum Part {
    Part1,
    Part2,
}

impl Part {
    pub fn as_u8(self) -> u8 {
        <Self as Into<u8>>::into(self)
    }
}

impl FromStr for Part {
    type Err = &'static str;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        Ok(match val {
            "Part1" | "part1" | "P1" | "p1" => Part::Part1,
            "Part2" | "part2" | "P2" | "p2" => Part::Part2,
            _ => {
                return Err(
                    "Failed to parse part, allowed patterns: PartX, partX, PX, pX with X in 1..=2",
                )
            }
        })
    }
}

impl TryFrom<u8> for Part {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        Ok(match val {
            1 => Part::Part1,
            2 => Part::Part2,
            _ => return Err("Part must be in range 1..=2"),
        })
    }
}

impl Into<u8> for Part {
    fn into(self) -> u8 {
        match self {
            Part::Part1 => 1,
            Part::Part2 => 2,
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "part{}", self.as_u8())
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum Alternative {
    Default,
    Alt1,
    Alt2,
    Alt3,
    Alt4,
    //    Alt5,
    //    Alt6,
    //    Alt7,
    //    Alt8,
}

impl Alternative {
    pub fn as_u8(self) -> u8 {
        <Self as Into<u8>>::into(self)
    }
}

impl FromStr for Alternative {
    type Err = &'static str;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        Ok(match val {
            "Default" | "default" | "Alternative0" | "alternative0" | "Alt0" | "alt0" | "A0" | "a0" => Alternative::Default,
            "Alternative1" | "alternative1" | "Alt1" | "alt1" | "A1" | "a1" => Alternative::Alt1,
            "Alternative2" | "alternative2" | "Alt2" | "alt2" | "A2" | "a2" => Alternative::Alt2,
            "Alternative3" | "alternative3" | "Alt3" | "alt3" | "A3" | "a3" => Alternative::Alt3,
            "Alternative4" | "alternative4" | "Alt4" | "alt4" | "A4" | "a4" => Alternative::Alt4,
//            "Alternative5" | "alternative5" | "Alt5" | "alt5" | "A5" | "a5" => Alternative::Alt5,
//            "Alternative6" | "alternative6" | "Alt6" | "alt6" | "A6" | "a6" => Alternative::Alt6,
//            "Alternative7" | "alternative7" | "Alt7" | "alt7" | "A7" | "a7" => Alternative::Alt7,
//            "Alternative8" | "alternative8" | "Alt8" | "alt8" | "A8" | "a8" => Alternative::Alt8,
            _ => return Err("Failed to parse alternative, allowed patterns: Default, default, AlternativeX, alternativeX, AltX, altX, AX, aX with X in 1..=8"),
        })
    }
}

impl TryFrom<u8> for Alternative {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        Ok(match val {
            0 => Alternative::Default,
            1 => Alternative::Alt1,
            2 => Alternative::Alt2,
            3 => Alternative::Alt3,
            4 => Alternative::Alt4,
            //            5 => Alternative::Alt5,
            //            6 => Alternative::Alt6,
            //            7 => Alternative::Alt7,
            //            8 => Alternative::Alt8,
            _ => return Err("Alternative must be in range 0..=8"),
        })
    }
}

impl Into<u8> for Alternative {
    fn into(self) -> u8 {
        match self {
            Alternative::Default => 0,
            Alternative::Alt1 => 1,
            Alternative::Alt2 => 2,
            Alternative::Alt3 => 3,
            Alternative::Alt4 => 4,
            //            Alternative::Alt5 => 5,
            //            Alternative::Alt6 => 6,
            //            Alternative::Alt7 => 7,
            //            Alternative::Alt8 => 8,
        }
    }
}

impl Display for Alternative {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Alternative::Default => write!(f, "default"),
            _ => write!(f, "alt{}", self.as_u8()),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct DayPart<'a> {
    pub day: Day,
    pub part: Part,
    pub alt: Alternative,
    pub name: Option<&'a str>,
}

impl PartialOrd for DayPart<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for DayPart<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.day
            .cmp(&other.day)
            .then(self.part.cmp(&other.part))
            .then(self.alt.cmp(&other.alt))
    }
}

#[derive(Debug)]
pub struct DayParts<'a> {
    pub year: u32,
    parts: Vec<DayPart<'a>>,
}

impl<'a> Deref for DayParts<'a> {
    type Target = [DayPart<'a>];

    fn deref(&self) -> &Self::Target {
        &self.parts
    }
}

impl<'a> DerefMut for DayParts<'a> {
    fn deref_mut(&mut self) -> &mut [DayPart<'a>] {
        &mut self.parts
    }
}

pub struct DayPartsBuilder<'a> {
    parts: Vec<DayPart<'a>>,
}

impl<'a> DayPartsBuilder<'a> {
    pub fn with_year(self, year: u32) -> DayParts<'a> {
        DayParts {
            year,
            parts: self.parts,
        }
    }
}

impl<'a> FromIterator<DayPart<'a>> for DayPartsBuilder<'a> {
    fn from_iter<T: IntoIterator<Item = DayPart<'a>>>(iter: T) -> Self {
        let parts = iter.into_iter().collect();
        DayPartsBuilder { parts }
    }
}
