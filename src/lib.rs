extern crate proc_macro;
extern crate quote;
extern crate syn;

mod types;
mod utils;

use proc_macro::TokenStream;
use quote::quote;
use std::cell::RefCell;
use std::collections::HashMap;
use syn::*;
use types::*;

thread_local! {
    static AOC_RUNNER: RefCell<HashMap<Day, Runner>> = RefCell::new(HashMap::new());
}

#[proc_macro_derive(Runner, attributes(runner, runner_type))]
pub fn aoc_runner_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    let attrs = ast
        .attrs
        .iter()
        .filter_map(get_meta_items)
        .collect::<Vec<_>>();

    let name = ast.ident;
    let fn_runner = &attrs[0][0];
    let result_type = find_field(ast.data, "output")
        .and_then(extract_type)
        .unwrap();
    let fn_generator = &attrs[0].get(1);

    // Build the output, possibly using quasi-quotation
    let expanded = if let Some(fn_generator) = fn_generator {
        quote! {
            impl Runner for #name {
                type Result = #result_type;

                fn gen(input: ArcStr) -> Self {
                    #name {
                        input: #fn_generator(input.as_ref()),
                        output: PhantomData,
                    }
                }

                fn run(&self) -> Self::Result {
                    #fn_runner(self.input.as_ref())
                }
            }
        }
    } else {
        quote! {
            impl Runner for #name {
                type Result = #result_type;

                fn gen(input: ArcStr) -> Self {
                    #name {
                        input,
                        output: PhantomData,
                    }
                }

                fn run(&self) -> Self::Result {
                    #fn_runner(self.input.as_ref())
                }
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

fn get_meta_items(attr: &syn::Attribute) -> Option<Vec<syn::NestedMeta>> {
    if attr.path.segments.len() == 1
        && (attr.path.segments[0].ident == "runner" || attr.path.segments[0].ident == "runner_type")
    {
        match attr.parse_meta() {
            Ok(Meta::List(ref meta)) => Some(meta.nested.iter().cloned().collect()),
            m => {
                // TODO: produce an error
                println!("FAILED TO INTERPRET : {:#?}", m);
                println!("{:#?}", attr);
                None
            }
        }
    } else {
        None
    }
}

fn find_field(data_struct: Data, field: &str) -> Option<Field> {
    if let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = data_struct
    {
        fields
            .named
            .iter()
            .find(|f| {
                if let Some(i) = &f.ident {
                    i == field
                } else {
                    false
                }
            }).cloned()
    } else {
        None
    }
}

fn extract_type(field: Field) -> Option<Type> {
    if let Type::Path(ty) = field.ty {
        let seg = &ty.path.segments[0];

        assert_eq!(seg.ident, "PhantomData");

        if let PathArguments::AngleBracketed(arg) = &seg.arguments {
            if let GenericArgument::Type(ty) = &arg.args[0] {
                Some(ty.clone())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

#[proc_macro_attribute]
pub fn aoc(args: TokenStream, input: TokenStream) -> TokenStream {
    let (day, part) = utils::extract_meta(args);
    let part = part.expect("No part");
    let input = parse_macro_input!(input as ItemFn);

    let expanded = quote! {
        #[aoc_step2(#day, #part)]
        #input
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn aoc_step2(args: TokenStream, input: TokenStream) -> TokenStream {
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

    let part_impl = AOC_RUNNER.with(|map| {
        let mut map = map.borrow_mut();
        let runner = map.entry(day).or_default();

        runner.with_solver(Solver::new(name.clone(), out_t.clone()), part)
    });

    let expanded = if let Some(generator) = part_impl.generator {
        let gen_out_t = generator.out_t;
        let gen_name = generator.name;
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
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn generator(args: TokenStream, input: TokenStream) -> TokenStream {
    let (day, part) = utils::extract_meta(args);
    let day = day.to_string().parse().unwrap();
    let part = part.and_then(|p| p.to_string().parse().ok());

    let input = parse_macro_input!(input as ItemFn);

    let input_cloned = input.clone();

    let name = input.ident;
    let decl = input.decl;
    let out_t = if let ReturnType::Type(_, p) = decl.output {
        p
    } else {
        panic!()
    };

    AOC_RUNNER.with(|map| {
        let mut map = map.borrow_mut();
        let runner = map.entry(day).or_default();

        runner.with_generator(Generator::new(name.clone(), out_t.clone()), part);
    });

    let expanded = quote! {
        #input_cloned
    };

    TokenStream::from(expanded)
}
