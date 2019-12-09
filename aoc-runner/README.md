# Advent of Code Runner

This is a simple project that aims to be a runner for the [Advent of Code](https://adventofcode.com). 

Implement your solution. Let us do the rest.

# Features
* Input downloading 
* Running your solution 
* Benchmarking of your solution (WIP)

# Getting started

* Create a lib project `cargo new advent-of-code-2018 --lib`
* Add deps to your Cargo.toml: 
```
aoc-runner = "0.2.0"
aoc-runner-derive = "0.2.0"
```
* Include libs in your lib.rs
```
extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;
```

* Add `aoc_lib!{ year = 2018 }` at the end of your lib.rs
* Start coding !

# Flags your solutions

just add a `#[aoc(day1, part1)]` before your function !
```
#[aoc(day1, part1)]
fn part1(input: &str) -> i32 {
    ...
}
```
Supported signatures : `&str` or `&[u8]` as input, any type implementing display as output.
For custom input, see below.

# Custom Generators

You need to pre-process input in a separated function ? generators are for you !
```
#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Gift> {
    ...
}

#[aoc(day2, part1)]
fn part1(input: &[Gift]) -> u32 {
    ...
}
```

# Run your code
See [cargo-aoc](https://github.com/gobanos/cargo-aoc)
