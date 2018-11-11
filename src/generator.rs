use proc_macro as pm;
use quote::quote;
use syn::*;
use types::{Generator, Part};
use utils;
use AOC_RUNNER;

pub fn generator_impl(args: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
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
        let mut map = map.borrow_mut().unwrap();

        let mut register = |p: Part| {
            let runner = map.entry((day, p)).or_default();
            runner.with_generator(Generator::new(name.clone(), out_t.clone()));
        };

        if let Some(p) = part {
            register(p);
        } else {
            register(Part(1));
            register(Part(2));
        }
    });

    let expanded = quote! {
        #input_cloned
    };

    pm::TokenStream::from(expanded)
}
