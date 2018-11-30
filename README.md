# Cargo Advent of Code Helper

`cargo-aoc` is a simple CLI tool that aims to be a helper for the [Advent of Code](https://adventofcode.com). 

Implement your solution. Let us handle the rest.

# Features
* Input downloading 
* Running your solution 
* Automatic benchmarking of your solution using [Criterion](https://github.com/japaric/criterion.rs)

# Getting started

## Install `cargo aoc`

`cargo-aoc` is hosted as a binary on crates.io.
Boot a terminal and install the program using `cargo install cargo-aoc`

## Setting up the CLI

You will need to find your session token for the AoC in order for cargo-aoc to work. Thankfully, finding your token is easy since it is stored in your Browser's cookies. Open up the devtools of your browser, and then :

* Firefox: "Storage" tab, Cookies, and copy the "Value" field of the `session` cookie.
* Google Chrome / Chromium: "Application" tab, Cookies, and copy the "Value" field of the `session` cookie.

Once you have it, simply run : `cargo aoc credentials -s {token}`

You're now ready to start coding ! 

NOTE: If for some reason your token has changed, dont forget to change it back. 

`cargo aoc credentials` will show the currently stored user token

## Setting up the project

In order for `cargo-aoc` to work properly, you have to set the project up correctly. 

If you get lost during the process, you can take [this example repository of AoC 2015](https://github.com/gobanos/advent-of-code-2015) as a template.

First, you must add a dependency on `aoc-runner` and `aoc-runner-derive` in your `Cargo.toml`.
In your `src/lib.rs`, add **each day as a separate module** using `pub mod dayX.rs`.
At the end of the `src/lib.rs`, you will have to use the macro aoc_lib!{ year = XXXX }, where XXXX is the 
year of the AoC puzzles being solved.

When implementing a solution for a day, you have to provide functions and tag them accordingly.
A function is either a **solver** or a **generator**. 

Those two types of functions are being executed and benchmarked seperately. Lets have a closer look : 

### Generator functions

Generators allows you to provide a custom type to the solver functions. Sometimes in AoC, you have to parse 
an input and extract a logical structure out of it, before you can actually solve the problem. 

Generator functions are tagged `#[aoc_generator(dayX)]`.

Because examples are worth a thousand words, lets take a look at [Year 2015, Day 2](https://adventofcode.com/2015/day/2) : 

From the puzzle's description, we know that `[we] have a list of the dimensions (length l, width w, and height h) of each present`, each present on one line, representend like so: `{L}x{W}x{H}`.

We might want to first parse the input and extract logical `Gift` structs out of it, like: 

```
pub struct Gift {
    l: u32,
    w: u32,
    h: u32
}
``` 

In @Gobanos' reference implementation, we can see that he instead chose to settle for a custom type :
`type Gift = (u32, u32, u32);`.

Thus, writing a generator for `Gift`s is fairly simple: 

```
#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Gift> {
    input
        .lines()
        .map(|l| {
            let mut gift = l.trim().split('x').map(|d| d.parse().unwrap());
            (
                gift.next().unwrap(),
                gift.next().unwrap(),
                gift.next().unwrap(),
            )
        }).collect()
}
``` 

As you can see, generators take a &str type as an input, and outputs any type that you want, so you can then use it in `solver` functions.

You can only have one generator per day, and its output will be used as the input of the solvers if they are tagged with the same day. 

### Solver functions 

Solver functions are typically your algorithms, they take any input type provided by a generator, and return any type that you want to use, provided that it implements the `Display` trait.

Solver functions are tagged `#[aoc(day2, part1)]`. 
Optionally, you can have multiple implementation for the same part of a day. You must then use a name to tag them correctly, for example : `#[aoc(day2, part1, for_loop)]`. 

Following with the previous example, implementing a solver for the part one could be done like this :

```
#[aoc(day2, part1)]
pub fn solve_part1(input: &[Gift]) -> u32 {
    input
        .iter()
        .map(|&(l, w, h)| {
            let (s1, s2) = smallest_side((l, w, h));
            2 * l * w + 2 * w * h + 2 * h * l + s1 * s2
        })
        .sum()
}
``` 

Notice how we're taking the `Gift`s generated previously, and using Rust's iterators to solve the problem efficiently, all the while keeping the code maintainable. 

The output of this particular solver is an `u32`, which of course implements `Display`.
When running your solution using `cargo aoc`, said result will then get printed in the console, along with other informations about execution time.

# Downloading your input manually

`cargo aoc input` will download an input and store it in `input/{year}/day_{day}.txt`. 

Please note that by default, we're taking today's date as the argument. Of course, you can change this using : `cargo aoc input -d {day} -y {year}`

# Running your solution

`cargo aoc` will run the latest implemented day, downloading your input beforehand. It will show you the result, and a short summary of how well it did perform.

Example output on my Chromebook, running [@Gobanos' AOC2015](https://github.com/gobanos/advent-of-code-2015) : 
```
[olivier@olivier-pc advent-of-code-2015]$ cargo aoc
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
   Compiling aoc-autobuild v0.1.0 (/home/olivier/Workspace/Rust/advent-of-code-2015/target/aoc/aoc-autobuild)
    Finished release [optimized] target(s) in 0.87s
     Running `target/release/aoc-autobuild`
AOC 2015
Day 5 - Part 1 : 238
        generator: 18.122µs,
        runner: 420.958µs

Day 5 - Part 2 : 69
        generator: 5.499µs,
        runner: 1.142373ms
```

If you want to run an older puzzle, or only a specific part, specify those using `cargo aoc -d {day} -p {part}`.

# Benchmarking your solution

Benchmarking is powered by [Criterion](https://github.com/japaric/criterion.rs). Use `cargo aoc bench` to launch the benchmarks, just like you would use `cargo aoc`.

Benchmarks for each days are then generated in `target/aoc/aoc-autobench/target/criterion`.

You can open the benchmark automatically in your Browser afterwards, using `cargo aoc bench -o` 

Soon(tm), you will also be able to use our (free) online platform, to compare your results with those of the community.

------

Happy Advent of Code !   