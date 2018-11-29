# Cargo Advent of Code Helper

This is a simple project that aims to be a helper for the [Advent of Code](https://adventofcode.com). 

Implement your solution. Let us do the rest.

# Features
* Input downloading 
* Benchmarking of your solution (WIP)

# Getting started

## Install `cargo aoc`

Boot a terminal and install the program using `cargo install cargo-aoc`

## Setting up

You will need to find your session token for the AoC in order for cargo-aoc to work. Thankfully, finding your token is easy since it is stored in your Browser's cookies. Open up the devtools of your browser, and then :

* Firefox: "Storage" tab, Cookies, and copy the "Value" field of the `session` cookie.
* Google Chrome / Chromium: "Application" tab, Cookies, and copy the "Value" field of the `session` cookie.

Once you have it, simply run : `cargo aoc credentials -s {token}`

You're now ready to start coding ! 

NOTE: If for some reason your token has changed, dont forget to change it back. 

`cargo aoc credentials` will show the currently stored user token

# Input downloading 

`cargo aoc input` will download an input and store it in `input/{year}/{day}.txt`. 

Please note that by default, we're taking today's date as the argument. Of course, you can change this using : 

`cargo aoc input -d {day} -y {year}`

# Run your solution

`cargo aoc` will run your last implemented day, with your own input.

Need to run an older solution, or only a part ? `cargo aoc -d {day} -p {part}` !

# Benchmarking your solution

{ WIP }
