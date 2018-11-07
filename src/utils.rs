use proc_macro::{TokenStream, TokenTree, Span};
use syn;
use types::{
    Day,
    Part
};

pub(crate) fn extract_meta(args: TokenStream) -> (syn::Ident, Option<syn::Ident>) {
    let mut idents = args.into_iter().filter_map(|a| {
        if let TokenTree::Ident(_) = a {
            Some(a.into())
        } else {
            None
        }
    });

    let day: TokenStream = idents.next().expect("Couldn't find day");
    let day: syn::Ident = syn::parse(day).unwrap();

    let part = idents.next().and_then(|i| syn::parse(i).ok());

    (day, part)
}

pub(crate) fn to_snakecase(day: Day, part: Part) -> syn::Ident {
    let name = format!("day{}_part{}", day.0, part.0);

    syn::Ident::new(&name, Span::call_site().into())
}

pub(crate) fn to_camelcase(day: Day, part: Part) -> syn::Ident {
    let name = format!("Day{}Part{}", day.0, part.0);

    syn::Ident::new(&name, Span::call_site().into())
}