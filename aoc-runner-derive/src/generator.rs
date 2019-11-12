use crate::types::{Generator, SpecialType};
use crate::utils;
use crate::utils::{to_camelcase, to_snakecase};
use crate::{AocGeneratorArgs, AOC_RUNNER};
use aoc_runner_internal::{DayPart, Part};
use darling::FromMeta;
use proc_macro as pm;
use proc_macro2::Span;
use quote::quote;
use syn::*;

pub fn generator_impl(args: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as ItemFn);
    let original_fn = input.clone();
    let args = match AocGeneratorArgs::from_list(&attr_args) {
        Ok(value) => value,
        Err(e) => return pm::TokenStream::from(e.write_errors()),
    };

    let generator_name = &input.sig.ident;
    let out_t = if let ReturnType::Type(_, p) = input.sig.output {
        p
    } else {
        panic!("Cannot find output type for {}", generator_name)
    };

    let (special_type, out_t) = if let Some((ty, inner)) = utils::extract_result(&*out_t) {
        (Some(ty), Box::new(inner))
    } else {
        (None, out_t)
    };

    let out_t = match &*out_t {
        Type::Reference(ty) => {
            let ty = ty.clone();
            Box::new(Type::Reference(TypeReference {
                lifetime: Some(Lifetime::new("'a", Span::call_site())),
                ..ty
            }))
        }
        _ => out_t,
    };

    let mod_name = Ident::new(
        &format!("__{}_aoc_generator", generator_name),
        Span::call_site(),
    );

    let generator_body = match special_type {
        Some(SpecialType::Result) => quote! { #generator_name(input).map_err(|err| err.into()) },
        Some(SpecialType::Option) => {
            quote! { #generator_name(input).ok_or_else(|| aoc_runner::GeneratorFailed.into()) }
        }
        None => quote! { Ok(#generator_name(input)) },
    };

    let gen_impl = |part: Part| {
        let generator_struct = to_camelcase(
            &DayPart {
                day: args.clone().day.into(),
                part,
                name: None,
            },
            "Generator",
        );
        quote! {
            impl<'a> aoc_runner::Generator<'a> for crate::__aoc::#generator_struct {
                type Output = #out_t;

                fn generate(&self, input: &'a str) -> Result<Self::Output, Box<dyn Error>> {
                    #generator_body
                }
            }
        }
    };

    let impls = if let Some(p) = args.clone().part {
        gen_impl(p.into())
    } else {
        let i1 = gen_impl(Part(1));
        let i2 = gen_impl(Part(2));
        quote! {
            #i1
            #i2
        }
    };

    (quote! {
        #original_fn

        #[doc(hidden)]
        pub mod #mod_name {
            use super::*;
            use std::error::Error;

            #impls
        }
    })
    .into()
}
