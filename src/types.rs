use std::str::FromStr;
use syn;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub(crate) struct Day(pub u8);

impl FromStr for Day {
    type Err = String;

    fn from_str(day: &str) -> Result<Self, Self::Err> {
        if day.len() < 4 || &day[..3] != "day" {
            return Err(format!("Failed to parse day: {}", day));
        }

        day[3..]
            .parse()
            .map_err(|e| format!("Failed to parse {}: {:?}", day, e))
            .and_then(|d| {
                if d == 0 || d > 25 {
                    Err(format!("day {} is not between 0 and 25", d))
                } else {
                    Ok(Day(d))
                }
            })
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub(crate) struct Part(pub u8);

impl FromStr for Part {
    type Err = String;

    fn from_str(part: &str) -> Result<Self, Self::Err> {
        Ok(match part {
            "part1" => Part(1),
            "part2" => Part(2),
            _ => return Err(format!("Failed to parse part: {}", part)),
        })
    }
}

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
