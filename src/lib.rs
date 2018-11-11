extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

mod derive;
mod generator;
mod map;
mod out;
mod runner;
mod types;
mod utils;

use map::Map;
use proc_macro as pm;

thread_local! {
    static AOC_RUNNER: Map = Map::new();
}

#[proc_macro_derive(Runner, attributes(runner, runner_type))]
pub fn aoc_runner_derive(input: pm::TokenStream) -> pm::TokenStream {
    derive::aoc_runner_derive_impl(input)
}

#[proc_macro_attribute]
pub fn aoc(args: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    runner::runner_impl(args, input)
}

#[proc_macro_attribute]
pub fn aoc_generator(args: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    generator::generator_impl(args, input)
}

#[proc_macro]
pub fn aoc_lib(input: pm::TokenStream) -> pm::TokenStream {
    out::lib_impl(input)
}

#[proc_macro]
pub fn aoc_main(input: pm::TokenStream) -> pm::TokenStream {
    out::main_impl(input)
}
