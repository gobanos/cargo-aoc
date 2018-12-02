use proc_macro as pm;
use quote::quote;
use std::error;
use syn;

#[derive(Debug)]
struct AocMeta {
    runner: syn::Ident,
    generator: Option<syn::Ident>,
    gen_result: bool,
    run_result: bool,
}

impl AocMeta {
    fn build(cmds: impl Iterator<Item = MetaCommand>) -> Result<AocMeta, Box<error::Error>> {
        #[derive(Default)]
        struct Builder {
            runner: Option<syn::Ident>,
            generator: Option<syn::Ident>,
            gen_result: Option<bool>,
            run_result: Option<bool>,
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
                MetaCommand::GenResult => if builder.gen_result.is_some() {
                    return Err(From::from("duplicate command: gen_result"));
                } else {
                    builder.gen_result = Some(true);
                },
                MetaCommand::RunResult => if builder.run_result.is_some() {
                    return Err(From::from("duplicate command: run_result"));
                } else {
                    builder.run_result = Some(true);
                },
            }
        }

        if let Some(runner) = builder.runner {
            Ok(AocMeta {
                runner,
                generator: builder.generator,
                gen_result: builder.gen_result.unwrap_or_default(),
                run_result: builder.run_result.unwrap_or_default(),
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
    GenResult,
    RunResult,
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
            Some(vec![MetaCommand::RunResult])
        } else if ident == "gen_result" {
            Some(vec![MetaCommand::GenResult])
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

    let gen = if aoc_meta.gen_result {
        quote! {
            fn gen(input: ArcStr) -> Self {
                Self::try_gen(input).expect("failed to generate input")
            }

            fn try_gen(input: ArcStr) -> Result<Self, Box<dyn Error>> {
                Ok( #name {
                    #input?,
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

    let run = if aoc_meta.run_result {
        quote! {
            fn run(&self) -> Box<dyn Display> {
                self.try_run().expect("failed to run")
            }

            fn try_run(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
                Ok( Box::new( #fn_runner(self.input.as_ref()) ? ) )
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
