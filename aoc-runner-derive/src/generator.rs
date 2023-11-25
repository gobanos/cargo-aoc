use crate::types::Generator;
use crate::utils;
use crate::AOC_RUNNER;
use aoc_runner_internal::{DayPart, Part};
use proc_macro as pm;
use syn::*;

pub fn generator_impl(args: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    let (day, part, name) = utils::extract_meta(args);
    let day = day
        .to_string()
        .parse()
        .expect("generators must have defined day");
    let part = part.and_then(|p| p.to_string().parse().ok());
    let name = name.map(|i| i.to_string());

    let input_cloned = input.clone();

    let input = parse_macro_input!(input as ItemFn);

    let fn_name = input.sig.ident;
    let out_t = if let ReturnType::Type(_, p) = input.sig.output {
        p
    } else {
        panic!("cannot find output type for {}", fn_name)
    };

    let (special_type, out_t) = if let Some((ty, inner)) = utils::extract_result(&out_t) {
        (Some(ty), Box::new(inner))
    } else {
        (None, out_t)
    };

    AOC_RUNNER.with(|map| {
        let mut map = map
            .borrow_mut()
            .expect("failed to borrow shared map from generator");

        let mut register = |p: Part| {
            let runner = map
                .entry(DayPart {
                    day,
                    part: p,
                    name: name.clone(),
                })
                .or_default();
            runner.with_generator(Generator::new(&fn_name, &out_t, special_type));
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
