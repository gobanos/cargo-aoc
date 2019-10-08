use aoc_runner_internal::DayPart;
use crate::types::{Generator, Solver, SpecialType};
use crate::utils::{self, extract_result, to_camelcase, to_snakecase};
use crate::AOC_RUNNER;
use proc_macro as pm;
use proc_macro2 as pm2;
use quote::quote;
use syn::*;

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

        runner.with_solver(Solver::new(&fn_name, &out_t, special_type));

        let derive = build_derive(runner.solver.as_ref().unwrap(), runner.generator.as_ref());

        if let Some(generator) = &runner.generator {
            let gen_out_t = &generator.get_out_t();

            quote! {
                pub struct RunnerStruct {
                    input: #gen_out_t,
                    output: PhantomData<#out_t>,
                }

                #derive
            }
        } else {
            quote! {
                pub struct RunnerStruct {
                    input: ArcStr,
                    output: PhantomData<#out_t>,
                }

                #derive
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
            use std::error::Error;
            use std::fmt::Display;
            use std::borrow::Borrow;
            use crate::{Factory, #trait_name};

            impl #trait_name for Factory {
                fn #mod_name(input: ArcStr) -> Result<Box<dyn Runner>, Box<dyn Error>> {
                    Ok(Box::new( RunnerStruct::try_gen(input)? ))
                }
            }

            #def
        }
    })
}

fn build_derive(solver: &Solver, generator: Option<&Generator>) -> pm2::TokenStream {
    let fn_runner = solver.get_name();

    // Build the output, possibly using quasi-quotation
    let input = if let Some(generator) = generator {
        let fn_generator = generator.get_name();
        quote! {
            input: #fn_generator(input.borrow())
        }
    } else {
        quote! {
            input
        }
    };

    let gen = if let Some(t) = generator.and_then(|g| g.special_type) {
        let input = match t {
            SpecialType::Result => quote! { #input? },
            SpecialType::Option => quote! { #input.ok_or("generator produce no value")? },
        };

        quote! {
            fn gen(input: ArcStr) -> Self {
                Self::try_gen(input).expect("failed to generate input")
            }

            fn try_gen(input: ArcStr) -> Result<Self, Box<dyn Error>> {
                Ok( RunnerStruct {
                    #input,
                    output: PhantomData,
                } )
            }
        }
    } else {
        quote! {
            fn gen(input: ArcStr) -> Self {
                RunnerStruct {
                    #input,
                    output: PhantomData,
                }
            }
        }
    };

    let run = if let Some(t) = solver.special_type {
        let runner = match t {
            SpecialType::Result => quote! { #fn_runner(self.input.borrow())? },
            SpecialType::Option => {
                quote! { #fn_runner(self.input.borrow()).ok_or("runner produce no value")? }
            }
        };

        quote! {
            fn run(&self) -> Box<dyn Display> {
                self.try_run().expect("failed to run")
            }

            fn try_run(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
                Ok( Box::new( #runner ) )
            }

            fn bench(&self, black_box: fn(&dyn Display)) {
                black_box( &#fn_runner(self.input.borrow()).unwrap() )
            }
        }
    } else {
        quote! {
            fn run(&self) -> Box<dyn Display> {
                Box::new( #fn_runner(self.input.borrow()) )
            }

            fn bench(&self, black_box: fn(&dyn Display)) {
                black_box( &#fn_runner(self.input.borrow()) )
            }
        }
    };

    quote! {
        impl Runner for RunnerStruct {
            #gen

            #run
        }
    }
}
