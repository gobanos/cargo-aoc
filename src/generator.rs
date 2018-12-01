use crate::types::Generator;
use crate::utils;
use crate::AOC_RUNNER;
use aoc_runner_internal::{DayPart, Part};
use proc_macro as pm;
use syn::*;

pub fn generator_impl(args: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    let (day, part, name) = utils::extract_meta(args);
    let day = day.to_string().parse().unwrap();
    let part = part.and_then(|p| p.to_string().parse().ok());
    let name = name.map(|i| i.to_string());

    let input_cloned = input.clone();

    let input = parse_macro_input!(input as ItemFn);

    let fn_name = input.ident;
    let decl = input.decl;
    let out_t = if let ReturnType::Type(_, p) = decl.output {
        p
    } else {
        panic!()
    };

    AOC_RUNNER.with(|map| {
        let mut map = map.borrow_mut().unwrap();

        let mut register = |p: Part| {
            let runner = map
                .entry(DayPart {
                    day,
                    part: p,
                    name: name.clone(),
                })
                .or_default();
            runner.with_generator(Generator::new(fn_name.clone(), out_t.clone()));
        };

        if let Some(p) = part {
            register(p);
        } else {
            register(Part(1));
            register(Part(2));
        }
    });

    input_cloned
}
