use proc_macro as pm;
use quote::quote;
use syn::*;

pub fn aoc_runner_derive_impl(input: pm::TokenStream) -> pm::TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    let attrs: Vec<_> = ast.attrs.iter().filter_map(get_meta_items).collect();

    let name = ast.ident;
    let fn_runner = &attrs[0][0];
    let fn_generator = &attrs[0].get(1);

    // Build the output, possibly using quasi-quotation
    let input = if let Some(fn_generator) = fn_generator {
        quote! {
            input: #fn_generator(input.as_ref())
        }
    } else {
        quote! {
            input
        }
    };

    // Hand the output tokens back to the compiler
    pm::TokenStream::from(quote! {
        impl Runner for #name {
            fn gen(input: ArcStr) -> Self {
                #name {
                    #input,
                    output: PhantomData,
                }
            }

            fn run(&self) -> Box<dyn std::fmt::Display> {
                Box::new( #fn_runner(self.input.as_ref()) )
            }
        }
    })
}

fn get_meta_items(attr: &syn::Attribute) -> Option<Vec<syn::NestedMeta>> {
    if attr.path.segments.len() == 1
        && (attr.path.segments[0].ident == "runner" || attr.path.segments[0].ident == "runner_type")
    {
        match attr.parse_meta() {
            Ok(Meta::List(ref meta)) => Some(meta.nested.iter().cloned().collect()),
            m => {
                // TODO: produce an error
                println!("FAILED TO INTERPRET : {:#?}", m);
                println!("{:#?}", attr);
                None
            }
        }
    } else {
        None
    }
}
