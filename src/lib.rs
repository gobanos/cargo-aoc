extern crate proc_macro;
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
use proc_macro::TokenStream;

thread_local! {
    static AOC_RUNNER: Map = Map::new();
}

#[proc_macro_derive(Runner, attributes(runner, runner_type))]
pub fn aoc_runner_derive(input: TokenStream) -> TokenStream {
    derive::aoc_runner_derive_impl(input)
}

#[proc_macro_attribute]
pub fn aoc(args: TokenStream, input: TokenStream) -> TokenStream {
    runner::runner_impl(args, input)
}

#[proc_macro_attribute]
pub fn aoc_generator(args: TokenStream, input: TokenStream) -> TokenStream {
    generator::generator_impl(args, input)
}

#[proc_macro]
pub fn aoc_lib(input: TokenStream) -> TokenStream {
    out::lib_impl(input)
}

#[proc_macro]
pub fn aoc_main(input: TokenStream) -> TokenStream {
    out::main_impl(input)
}
