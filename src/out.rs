use proc_macro::TokenStream;
use quote::quote;
use std::fs;
use std::io::Write;
use AOC_RUNNER;
use utils::to_camelcase;
use utils::to_snakecase;

pub fn lib_impl(_input: TokenStream) -> TokenStream {
    AOC_RUNNER.with(|map| {
        fs::create_dir_all("target/aoc").unwrap();
        let mut f = fs::File::create("target/aoc/test.txt").unwrap();
        write!(f, "{:#?}", map.borrow()).unwrap();

        let mut previous_quote = quote! {
            use aoc_runner::{Runner, ArcStr};

            pub struct Factory();
        };

        for &(d, p) in map.borrow().keys() {
            let snake = to_snakecase(d, p);
            let camel = to_camelcase(d, p);

            previous_quote = quote! {
                #previous_quote

                pub trait #camel {
                    fn #snake(&self, input: ArcStr) -> Box<Runner>;
                }
            };
        }

        TokenStream::from(quote! {
            pub use aoc_factory::*;

            #[allow(unused)]
            mod aoc_factory {
                #previous_quote
            }
        })
    })
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
