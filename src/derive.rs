use proc_macro as pm;
use quote::quote;
use std::error;
use syn;
use types::SpecialType;

#[derive(Debug)]
struct AocMeta {
    runner: syn::Ident,
    generator: Option<syn::Ident>,
    gen_special: Option<SpecialType>,
    run_special: Option<SpecialType>,
}

impl AocMeta {
    fn build(cmds: impl Iterator<Item = MetaCommand>) -> Result<AocMeta, Box<error::Error>> {
        #[derive(Default)]
        struct Builder {
            runner: Option<syn::Ident>,
            generator: Option<syn::Ident>,
            gen_special: Option<SpecialType>,
            run_special: Option<SpecialType>,
        }

        let mut builder = Builder::default();

        for cmd in cmds {
            match cmd {
                MetaCommand::Runner(i) => if builder.runner.is_some() {
                    return Err(From::from("duplicate command: runner"));
                } else {
                    builder.runner = Some(i);
                },
                MetaCommand::Generator(i) => if builder.generator.is_some() {
                    return Err(From::from("duplicate command: generator"));
                } else {
                    builder.generator = Some(i);
                },
                MetaCommand::GenSpecial(t) => if builder.gen_special.is_some() {
                    return Err(From::from("duplicate command: gen_*"));
                } else {
                    builder.gen_special = Some(t);
                },
                MetaCommand::RunSpecial(t) => if builder.run_special.is_some() {
                    return Err(From::from("duplicate command: run_*"));
                } else {
                    builder.run_special = Some(t);
                },
            }
        }

        if let Some(runner) = builder.runner {
            Ok(AocMeta {
                runner,
                generator: builder.generator,
                gen_special: builder.gen_special,
                run_special: builder.run_special,
            })
        } else {
            Err(From::from("no runner provided"))
        }
    }
}

#[derive(Debug)]
enum MetaCommand {
    Runner(syn::Ident),
    Generator(syn::Ident),
    GenSpecial(SpecialType),
    RunSpecial(SpecialType),
}

impl MetaCommand {
    fn from_attr(attr: &syn::Attribute) -> Option<Vec<MetaCommand>> {
        use syn::*;

        let ident = &attr.path.segments.first()?.value().ident;

        if ident == "runner" {
            let meta = attr.parse_meta().ok()?;

            if let Meta::List(MetaList { nested, .. }) = &meta {
                Some(
                    nested
                        .iter()
                        .filter_map(|n| {
                            if let NestedMeta::Meta(Meta::Word(i)) = n {
                                Some(i)
                            } else {
                                None
                            }
                        }).enumerate()
                        .map(|(i, id)| match i {
                            0 => MetaCommand::Runner(id.clone()),
                            1 => MetaCommand::Generator(id.clone()),
                            _ => unreachable!(),
                        }).collect(),
                )
            } else {
                None
            }
        } else if ident == "run_result" {
            Some(vec![MetaCommand::RunSpecial(SpecialType::Result)])
        } else if ident == "run_option" {
            Some(vec![MetaCommand::RunSpecial(SpecialType::Option)])
        } else if ident == "gen_result" {
            Some(vec![MetaCommand::GenSpecial(SpecialType::Result)])
        } else if ident == "gen_option" {
            Some(vec![MetaCommand::GenSpecial(SpecialType::Option)])
        } else {
            None
        }
    }
}

pub fn aoc_runner_derive_impl(input: pm::TokenStream) -> pm::TokenStream {
    use syn::*;

    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    let aoc_meta = parse_meta(&ast.attrs).expect("failed to parse metas");

    let name = ast.ident;
    let fn_runner = aoc_meta.runner;
    let fn_generator = aoc_meta.generator;

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

    let gen = if let Some(t) = aoc_meta.gen_special {
        let input = match t {
            SpecialType::Result => quote! { #input? },
            SpecialType::Option => quote! { #input.ok_or("generator produce no value")? },
        };

        quote! {
            fn gen(input: ArcStr) -> Self {
                Self::try_gen(input).expect("failed to generate input")
            }

            fn try_gen(input: ArcStr) -> Result<Self, Box<dyn Error>> {
                Ok( #name {
                    #input,
                    output: PhantomData,
                } )
            }
        }
    } else {
        quote! {
            fn gen(input: ArcStr) -> Self {
                #name {
                    #input,
                    output: PhantomData,
                }
            }
        }
    };

    let run = if let Some(t) = aoc_meta.run_special {
        let runner = match t {
            SpecialType::Result => quote! { #fn_runner(self.input.as_ref())? },
            SpecialType::Option => {
                quote! { #fn_runner(self.input.as_ref()).ok_or("runner produce no value")? }
            }
        };

        quote! {
            fn run(&self) -> Box<dyn Display> {
                self.try_run().expect("failed to run")
            }

            fn try_run(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
                Ok( Box::new( #runner ) )
            }
        }
    } else {
        quote! {
            fn run(&self) -> Box<dyn Display> {
                Box::new( #fn_runner(self.input.as_ref()) )
            }
        }
    };

    // Hand the output tokens back to the compiler
    pm::TokenStream::from(quote! {
        impl Runner for #name {
            #gen

            #run
        }
    })
}

fn parse_meta(attrs: &[syn::Attribute]) -> Result<AocMeta, Box<dyn error::Error>> {
    AocMeta::build(attrs.iter().filter_map(MetaCommand::from_attr).flatten())
}
