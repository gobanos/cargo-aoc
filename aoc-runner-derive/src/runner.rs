use crate::types::{Generator, Solver, SpecialType};
use crate::utils::{self, extract_result, to_camelcase, to_snakecase};
use aoc_runner_internal::{Alternative, DayPart};
use proc_macro as pm;
use proc_macro2 as pm2;
use proc_macro2::Span;
use quote::quote;
use syn::*;

pub fn runner_impl(args: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    let (day, part, alt, name) = utils::extract_meta(args);
    let day = day
        .to_string()
        .parse()
        .expect("runners must have a defined day");
    let part = part
        .expect("runners must have a defined part")
        .to_string()
        .parse()
        .expect("runners must have a defined part");
    let alt = alt
        .map(|i| {
            i.to_string()
                .parse()
                .expect("couldn't parse runner alternative")
        })
        .unwrap_or(Alternative::Default);
    let name = name.map(|i| i.to_string());

    let dp = DayPart {
        day,
        part,
        alt,
        name: name.as_ref().map(String::as_str),
    };

    let input = parse_macro_input!(input as ItemFn);

    let input_t = match input
        .sig
        .inputs
        .first()
        .expect("runners must take an input")
    {
        FnArg::Typed(pat) => pat.ty.clone(),
        _ => panic!("runners functions can't have a self parameter"),
    };

    let input_t = match &*input_t {
        Type::Reference(ty) => {
            let ty = ty.clone();
            Box::new(Type::Reference(TypeReference {
                lifetime: Some(Lifetime::new("'a", Span::call_site())),
                ..ty
            }))
        }
        _ => input_t,
    };

    let original_fn = input.clone();

    let fn_name = input.sig.ident;
    let out_t = if let ReturnType::Type(_, p) = input.sig.output {
        p
    } else {
        panic!("runners must return a value")
    };

    let (special_type, out_t) = if let Some((ty, inner)) = extract_result(&*out_t) {
        (Some(ty), Box::new(inner))
    } else {
        (None, out_t)
    };

    let mod_name = Ident::new(&format!("__{}_runner", fn_name), Span::call_site());
    let struct_name = to_camelcase(dp, "Runner");

    let runner_body = match special_type {
        Some(SpecialType::Result) => quote! { #fn_name(input).map_err(|err| err.into()) },
        Some(SpecialType::Option) => {
            quote! { #fn_name(input).ok_or_else(|| aoc_runner::GeneratorFailed.into()) }
        }
        None => quote! { Ok(#fn_name(input)) },
    };

    pm::TokenStream::from(quote! {
        #original_fn

        #[doc(hidden)]
        mod #mod_name {
            use super::*;
            use crate::__aoc::#struct_name;
            use aoc_runner::Runner;

            impl<'a> Runner<'a, #input_t> for #struct_name<#input_t> {
                type Output = #out_t;

                fn run(&self, input: #input_t) -> std::result::Result<Self::Output, Box<dyn std::error::Error>> {
                    #runner_body
                }
            }
        }
    })
}

//fn build_derive(solver: &Solver, generator: Option<&Generator>) -> pm2::TokenStream {
//    let fn_runner = solver.get_name();
//
//    // Build the output, possibly using quasi-quotation
//    let input = if let Some(generator) = generator {
//        let fn_generator = generator.get_name();
//        quote! {
//            input: #fn_generator(input.borrow())
//        }
//    } else {
//        quote! {
//            input
//        }
//    };
//
//    let gen = if let Some(t) = generator.and_then(|g| g.special_type) {
//        let input = match t {
//            SpecialType::Result => quote! { #input? },
//            SpecialType::Option => quote! { #input.ok_or("generator produce no value")? },
//        };
//
//        quote! {
//            fn gen(input: ArcStr) -> Self {
//                Self::try_gen(input).expect("failed to generate input")
//            }
//
//            fn try_gen(input: ArcStr) -> Result<Self, Box<dyn Error>> {
//                Ok( RunnerStruct {
//                    #input,
//                    output: PhantomData,
//                } )
//            }
//        }
//    } else {
//        quote! {
//            fn gen(input: ArcStr) -> Self {
//                RunnerStruct {
//                    #input,
//                    output: PhantomData,
//                }
//            }
//        }
//    };
//
//    let run = if let Some(t) = solver.special_type {
//        let runner = match t {
//            SpecialType::Result => quote! { #fn_runner(self.input.borrow())? },
//            SpecialType::Option => {
//                quote! { #fn_runner(self.input.borrow()).ok_or("runner produce no value")? }
//            }
//        };
//
//        quote! {
//            fn run(&self) -> Box<dyn Display> {
//                self.try_run().expect("failed to run")
//            }
//
//            fn try_run(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
//                Ok( Box::new( #runner ) )
//            }
//
//            fn bench(&self, black_box: fn(&dyn Display)) {
//                black_box( &#fn_runner(self.input.borrow()).unwrap() )
//            }
//        }
//    } else {
//        quote! {
//            fn run(&self) -> Box<dyn Display> {
//                Box::new( #fn_runner(self.input.borrow()) )
//            }
//
//            fn bench(&self, black_box: fn(&dyn Display)) {
//                black_box( &#fn_runner(self.input.borrow()) )
//            }
//        }
//    };
//
//    quote! {
//        impl Runner for RunnerStruct {
//            #gen
//
//            #run
//        }
//    }
//}
