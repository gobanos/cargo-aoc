use std::str::FromStr;
use syn;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub(crate) enum Day {
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    D10,
    D11,
    D12,
    D13,
    D14,
    D15,
    D16,
    D17,
    D18,
    D19,
    D20,
    D21,
    D22,
    D23,
    D24,
    D25,
}

impl FromStr for Day {
    type Err = String;

    fn from_str(day: &str) -> Result<Self, Self::Err> {
        Ok(match day {
            "day1" => Day::D1,
            "day2" => Day::D2,
            "day3" => Day::D3,
            "day4" => Day::D4,
            "day5" => Day::D5,
            "day6" => Day::D6,
            "day7" => Day::D7,
            "day8" => Day::D8,
            "day9" => Day::D9,
            "day10" => Day::D10,
            "day11" => Day::D11,
            "day12" => Day::D12,
            "day13" => Day::D13,
            "day14" => Day::D14,
            "day15" => Day::D15,
            "day16" => Day::D16,
            "day17" => Day::D17,
            "day18" => Day::D18,
            "day19" => Day::D19,
            "day20" => Day::D20,
            "day21" => Day::D21,
            "day22" => Day::D22,
            "day23" => Day::D23,
            "day24" => Day::D24,
            "day25" => Day::D25,
            _ => return Err(format!("Failed to parse day: {}", day)),
        })
    }
}

impl Into<&'static str> for Day {
    fn into(self) -> &'static str {
        match self {
            Day::D1 => "day1",
            Day::D2 => "day2",
            Day::D3 => "day3",
            Day::D4 => "day4",
            Day::D5 => "day5",
            Day::D6 => "day6",
            Day::D7 => "day7",
            Day::D8 => "day8",
            Day::D9 => "day9",
            Day::D10 => "day10",
            Day::D11 => "day11",
            Day::D12 => "day12",
            Day::D13 => "day13",
            Day::D14 => "day14",
            Day::D15 => "day15",
            Day::D16 => "day16",
            Day::D17 => "day17",
            Day::D18 => "day18",
            Day::D19 => "day19",
            Day::D20 => "day20",
            Day::D21 => "day21",
            Day::D22 => "day22",
            Day::D23 => "day23",
            Day::D24 => "day24",
            Day::D25 => "day25",
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub(crate) enum Part {
    Part1,
    Part2,
}

impl Into<&'static str> for Part {
    fn into(self) -> &'static str {
        match self {
            Part::Part1 => "part1",
            Part::Part2 => "part2",
        }
    }
}

impl FromStr for Part {
    type Err = String;

    fn from_str(part: &str) -> Result<Self, Self::Err> {
        Ok(match part {
            "part1" => Part::Part1,
            "part2" => Part::Part2,
            _ => return Err(format!("Failed to parse part: {}", part)),
        })
    }
}

#[derive(Default)]
pub(crate) struct Runner {
    part1: Option<PartBuilder>,
    part2: Option<PartBuilder>,
}

impl Runner {
    pub fn with_generator(&mut self, generator: Generator, part: Option<Part>) {
        match part {
            Some(Part::Part1) => self.part1 = Some(PartBuilder::WithGenerator(generator)),
            Some(Part::Part2) => self.part2 = Some(PartBuilder::WithGenerator(generator)),
            None => {
                self.part1 = Some(PartBuilder::WithGenerator(generator.clone()));
                self.part2 = Some(PartBuilder::WithGenerator(generator));
            }
        }
    }

    pub fn with_solver(&mut self, solver: Solver, part: Part) -> PartImpl {
        match part {
            Part::Part1 => PartBuilder::complete(&mut self.part1, solver),
            Part::Part2 => PartBuilder::complete(&mut self.part2, solver),
        }
    }
}

pub(crate) enum PartBuilder {
    WithGenerator(Generator),
    Complete(PartImpl),
}

impl PartBuilder {
    fn complete(builder: &mut Option<PartBuilder>, solver: Solver) -> PartImpl {
        *builder = match builder.take() {
            Some(PartBuilder::WithGenerator(g)) => Some(PartBuilder::Complete(PartImpl {
                generator: Some(g),
                solver,
            })),
            None => Some(PartBuilder::Complete(PartImpl {
                generator: None,
                solver,
            })),
            _ => unreachable!("ALREADY COMPLETED PART"),
        };

        if let Some(PartBuilder::Complete(part)) = builder {
            part.clone()
        } else {
            unreachable!("INCOMPLETE PART")
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct PartImpl {
    pub generator: Option<Generator>,
    pub solver: Solver,
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
