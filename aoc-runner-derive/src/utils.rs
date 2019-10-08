use aoc_runner_internal::{Day, DayPart};
use proc_macro as pm;
use syn;
use types::SpecialType;

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

pub(crate) fn extract_result(ty: &syn::Type) -> Option<(SpecialType, syn::Type)> {
    use syn::*;

    if let Type::Path(TypePath {
        path: Path { segments: s, .. },
        ..
    }) = ty
    {
        if let Some(p) = s.last() {
            let ps = p.value();

            if ps.ident == "Result" {
                if let PathArguments::AngleBracketed(a) = &ps.arguments {
                    if let Some(arg) = a.args.first() {
                        if let GenericArgument::Type(t) = arg.value() {
                            return Some((SpecialType::Result, t.clone()));
                        }
                    }
                }
            } else if ps.ident == "Option" {
                if let PathArguments::AngleBracketed(a) = &ps.arguments {
                    if let Some(arg) = a.args.first() {
                        if let GenericArgument::Type(t) = arg.value() {
                            return Some((SpecialType::Option, t.clone()));
                        }
                    }
                }
            }
        }
    }

    None
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

pub(crate) fn to_input(d: Day) -> syn::Ident {
    syn::Ident::new(&format!("input_day{}", d.0), pm::Span::call_site().into())
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
