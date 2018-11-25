extern crate {CRATE_SLUG};
extern crate aoc_runner;

use {CRATE_SLUG}::*;

fn main() {
    use std::time::Instant;
    use aoc_runner::ArcStr;

    let start_time = Instant::now();
    let runner = Factory::day1_part1(ArcStr::from(include_str!("../../../../input/day1")));
    let inter_time = Instant::now();
    let result = runner.run();
    let final_time = Instant::now();
    println!("Day 1 - Part 1 : {}\n\tgenerator: {:?},\n\trunner: {:?}\n", result, (inter_time - start_time), (final_time - inter_time));
}