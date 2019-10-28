#[doc(hidden)]
pub mod __aoc {
    use std::error::Error;

    pub trait Generator<'a> {
        type Output;

        fn generate(&self, input: &'a str) -> Result<Self::Output, Box<dyn Error>>;

        fn is_default(&self) -> bool { false }
    }

    pub struct Day1Part1Generator;

    impl<'a> Generator<'a> for &Day1Part1Generator {
        type Output = &'a str;

        fn generate(&self, input: &'a str) -> Result<Self::Output, Box<dyn Error>> {
            Ok(input)
        }

        fn is_default(&self) -> bool { true }
    }

    pub struct Day1Part2Generator;

    impl<'a> Generator<'a> for &Day1Part2Generator {
        type Output = &'a str;

        fn generate(&self, input: &'a str) -> Result<Self::Output, Box<dyn Error>> {
            Ok(input)
        }

        fn is_default(&self) -> bool { true }
    }
}

mod day1 {
    use std::collections::HashSet;
    use std::num::ParseIntError;
    use std::error::Error;
    use std::ops::Deref;

    impl<'a> crate::__aoc::Generator<'a> for crate::__aoc::Day1Part1Generator {
        type Output = Vec<i32>;

        fn generate(&self, input: &'a str) -> Result<Self::Output, Box<dyn Error>> {
            parse_input_day1(input).map_err(|err| err.into())
        }
    }

    impl<'a> crate::__aoc::Generator<'a> for crate::__aoc::Day1Part2Generator {
        type Output = Vec<i32>;

        fn generate(&self, input: &'a str) -> Result<Self::Output, Box<dyn Error>> {
            parse_input_day1(input).map_err(|err| err.into())
        }
    }

    fn parse_input_day1(input: &str) -> Result<Vec<i32>, ParseIntError> {
        input.lines().map(|l| l.parse()).collect()
    }

    fn parse_input_day1_unwrap(input: &str) -> Vec<i32> {
        input.lines().map(|l| l.parse().unwrap()).collect()
    }

    pub struct Day1Part1UnwrapGenerator { base: crate::__aoc::Day1Part1Generator }

    impl Default for Day1Part1UnwrapGenerator {
        fn default() -> Self {
            Day1Part1UnwrapGenerator { base: crate::__aoc::Day1Part1Generator }
        }
    }

    impl Deref for Day1Part1UnwrapGenerator {
        type Target = crate::__aoc::Day1Part1Generator;

        fn deref(&self) -> &Self::Target {
            &self.base
        }
    }

    impl<'a> crate::__aoc::Generator<'a> for Day1Part1UnwrapGenerator {
        type Output = Vec<i32>;

        fn generate(&self, input: &'a str) -> Result<Self::Output, Box<dyn Error>> {
            Ok(parse_input_day1_unwrap(input))
        }
    }

    pub struct Day1Part2UnwrapGenerator { base: crate::__aoc::Day1Part2Generator }

    impl Default for Day1Part2UnwrapGenerator {
        fn default() -> Self {
            Day1Part2UnwrapGenerator { base: crate::__aoc::Day1Part2Generator }
        }
    }

    impl Deref for Day1Part2UnwrapGenerator {
        type Target = crate::__aoc::Day1Part2Generator;

        fn deref(&self) -> &Self::Target {
            &self.base
        }
    }

    pub fn part1(freqs: &[i32]) -> i32 {
        freqs.iter().sum()
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
