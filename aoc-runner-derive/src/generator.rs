use crate::types::{Generator, SpecialType};
use crate::utils;
use crate::utils::{to_camelcase, to_snakecase};
use crate::AOC_RUNNER;
use aoc_runner_internal::{DayPart, Part};
use proc_macro as pm;
use proc_macro2 as pm2;
use proc_macro2::Span;
use quote::quote;
use syn::*;

pub fn generator_impl(args: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    let (day, part, name) = utils::extract_meta(args);
    let day = day
        .to_string()
        .parse()
        .expect("generators must have defined day");
    let part = part.and_then(|p| p.to_string().parse().ok());
    let name = name.map(|i| i.to_string());

    let input = parse_macro_input!(input as ItemFn);
    let original_fn = input.clone();

    let generator_name = &input.sig.ident;
    let out_t = if let ReturnType::Type(_, p) = input.sig.output {
        p
    } else {
        panic!("cannot find output type for {}", generator_name)
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
                day,
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

    let impls = if let Some(p) = part {
        gen_impl(p)
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
