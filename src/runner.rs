use aoc_runner_internal::DayPart;
use crate::types::Solver;
use crate::utils::{self, to_camelcase, to_snakecase};
use crate::AOC_RUNNER;
use proc_macro as pm;
use quote::quote;
use syn::*;
use types::SpecialType;
use utils::extract_result;

pub fn runner_impl(args: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    let (day, part, name) = utils::extract_meta(args);
    let day = day
        .to_string()
        .parse()
        .expect("runners must have a defined day");
    let part = part
        .expect("runners must have a defined part")
        .to_string()
        .parse()
        .expect("runners must have a defined part");
    let name = name.map(|i| i.to_string());

    let dp = DayPart { day, part, name };

    let input = parse_macro_input!(input as ItemFn);

    let original_fn = input.clone();

    let fn_name = input.ident;
    let decl = input.decl;
    let out_t = if let ReturnType::Type(_, p) = decl.output {
        p
    } else {
        panic!()
    };

    let (special_type, out_t) = if let Some((ty, inner)) = extract_result(&*out_t) {
        (Some(ty), Box::new(inner))
    } else {
        (None, out_t)
    };

    let def = AOC_RUNNER.with(|map| {
        let mut map = map
            .borrow_mut()
            .expect("failed to borrow shared map from runner");

        let dp = dp.clone();
        let def = dp.without_name();

        if !map.contains_key(&dp) && map.contains_key(&def) {
            let mut val = map[&def].clone();
            val.solver = None;
            map.insert(dp.clone(), val);
        }

        let runner = map.entry(dp).or_default();

        runner.with_solver(Solver::new(&fn_name, &out_t));

        let run_special = match special_type {
            Some(SpecialType::Result) => quote! { #[run_result] },
            Some(SpecialType::Option) => quote! { #[run_option] },
            None => quote!{},
        };

        if let Some(generator) = &runner.generator {
            let gen_out_t = &generator.get_out_t();
            let gen_name = &generator.get_name();
            let gen_special = match generator.special_type {
                Some(SpecialType::Result) => quote! { #[gen_result] },
                Some(SpecialType::Option) => quote! { #[gen_option] },
                None => quote!{},
            };

            quote! {
                #[runner(#fn_name, #gen_name)]
                #gen_special
                #run_special
                pub struct RunnerStruct {
                    input: #gen_out_t,
                    output: PhantomData<#out_t>,
                }
            }
        } else {
            quote! {
                #[runner(#fn_name)]
                #run_special
                pub struct RunnerStruct {
                    input: ArcStr,
                    output: PhantomData<#out_t>,
                }
            }
        }
    });

    let mod_name = to_snakecase(&dp);
    let trait_name = to_camelcase(&dp);

    pm::TokenStream::from(quote! {
        #original_fn

        #[allow(unused_imports)]
        mod #mod_name {
            use super::*;
            use aoc_runner::{ArcStr, Runner};
            use aoc_runner_derive::Runner;
            use std::marker::PhantomData;
            use std::error::Error;
            use std::fmt::Display;
            use crate::{Factory, #trait_name};

            impl #trait_name for Factory {
                fn #mod_name(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>> {
                    Ok(Box::new( RunnerStruct::try_gen(input)? ))
                }
            }

            #[derive(Runner)]
            #def
        }
    })
}
