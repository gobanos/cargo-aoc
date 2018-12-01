use aoc_runner_internal::DayPart;
use proc_macro as pm;
use quote::quote;
use syn::*;
use types::Solver;
use utils::{self, to_camelcase, to_snakecase};
use AOC_RUNNER;

pub fn runner_impl(args: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    let (day, part, name) = utils::extract_meta(args);
    let day = day.to_string().parse().unwrap();
    let part = part.expect("missing part").to_string().parse().unwrap();
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

    let def = AOC_RUNNER.with(|map| {
        let mut map = map.borrow_mut().unwrap();

        let dp = dp.clone();
        let def = dp.without_name();

        if !map.contains_key(&dp) && map.contains_key(&def) {
            let mut val = map[&def].clone();
            val.solver = None;
            map.insert(dp.clone(), val);
        }

        let runner = map.entry(dp).or_default();

        runner.with_solver(Solver::new(fn_name.clone(), out_t.clone()));

        if let Some(generator) = &runner.generator {
            let gen_out_t = &generator.out_t;
            let gen_name = &generator.name;

            quote! {
                #[runner(#fn_name, #gen_name)]
                pub struct RunnerStruct {
                    input: #gen_out_t,
                    output: PhantomData<#out_t>,
                }
            }
        } else {
            quote! {
                #[runner(#fn_name)]
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
            use std::marker::PhantomData;
            use crate::{Factory, #trait_name};

            impl #trait_name for Factory {
                fn #mod_name(input: ArcStr) -> Box<Runner> {
                    Box::new( RunnerStruct::gen(input) )
                }
            }

            #[derive(Runner)]
            #def
        }
    })
}
