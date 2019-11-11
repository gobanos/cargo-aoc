#[doc(hidden)]
pub mod __aoc {
    use aoc_runner::{Generator, NotImplemented, RunnerV2, Void};
    use std::error::Error;

    pub struct Day1Part1Generator;

    impl<'a> Generator<'a> for &Day1Part1Generator {
        type Output = &'a str;

        fn generate(&self, input: &'a str) -> Result<Self::Output, Box<dyn Error>> {
            Ok(input)
        }

        fn is_default(&self) -> bool {
            true
        }
    }

    pub struct Day1Part2Generator;

    impl<'a> Generator<'a> for &Day1Part2Generator {
        type Output = &'a str;

        fn generate(&self, input: &'a str) -> Result<Self::Output, Box<dyn Error>> {
            Ok(input)
        }

        fn is_default(&self) -> bool {
            true
        }
    }

    pub struct Day1Part1Runner;

    impl<'a> RunnerV2<'a> for &Day1Part1Runner {
        type Input = &'a str;
        type Output = Void;

        fn run(&self, _input: Self::Input) -> Result<Self::Output, Box<dyn Error>> {
            Err(Box::new(NotImplemented))
        }

        fn is_implemented(&self) -> bool {
            false
        }
    }

    pub struct Day1Part2Runner;

    impl<'a> RunnerV2<'a> for &Day1Part2Runner {
        type Input = &'a str;
        type Output = ();

        fn run(&self, _input: Self::Input) -> Result<Self::Output, Box<dyn Error>> {
            Err(Box::new(NotImplemented))
        }

        fn is_implemented(&self) -> bool {
            false
        }
    }
}

pub mod day1 {
    use std::collections::HashSet;
    use std::num::ParseIntError;

    fn parse_input_day1(input: &str) -> Result<Vec<i32>, ParseIntError> {
        input.lines().map(|l| l.parse()).collect()
    }

    #[doc(hidden)]
    pub mod __parse_input_day1_aoc_generator {
        use super::parse_input_day1;
        use std::error::Error;

        impl<'a> aoc_runner::Generator<'a> for crate::__aoc::Day1Part1Generator {
            type Output = Vec<i32>;

            fn generate(&self, input: &'a str) -> Result<Self::Output, Box<dyn Error>> {
                parse_input_day1(input).map_err(|err| err.into())
            }
        }

        impl<'a> aoc_runner::Generator<'a> for crate::__aoc::Day1Part2Generator {
            type Output = Vec<i32>;

            fn generate(&self, input: &'a str) -> Result<Self::Output, Box<dyn Error>> {
                parse_input_day1(input).map_err(|err| err.into())
            }
        }
    }

    fn parse_input_day1_unwrap(input: &str) -> Vec<i32> {
        input.lines().map(|l| l.parse().unwrap()).collect()
    }

    #[doc(hidden)]
    pub mod __parse_input_day1_unwrap_aoc_generator {
        use super::parse_input_day1_unwrap;
        use std::error::Error;
        use std::ops::Deref;

        pub struct Day1Part1UnwrapGenerator {
            base: crate::__aoc::Day1Part1Generator,
        }

        impl Default for Day1Part1UnwrapGenerator {
            fn default() -> Self {
                Day1Part1UnwrapGenerator {
                    base: crate::__aoc::Day1Part1Generator,
                }
            }
        }

        impl Deref for Day1Part1UnwrapGenerator {
            type Target = crate::__aoc::Day1Part1Generator;

            fn deref(&self) -> &Self::Target {
                &self.base
            }
        }

        impl<'a> aoc_runner::Generator<'a> for Day1Part1UnwrapGenerator {
            type Output = Vec<i32>;

            fn generate(&self, input: &'a str) -> Result<Self::Output, Box<dyn Error>> {
                Ok(parse_input_day1_unwrap(input))
            }
        }

        pub struct Day1Part2UnwrapGenerator {
            base: crate::__aoc::Day1Part2Generator,
        }

        impl Default for Day1Part2UnwrapGenerator {
            fn default() -> Self {
                Day1Part2UnwrapGenerator {
                    base: crate::__aoc::Day1Part2Generator,
                }
            }
        }

        impl Deref for Day1Part2UnwrapGenerator {
            type Target = crate::__aoc::Day1Part2Generator;

            fn deref(&self) -> &Self::Target {
                &self.base
            }
        }
    }

    pub fn part1(freqs: &[i32]) -> i32 {
        freqs.iter().sum()
    }

    #[doc(hidden)]
    pub mod __part1_aoc_runner {
        use super::part1;
        use crate::__aoc::Day1Part1Runner;
        use aoc_runner::RunnerV2;
        use std::error::Error;

        impl<'a> RunnerV2<'a> for Day1Part1Runner {
            type Input = &'a [i32];
            type Output = i32;

            fn run(&self, input: Self::Input) -> Result<Self::Output, Box<dyn Error>> {
                Ok(part1(input))
            }
        }
    }

    pub fn part2(freqs: &[i32]) -> i32 {
        let mut reached = HashSet::new();
        let mut sum = 0;

        reached.insert(sum);

        freqs
            .iter()
            .cycle()
            .take_while(|&&f| {
                sum += f;
                reached.insert(sum)
            })
            .count();

        sum
    }

    #[doc(hidden)]
    pub mod __part2_aoc_runner {
        use super::part2;
        use crate::__aoc::Day1Part2Runner;
        use aoc_runner::RunnerV2;
        use std::error::Error;

        impl<'a> RunnerV2<'a> for Day1Part2Runner {
            type Input = &'a [i32];
            type Output = i32;

            fn run(&self, input: Self::Input) -> Result<Self::Output, Box<dyn Error>> {
                Ok(part2(input))
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn part1_example() {
            assert_eq!(part1(&[1, -2, 3, 1]), 3);
            assert_eq!(part1(&[1, 1, 1]), 3);
            assert_eq!(part1(&[1, 1, -2]), 0);
            assert_eq!(part1(&[-1, -2, -3]), -6);
        }

        #[test]
        fn part2_example() {
            assert_eq!(part2(&[1, -2, 3, 1]), 2);
            assert_eq!(part2(&[1, -1]), 0);
            assert_eq!(part2(&[3, 3, 4, -2, -4]), 10);
            assert_eq!(part2(&[-6, 3, 8, 5, -6]), 5);
            assert_eq!(part2(&[7, 7, -2, -7, -4]), 14);
        }
    }
}
