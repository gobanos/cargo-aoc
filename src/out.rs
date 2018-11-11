use proc_macro as pm;
use proc_macro2 as pm2;
use quote::quote;
use std::fs;
use std::io::Write;
use utils::to_camelcase;
use utils::to_snakecase;
use AOC_RUNNER;
use map::InnerMap;
use std::cell::Ref;

#[derive(Debug)]
struct LibInfos {
    year: u32,
}

#[derive(Debug)]
enum MainInfos {
    Ref { lib: pm2::Ident },
    Standalone { year: u32 },
}

pub fn lib_impl(input: pm::TokenStream) -> pm::TokenStream {
    let infos = parse_lib_infos(input).unwrap();
    println!("{:?}", infos);

    AOC_RUNNER.with(|map| {
        let map = map.consume().unwrap();
        fs::create_dir_all("target/aoc").unwrap();
        let mut f = fs::File::create("target/aoc/test.txt").unwrap();
        write!(f, "{:#?}", map).unwrap();

        let year = infos.year;

        pm::TokenStream::from(headers(map, year))
    })
}

pub fn main_impl(input: pm::TokenStream) -> pm::TokenStream {
    let infos = parse_main_infos(input).unwrap();

    AOC_RUNNER.with(|map| {
        let map = map.consume().unwrap();
        fs::create_dir_all("target/aoc").unwrap();
        let mut f = fs::File::create("target/aoc/test.txt").unwrap();
        write!(f, "{:#?}", map).unwrap();

        let expanded = match infos {
            MainInfos::Ref { lib } => body(Some(lib)),
            MainInfos::Standalone { year } => {
                let headers = headers(map, year);
                let body = body(None);

                quote! {
                    #headers

                    #body
                }
            }
        };

        pm::TokenStream::from(expanded)
    })
}

fn headers(map: Ref<InnerMap>, year: u32) -> pm2::TokenStream {
    let mut previous_quote = quote! {
        use aoc_runner::{Runner, ArcStr};

        pub static YEAR : u32 = #year;

        pub struct Factory();
    };

    for &(d, p) in map.keys() {
        let snake = to_snakecase(d, p);
        let camel = to_camelcase(d, p);

        previous_quote = quote! {
            #previous_quote

            pub trait #camel {
                fn #snake(input: ArcStr) -> Box<Runner>;
            }
        };
    }

    quote! {
        pub use aoc_factory::*;

        #[allow(unused)]
        mod aoc_factory {
            #previous_quote
        }
    }
}

fn body(lib: Option<pm2::Ident>) -> pm2::TokenStream {
    if let Some(lib) = lib {
        quote! {
            use #lib::*;

            fn main() {
                println!("Advent of code {}", YEAR);
            }
        }
    } else {
        quote! {
            fn main() {
                println!("Hello world !");
            }
        }
    }
}

fn parse_lib_infos(infos: pm::TokenStream) -> Result<LibInfos, ()> {
    println!("{:?}", infos);

    let tokens: Vec<_> = infos.into_iter().collect();

    if let pm::TokenTree::Ident(i) = tokens.get(0).ok_or(())? {
        if i.to_string() != "year" {
            return Err(());
        }
    } else {
        return Err(());
    }

    if let pm::TokenTree::Punct(p) = tokens.get(1).ok_or(())? {
        if p.as_char() != '=' {
            return Err(());
        }
    } else {
        return Err(());
    }

    if let pm::TokenTree::Literal(l) = tokens.get(2).ok_or(())? {
        let repr = l.to_string();

        let year = repr.parse().map_err(|_| ())?;

        Ok(LibInfos { year })
    } else {
        Err(())
    }
}

fn parse_main_infos(infos: pm::TokenStream) -> Result<MainInfos, ()> {
    println!("{:?}", infos);

    let tokens: Vec<_> = infos.into_iter().collect();

    if let pm::TokenTree::Punct(p) = tokens.get(1).ok_or(())? {
        if p.as_char() != '=' {
            return Err(());
        }
    } else {
        return Err(());
    }

    if let pm::TokenTree::Ident(i) = tokens.get(0).ok_or(())? {
        let ty = i.to_string();

        Ok(match ty.as_ref() {
            "year" => {
                let repr = if let pm::TokenTree::Literal(l) = tokens.get(2).ok_or(())? {
                    l.to_string()
                } else {
                    return Err(());
                };

                let year = repr.parse().map_err(|_| ())?;
                MainInfos::Standalone { year }
            }
            "lib" => {
                let lib = if let pm::TokenTree::Ident(i) = tokens.get(2).ok_or(())? {
                    pm2::Ident::new(&i.to_string(), pm2::Span::from(i.span()))
                } else {
                    return Err(());
                };

                MainInfos::Ref { lib }
            }
            _ => return Err(()),
        })
    } else {
        return Err(());
    }
}
