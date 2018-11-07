use proc_macro::TokenStream;
use quote::quote;
use std::fs;
use std::io::Write;
use AOC_RUNNER;

pub fn lib_impl(_input: TokenStream) -> TokenStream {
    AOC_RUNNER.with(|map| {
        fs::create_dir_all("target/aoc").unwrap();
        let mut f = fs::File::create("target/aoc/test.txt").unwrap();

        write!(f, "{:#?}", map.borrow()).unwrap();
    });

    TokenStream::new()
}

pub fn main_impl(_input: TokenStream) -> TokenStream {
    AOC_RUNNER.with(|map| {
        fs::create_dir_all("target/aoc").unwrap();
        let mut f = fs::File::create("target/aoc/test.txt").unwrap();

        write!(f, "{:#?}", map.borrow()).unwrap();
    });

    let expanded = quote!{
        fn main() {
            println!("Hello World !");
        }
    };

    TokenStream::from(expanded)
}
