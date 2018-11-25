use aoc_runner_internal::Day;
use aoc_runner_internal::Part;
use proc_macro as pm;
use syn;

pub(crate) fn extract_meta(args: pm::TokenStream) -> (syn::Ident, Option<syn::Ident>) {
    let mut idents = args.into_iter().filter_map(|a| {
        if let pm::TokenTree::Ident(_) = a {
            Some(a.into())
        } else {
            None
        }
    });

    let day: pm::TokenStream = idents.next().expect("Couldn't find day");
    let day: syn::Ident = syn::parse(day).unwrap();

    let part = idents.next().and_then(|i| syn::parse(i).ok());

    (day, part)
}

pub(crate) fn to_snakecase(day: Day, part: Part) -> syn::Ident {
    let name = format!("day{}_part{}", day.0, part.0);

    syn::Ident::new(&name, pm::Span::call_site().into())
}

pub(crate) fn to_camelcase(day: Day, part: Part) -> syn::Ident {
    let name = format!("Day{}Part{}", day.0, part.0);

    syn::Ident::new(&name, pm::Span::call_site().into())
}
