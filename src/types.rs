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
    pub name: syn::Ident,
    pub out_t: Box<syn::Type>,
}

impl Generator {
    pub fn new(name: syn::Ident, out_t: Box<syn::Type>) -> Generator {
        Generator { name, out_t }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Solver {
    pub name: syn::Ident,
    pub out_t: Box<syn::Type>,
}

impl Solver {
    pub fn new(name: syn::Ident, out_t: Box<syn::Type>) -> Solver {
        Solver { name, out_t }
    }
}
