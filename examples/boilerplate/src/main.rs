use aoc_runner::{Generator, RunnerV2};
use boilerplate::__aoc::*;
use boilerplate::day1;
use std::borrow::Borrow;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../../aoc-2018/input/2018/day1.txt");

    {
        let generator = &Day1Part1Generator;
        let parsed_input = generator.generate(input)?;
        let runner = &Day1Part1Runner;
        let result = runner.run(parsed_input.borrow())?;
        println!("day1 - part1: {}", result);
    }

    {
        let generator = &Day1Part2Generator;
        let parsed_input = generator.generate(input)?;
        println!("day1 - part2: {}", day1::part2(parsed_input.borrow()));
    }

    {
        let generator =
            &day1::parse_input_day1_unwrap__aoc_generator::Day1Part1UnwrapGenerator::default();
        let parsed_input = generator.generate(input)?;
        println!("day1 - part1: {}", day1::part1(parsed_input.borrow()));
    }

    {
        let generator =
            &day1::parse_input_day1_unwrap__aoc_generator::Day1Part2UnwrapGenerator::default();
        let parsed_input = generator.generate(input)?;
        println!("day1 - part2: {}", day1::part2(parsed_input.borrow()));
    }

    Ok(())
}
