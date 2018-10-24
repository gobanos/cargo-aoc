extern crate proc_macro;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::Data;
use syn::DataStruct;
use syn::Field;
use syn::Fields;
use syn::GenericArgument;
use syn::ItemFn;
use syn::PathArguments;
use syn::Type;
use syn::{DeriveInput, Meta};
use syn::FnArg;
use syn::ReturnType;

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

#[cfg(feature = "nightly")]
#[proc_macro_attribute]
pub fn aoc(args: TokenStream, input: TokenStream) -> TokenStream {
    let _metas = attrs::extract_meta(args);

    let input = parse_macro_input!(input as ItemFn);

    let name = input.ident;
    let decl = input.decl;
    let arg_decl = if let FnArg::Captured(a) = decl.inputs[0].clone() { a } else { panic!() };
    let arg = arg_decl.pat;
    let arg_t = arg_decl.ty;
    let out_t = if let ReturnType::Type(_, p) = decl.output { p } else { panic!() };
    let body = input.block;

    println!("{:?}", out_t);

    let expanded = quote! {
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
    };

    TokenStream::from(expanded)
}

#[cfg(feature = "nightly")]
#[proc_macro_attribute]
pub fn generator(args: TokenStream, input: TokenStream) -> TokenStream {
    println!("ARGS: {:#?}", args);
    println!("INPUT: {:#?}", input);

    input
}

#[cfg(feature = "nightly")]
mod attrs {
    use proc_macro::{Ident, TokenStream, TokenTree};

    pub fn extract_meta(args: TokenStream) -> (Ident, Ident) {
        let mut idents = args.into_iter().filter_map(|a| {
            if let TokenTree::Ident(i) = a {
                Some(i)
            } else {
                None
            }
        });

        (idents.next().unwrap(), idents.next().unwrap())
    }
}
