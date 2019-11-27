use crate::utils::{to_camelcase, to_input, to_snakecase};
use aoc_runner_internal::{Day, DayPart, DayParts, DayPartsBuilder, Part};
use proc_macro as pm;
use proc_macro2 as pm2;
use quote::quote;
use std::collections::HashSet;
use std::fs::read_dir;
use std::path::Path;
use std::{env, error};
use std::convert::TryInto;

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
    let infos = parse_lib_infos(input).expect("failed to parse lib infos");

    unimplemented!()
//    AOC_RUNNER.with(|map| {
//        let map = map.consume().expect("failed to consume map from lib");
//
//        let year = infos.year;
//
//        write_infos(&map, year).expect("failed to write infos from lib");
//
//        pm::TokenStream::from(headers(&map, year))
//    })
}

pub fn main_impl(input: pm::TokenStream) -> pm::TokenStream {
    let infos = parse_main_infos(input).expect("failed to parse main infos");

    unimplemented!()
//    AOC_RUNNER.with(|map| {
//        let map = map.consume().expect("failed to consume map from main");
//
//        let expanded = match infos {
//            MainInfos::Ref { lib } => {
//                let infos = read_infos().expect("failed to read infos from ref main");
//                body(&infos, Some(lib))
//            }
//            MainInfos::Standalone { year } => {
//                let infos =
//                    write_infos(&map, year).expect("failed to write infos from standalone main");
//                let headers = headers(&map, year);
//                let body = body(&infos, None);
//
//                quote! {
//                    #headers
//
//                    #body
//                }
//            }
//        };
//
//        pm::TokenStream::from(expanded)
//    })
}

fn headers(year: u32) -> pm2::TokenStream {
    let day_parts_headers: pm2::TokenStream = base_day_part()
        .map(|day_part| {
            let generator_struct = to_camelcase(day_part, "Generator");
            let runner_struct = to_camelcase(day_part, "Runner");

            quote! {
                pub struct #generator_struct;

                impl<'a> Generator<'a> for &#generator_struct {
                    type Output = &'a str;

                    fn generate(&self, input: &'a str) -> Result<Self::Output, Box<dyn Error>> {
                        Ok(input)
                    }

                    fn is_default(&self) -> bool {
                        true
                    }
                }

                pub struct #runner_struct<I>(pub PhantomData<I>);

                impl<'a, I> RunnerV2<'a, I> for &#runner_struct<I> {
                    type Output = Void;

                    fn run(&self, _input: I) -> Result<Self::Output, Box<dyn Error>> {
                        Err(Box::new(NotImplemented))
                    }

                    fn is_implemented(&self) -> bool {
                        false
                    }
                }
            }
        })
        .collect();

    quote! {
        #[doc(hidden)]
        pub mod __aoc {
            use aoc_runner::{Generator, NotImplemented, RunnerV2, Void};
            use std::error::Error;
            use std::marker::PhantomData;

            pub const YEAR : u32 = #year;

            #day_parts_headers
        }
    }
}

fn body(infos: &DayParts, lib: Option<pm2::Ident>) -> pm2::TokenStream {
    let mut days: Vec<_> = base_day_part().map(|dp| dp.day).collect();
    days.sort();
    days.dedup();

    let input_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string()))
        .join("input")
        .join(&infos.year.to_string());

    let days_with_input = read_dir(&input_dir)
        .map(|files| {
            files
                .filter_map(Result::ok)
                .map(|file| file.file_name())
                .filter_map(|file_name| {
                    let file_name = file_name.to_str()?;
                    if file_name.starts_with("day") && file_name.ends_with(".txt") {
                        Some(file_name[..file_name.len() - 4].parse::<Day>().ok()?)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_else(|_| HashSet::new());

    let inputs: pm2::TokenStream = days
        .into_iter()
        .map(|d| {
            let name = to_input(d);

            let input = format!("../input/{}/{}.txt", infos.year, d);
            if days_with_input.contains(&d) {
                quote! { let #name = Some(include_str!(#input)); }
            } else {
                quote! { let #name = None; }
            }
        })
        .collect();

    let body : pm2::TokenStream = base_day_part().map(|dp| {
        let (pattern, err) = if let Some(n) = &dp.name {
            (
                format!(
                    "Day {} - Part {} - {}: {{}}\n\tgenerator: {{:?}},\n\trunner: {{:?}}\n",
                    dp.day.as_u8(), dp.part.as_u8(), n
                ),
                format!(
                    "Day {} - Part {} - {}: FAILED while {{}}:\n{{:#?}}\n",
                    dp.day.as_u8(), dp.part.as_u8(), n
                )
            )
        } else {
            (
                format!(
                    "Day {} - Part {}: {{}}\n\tgenerator: {{:?}},\n\trunner: {{:?}}\n",
                    dp.day.as_u8(), dp.part.as_u8()
                ),
                format! (
                    "Day {} - Part {}: FAILED while {{}}:\n{{:#?}}\n",
                    dp.day.as_u8(), dp.part.as_u8()
                )
            )
        };

        let input = to_input(dp.day);
        let generator = to_camelcase(dp, "Generator");
        let runner = to_camelcase(dp, "Runner");

        quote! {
            {
                if let Some(input) = #input {
                    let start_time = Instant::now();

                    let generator = &#generator;
                    match generator.generate(input) {
                        Ok(parsed_input) => {
                            let runner = &#runner(PhantomData);
                            if runner.is_implemented() {
                                let inter_time = Instant::now();
                                match runner.run(&parsed_input) {
                                    Ok(result) => {
                                        let final_time = Instant::now();
                                        println!(#pattern, result, (inter_time - start_time), (final_time - inter_time));
                                    },
                                    Err(e) => eprintln!(#err, "running", e),
                                }
                            }
                        },
                        Err(e) => eprintln!(#err, "generating", e),
                    }
                }
            }
        }
    }).collect();
    if let Some(lib) = lib {
        quote! {
            fn main() {
                use #lib::__aoc::*;
                use aoc_runner::{Generator, RunnerV2};
                use std::marker::PhantomData;
                use std::time::{Duration, Instant};

                #inputs

                println!("Advent of code {}", YEAR);

                #body
            }
        }
    } else {
        quote! {
            fn main() {
                use crate::__aoc::*;
                use aoc_runner::{Generator, RunnerV2};
                use std::marker::PhantomData;
                use std::time::{Duration, Instant};

                #inputs

                println!("Advent of code {}", YEAR);

                #body
            }
        }
    }
}

fn write_infos(year: u32) -> Result<DayParts<'static>, Box<dyn error::Error>> {
    unimplemented!()
//    let mut day_parts = base_day_part()
//        .filter_map(|(dp, runner)| {
//            if runner.solver.is_some() {
//                Some(dp.clone())
//            } else {
//                None
//            }
//        })
//        .collect::<DayPartsBuilder>()
//        .with_year(year);
//
//    day_parts.sort();
//
//    day_parts.save()?;
//
//    Ok(day_parts)
}

fn parse_lib_infos(infos: pm::TokenStream) -> Result<LibInfos, ()> {
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
        Err(())
    }
}

fn base_day_part() -> impl Iterator<Item = DayPart<'static>> {
    (1..=25).into_iter().flat_map(|day| {
        (1..=2).into_iter().flat_map(move |part| {
            (0..=8).into_iter().map(move |alt| {
                DayPart {
                    day: day.try_into().unwrap(),
                    part: part.try_into().unwrap(),
                    alt: alt.try_into().ok(),
                    name: None,
                }
            })
        })
    })
}
