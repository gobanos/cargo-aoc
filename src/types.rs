use proc_macro as pm;
use proc_macro2 as pm2;
use quote::quote;
use syn;

#[derive(Clone, Debug, Default)]
pub(crate) struct Runner {
    pub generator: Option<Generator>,
    pub solver: Option<Solver>,
}

impl Runner {
    pub fn with_generator(&mut self, generator: Generator) {
        if self.solver.is_some() {
            panic!("Generators must be defined before solutions: {:?}", self);
        }
        if self.generator.is_some() {
            panic!("A generator is already defined: {:?}", self);
        }
        self.generator = Some(generator)
    }

    pub fn with_solver(&mut self, solver: Solver) {
        if self.solver.is_some() {
            panic!("A solution is already defined: {:?}", self);
        }
        self.solver = Some(solver)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Generator {
    name: String,
    out_t: String,
    pub is_result: bool,
}

impl Generator {
    pub fn new(name: &syn::Ident, out_t: &syn::Type, is_result: bool) -> Generator {
        Generator {
            name: name.to_string(),
            out_t: quote! { #out_t }.to_string(),
            is_result,
        }
    }

    pub fn get_name(&self) -> syn::Ident {
        syn::Ident::new(&self.name, pm::Span::call_site().into())
    }

    pub fn get_out_t(&self) -> pm2::TokenStream {
        self.out_t.parse().expect("failed to parse generator type")
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Solver {
    name: String,
    out_t: String,
}

impl Solver {
    pub fn new(name: &syn::Ident, out_t: &syn::Type) -> Solver {
        Solver {
            name: name.to_string(),
            out_t: quote! { #out_t }.to_string(),
        }
    }
}
