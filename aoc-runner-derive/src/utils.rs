use crate::types::SpecialType;
use aoc_runner_internal::{Day, DayPart};
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

pub(crate) fn extract_result(ty: &syn::Type) -> Option<(SpecialType, syn::Type)> {
    use syn::*;

    if let Type::Path(TypePath {
        path: Path { segments: s, .. },
        ..
    }) = ty
    {
        if let Some(p) = s.last() {
            if p.ident == "Result" {
                if let PathArguments::AngleBracketed(a) = &p.arguments {
                    if let Some(arg) = a.args.first() {
                        if let GenericArgument::Type(t) = arg {
                            return Some((SpecialType::Result, t.clone()));
                        }
                    }
                }
            } else if p.ident == "Option" {
                if let PathArguments::AngleBracketed(a) = &p.arguments {
                    if let Some(arg) = a.args.first() {
                        if let GenericArgument::Type(t) = arg {
                            return Some((SpecialType::Option, t.clone()));
                        }
                    }
                }
            }
        }
    }

    None
}

pub(crate) fn to_snakecase(dp: DayPart) -> syn::Ident {
    let DayPart { day, part, name, alt } = dp;
    let name = if let Some(name) = name {
        format!("day{}_part{}_{}", day.as_u8(), part.as_u8(), name.to_lowercase())
    } else {
        format!("day{}_part{}", day.as_u8(), part.as_u8())
    };

    syn::Ident::new(&name, pm::Span::call_site().into())
}

pub(crate) fn to_camelcase(dp: DayPart, suffix: &str) -> syn::Ident {
    let DayPart { day, part, alt, .. } = dp;

    let name = if let Some(alt) = alt {
        format!(
            "Day{}Part{}Alt{}{}",
            day.as_u8(),
            part.as_u8(),
            alt.as_u8(),
            suffix
        )
    } else {
        format!("Day{}Part{}{}", day.as_u8(), part.as_u8(), suffix)
    };

    syn::Ident::new(&name, pm::Span::call_site().into())
}

pub(crate) fn to_input(d: Day) -> syn::Ident {
    syn::Ident::new(&format!("input_{}", d), pm::Span::call_site().into())
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
