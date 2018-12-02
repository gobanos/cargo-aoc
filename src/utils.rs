use aoc_runner_internal::DayPart;
use proc_macro as pm;
use syn;

pub(crate) fn extract_meta(
    args: pm::TokenStream,
) -> (syn::Ident, Option<syn::Ident>, Option<syn::Ident>) {
    let mut idents = args.into_iter().filter_map(|a| {
        if let pm::TokenTree::Ident(_) = a {
            Some(a.into())
        } else {
            None
        }
    });

    let day: pm::TokenStream = idents.next().expect("Couldn't find day");
    let day: syn::Ident = syn::parse(day).expect("failed to parse day");

    let part = idents.next().and_then(|i| syn::parse(i).ok());
    let name = idents.next().and_then(|i| syn::parse(i).ok());

    (day, part, name)
}

pub(crate) fn to_snakecase(dp: &DayPart) -> syn::Ident {
    let DayPart { day, part, name } = dp;
    let name = if let Some(name) = name {
        format!("day{}_part{}_{}", day.0, part.0, name.to_lowercase())
    } else {
        format!("day{}_part{}", day.0, part.0)
    };

    syn::Ident::new(&name, pm::Span::call_site().into())
}

pub(crate) fn to_camelcase(dp: &DayPart) -> syn::Ident {
    let DayPart { day, part, name } = dp;

    let name = if let Some(name) = name {
        format!("Day{}Part{}{}", day.0, part.0, name.to_uppercase())
    } else {
        format!("Day{}Part{}", day.0, part.0)
    };

    syn::Ident::new(&name, pm::Span::call_site().into())
}

pub(crate) fn is_rls() -> bool {
    use std::env;
    use std::path;

    let p = match env::var("CARGO") {
        Ok(p) => p,
        Err(_) => return false,
    };

    path::Path::new(&p)
        .file_stem()
        .map(|file| file == "rls")
        .unwrap_or(false)
}
