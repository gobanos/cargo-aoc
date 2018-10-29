use proc_macro::{TokenStream, TokenTree};
use syn;

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
