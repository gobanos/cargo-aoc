//use boilerplate::day1;
use boilerplate::__aoc::*;
use std::error::Error;
use std::borrow::Borrow;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../../aoc-2018/input/2018/day1.txt");

    {
        let generator = &Day1Part1Generator;
        let parsed_input = generator.generate(input)?;

        println!("day1 - part1: {}", day1::part1(parsed_input.borrow()));
    }

    {
        let generator = &Day1Part2Generator;
        let parsed_input = generator.generate(input)?;
        println!("day1 - part2: {}", day1::part2(parsed_input.borrow()));
    }

    {
        let generator = &day1::Day1Part1UnwrapGenerator::default();
        let parsed_input = generator.generate(input)?;
        println!("day1 - part1: {}", day1::part1(parsed_input.borrow()));
    }

    {
        let generator = &day1::Day1Part2UnwrapGenerator::default();
        let parsed_input = generator.generate(input)?;
        println!("day1 - part2: {}", day1::part2(parsed_input.borrow()));
    }

    Ok(())
}