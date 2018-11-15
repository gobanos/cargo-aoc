use map::InnerMap;
use proc_macro as pm;
use proc_macro2 as pm2;
use quote::quote;
use std::{cmp::Ordering, error, fs};
use types::{Day, Part};
use utils::{to_camelcase, to_snakecase};
use AOC_RUNNER;

#[derive(Debug)]
struct LibInfos {
    year: u32,
}

#[derive(Debug)]
enum MainInfos {
    Ref { lib: pm2::Ident },
    Standalone { year: u32 },
}

#[derive(Serialize, Deserialize, Eq, PartialEq)]
struct DayPart {
    day: Day,
    part: Part,
}

impl PartialOrd for DayPart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for DayPart {
    fn cmp(&self, other: &Self) -> Ordering {
        self.day.cmp(&other.day).then(self.part.cmp(&other.part))
    }
}

pub fn lib_impl(input: pm::TokenStream) -> pm::TokenStream {
    let infos = parse_lib_infos(input).unwrap();

    AOC_RUNNER.with(|map| {
        let map = map.consume().unwrap();

        write_infos(&map).unwrap();

        let year = infos.year;

        pm::TokenStream::from(headers(&map, year))
    })
}

pub fn main_impl(input: pm::TokenStream) -> pm::TokenStream {
    let infos = parse_main_infos(input).unwrap();

    AOC_RUNNER.with(|map| {
        let map = map.consume().unwrap();

        let expanded = match infos {
            MainInfos::Ref { lib } => {
                let infos = read_infos().unwrap();
                body(infos, Some(lib))
            }
            MainInfos::Standalone { year } => {
                let infos = write_infos(&map).unwrap();
                let headers = headers(&map, year);
                let body = body(infos, None);

                quote! {
                    #headers

                    #body
                }
            }
        };

        pm::TokenStream::from(expanded)
    })
}

fn headers(map: &InnerMap, year: u32) -> pm2::TokenStream {
    let traits_impl: pm2::TokenStream = map
        .keys()
        .map(|&(d, p)| {
            let snake = to_snakecase(d, p);
            let camel = to_camelcase(d, p);

            quote! {
                pub trait #camel {
                    fn #snake(input: ArcStr) -> Box<Runner>;
                }
            }
        }).collect();

    quote! {
        pub use aoc_factory::*;

        #[allow(unused)]
        mod aoc_factory {
            use aoc_runner::{Runner, ArcStr};

            pub static YEAR : u32 = #year;

            pub struct Factory();

            #traits_impl
        }
    }
}

fn body(infos: Vec<DayPart>, lib: Option<pm2::Ident>) -> pm2::TokenStream {
    let body : pm2::TokenStream = infos.into_iter().map(|dp| {
        let identifier = to_snakecase(dp.day, dp.part);
        let input = format!("../input/day{}", dp.day.0);
        let pattern = format!(
            "Day {} - Part {} : {{}}\n\tgenerator: {{:?}},\n\trunner: {{:?}}\n",
            dp.day.0, dp.part.0
        );

        quote! {
            {
                use std::time::{Duration, Instant};
                use aoc_runner::ArcStr;

                let start_time = Instant::now();
                let runner = Factory::#identifier(ArcStr::from(include_str!(#input)));
                let inter_time = Instant::now();
                let result = runner.run();
                let final_time = Instant::now();
                println!(#pattern, result, (inter_time - start_time), (final_time - inter_time));
            }
        }
    }).collect();

    if let Some(lib) = lib {
        quote! {
            use #lib::*;

            fn main() {
                println!("Advent of code {}", YEAR);

                #body
            }
        }
    } else {
        quote! {
            fn main() {
                println!("Advent of code {}", YEAR);

                #body
            }
        }
    }
}

fn write_infos(map: &InnerMap) -> Result<Vec<DayPart>, Box<error::Error>> {
    fs::create_dir_all("target/aoc")?;
    let f = fs::File::create("target/aoc/completed.json")?;

    let mut day_parts: Vec<_> = map
        .iter()
        .filter_map(|(&(day, part), runner)| {
            if runner.solver.is_some() {
                Some(DayPart { day, part })
            } else {
                None
            }
        }).collect();
    day_parts.sort();

    serde_json::to_writer_pretty(f, &day_parts)?;

    Ok(day_parts)
}

fn read_infos() -> Result<Vec<DayPart>, Box<error::Error>> {
    let f = fs::File::open("target/aoc/completed.json")?;

    let infos = serde_json::from_reader(f)?;

    Ok(infos)
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
        return Err(());
    }
}
