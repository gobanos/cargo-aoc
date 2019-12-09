use aoc_runner::{Generator, Runner};
use boilerplate::__aoc::*;
use boilerplate::day1::__parse_input_day1_unwrap_aoc_generator::{
    Day1Part1UnwrapGenerator, Day1Part2UnwrapGenerator,
};
use std::error::Error;
use std::marker::PhantomData;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../../aoc-2018/input/2018/day1.txt");

    //    {
    //        let generator = &Day1Part1Generator;
    //        let parsed_input = generator.generate(input)?;
    //        let runner = &Day1Part1Runner(PhantomData);
    //        if runner.is_implemented() {
    //            let result = runner.run(&parsed_input)?;
    //            println!("day1 - part1: {}", result);
    //        }
    //    }
    //
    //    {
    //        let generator = &Day1Part2Generator;
    //        let parsed_input = generator.generate(input)?;
    //        let runner = &Day1Part2Runner(PhantomData);
    //        if runner.is_implemented() {
    //            let result = runner.run(&parsed_input)?;
    //            println!("day1 - part2: {}", result);
    //        }
    //    }

    {
        let generator = &Day1Part1UnwrapGenerator::default();
        let parsed_input = generator.generate(input)?;
        let runner = &Day1Part1Runner(PhantomData);
        if runner.is_implemented() {
            let result = runner.run(&parsed_input)?;
            println!("day1 - part1 - unwrap: {}", result);
        }
    }

    {
        let generator = &Day1Part2UnwrapGenerator::default();
        let parsed_input = generator.generate(input)?;
        let runner = &Day1Part2Runner(PhantomData);
        if runner.is_implemented() {
            let result = runner.run(&parsed_input)?;
            println!("day1 - part1 - unwrap: {}", result);
        }
    }

    Ok(())
}
