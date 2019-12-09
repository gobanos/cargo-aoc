use crate::types::{Generator, SpecialType};
use crate::utils;
use crate::utils::{to_camelcase, to_snakecase};
use aoc_runner_internal::{Alternative, DayPart, Part};
use proc_macro as pm;
use proc_macro2 as pm2;
use proc_macro2::Span;
use quote::quote;
use std::convert::TryInto;
use syn::*;

pub fn generator_impl(args: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    let (day, part, alt, name) = utils::extract_meta(args);
    let day = day
        .to_string()
        .parse()
        .expect("generators must have defined day");
    let part = part.and_then(|p| p.to_string().parse().ok());
    let alt = alt.and_then(|p| p.to_string().parse().ok());
    let name = name.map(|i| i.to_string());

    if let Some(name) = &name {
        use std::io::Write;
        let mut aoc_file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("target/aoc.txt")
            .expect("could not open aoc.txt");

        writeln!(aoc_file, "{}", name).expect("failed to write to aoc.txt");
    }

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

    let gen_impl = |part: Part, alt: Alternative| {
        let generator_struct = to_camelcase(
            DayPart {
                day,
                part,
                alt,
                name: name.as_ref().map(String::as_str),
            },
            "Generator",
        );
        quote! {
            impl<'a> aoc_runner::Generator<'a> for crate::__aoc::#generator_struct {
                type Output = #out_t;

                fn generate(&self, input: &'a str) -> std::result::Result<Self::Output, Box<dyn std::error::Error>> {
                    #generator_body
                }
            }
        }
    };

    let impls = match (part, alt) {
        (Some(part), Some(alt)) => gen_impl(part, alt),
        (Some(part), None) => (0..=4)
            .into_iter()
            .map(|alt| gen_impl(part, alt.try_into().unwrap()))
            .collect(),
        (None, Some(alt)) => (1..=2)
            .into_iter()
            .map(|part| gen_impl(part.try_into().unwrap(), alt))
            .collect(),
        (None, None) => (1..=2)
            .into_iter()
            .flat_map(|part| (0..=4).into_iter().map(move |alt| (part, alt)))
            .map(|(part, alt)| gen_impl(part.try_into().unwrap(), alt.try_into().unwrap()))
            .collect(),
    };

    (quote! {
        #original_fn

        #[doc(hidden)]
        pub mod #mod_name {
            use super::*;

            #impls
        }
    })
    .into()
}
