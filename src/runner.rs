use proc_macro::TokenStream;
use quote::quote;
use syn::*;
use types::Solver;
use utils;
use AOC_RUNNER;
use utils::to_snakecase;
use utils::to_camelcase;

pub fn runner_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let (day, part) = utils::extract_meta(args);
    let day = day.to_string().parse().unwrap();
    let part = part.expect("missing part").to_string().parse().unwrap();

    let input = parse_macro_input!(input as ItemFn);

    let original_fn = input.clone();

    let name = input.ident;
    let decl = input.decl;
    let out_t = if let ReturnType::Type(_, p) = decl.output {
        p
    } else {
        panic!()
    };

    let def = AOC_RUNNER.with(|map| {
        let mut map = map.borrow_mut();
        let runner = map.entry((day, part)).or_default();

        runner.with_solver(Solver::new(name.clone(), out_t.clone()));

        if let Some(generator) = &runner.generator {
            let gen_out_t = &generator.out_t;
            let gen_name = &generator.name;

            quote! {
                #[runner(#name, #gen_name)]
                pub struct RunnerStruct {
                    input: #gen_out_t,
                    output: PhantomData<#out_t>,
                }
            }
        } else {
            quote! {
                #[runner(#name)]
                pub struct RunnerStruct {
                    input: ArcStr,
                    output: PhantomData<#out_t>,
                }
            }
        }
    });

    let mod_name = to_snakecase(day, part);
    let trait_name = to_camelcase(day, part);

    TokenStream::from(quote! {
        #original_fn

        #[allow(unused_imports)]
        mod #mod_name {
            use super::*;
            use aoc_runner::{ArcStr, Runner, RunnerGen};
            use std::marker::PhantomData;
            use {Factory, #trait_name};

            impl #trait_name for Factory {
                fn #mod_name(&self, input: ArcStr) -> Box<Runner> {
                    Box::new( RunnerStruct::gen(input) )
                }
            }

            #[derive(Runner)]
            #def
        }
    })
}
