use proc_macro::TokenStream;
use quote::quote;
use syn::*;
use types::Solver;
use utils;
use AOC_RUNNER;

pub fn runner_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let (day, part) = utils::extract_meta(args);
    let day = day.to_string().parse().unwrap();
    let part = part.expect("missing part").to_string().parse().unwrap();

    let input = parse_macro_input!(input as ItemFn);

    let name = input.ident;
    let decl = input.decl;
    let arg_decl = if let FnArg::Captured(a) = decl.inputs[0].clone() {
        a
    } else {
        panic!()
    };
    let arg = arg_decl.pat;
    let arg_t = arg_decl.ty;
    let out_t = if let ReturnType::Type(_, p) = decl.output {
        p
    } else {
        panic!()
    };
    let body = input.block;

    let expanded = AOC_RUNNER.with(|map| {
        let mut map = map.borrow_mut();
        let runner = map.entry((day, part)).or_default();

        runner.with_solver(Solver::new(name.clone(), out_t.clone()));

        if let Some(generator) = &runner.generator {
            let gen_out_t = &generator.out_t;
            let gen_name = &generator.name;
            quote! {
                pub use self::#name::runner as #name;

                #[allow(unused_imports)]
                mod #name {
                    use super::*;
                    use aoc_runner::{ArcStr, Runner};
                    use std::marker::PhantomData;

                    pub fn runner(#arg: #arg_t) -> #out_t {
                        #body
                    }

                    #[derive(Runner)]
                    #[runner(runner, #gen_name)]
                    pub struct RunnerStruct {
                        input: #gen_out_t,
                        output: PhantomData<#out_t>,
                    }
                }
            }
        } else {
            quote! {
                pub use self::#name::runner as #name;

                #[allow(unused_imports)]
                mod #name {
                    use super::*;
                    use aoc_runner::{ArcStr, Runner};
                    use std::marker::PhantomData;

                    pub fn runner(#arg: #arg_t) -> #out_t {
                        #body
                    }

                    #[derive(Runner)]
                    #[runner(runner)]
                    pub struct RunnerStruct {
                        input: ArcStr,
                        output: PhantomData<#out_t>,
                    }
                }
            }
        }
    });

    TokenStream::from(expanded)
}
