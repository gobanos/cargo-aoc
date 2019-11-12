use aoc_runner_derive::{aoc, aoc_generator};
use fnv::FnvHashSet;
use std::collections::HashSet;
use std::num::ParseIntError;

#[aoc_generator(day = "1")]
fn parse_input_day1(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

#[aoc(day = "1", part = "1")]
fn part1(freqs: &[i32]) -> i32 {
    freqs.iter().sum()
}

#[aoc(day = "1", part = "2")]
fn part2(freqs: &[i32]) -> i32 {
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

#[aoc(day = "1", part = "2", name = "Fnv")]
fn part2_fnv(freqs: &[i32]) -> i32 {
    let mut reached = FnvHashSet::default();
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

    #[test]
    fn part2_fnv_example() {
        assert_eq!(part2_fnv(&[1, -2, 3, 1]), 2);
        assert_eq!(part2_fnv(&[1, -1]), 0);
        assert_eq!(part2_fnv(&[3, 3, 4, -2, -4]), 10);
        assert_eq!(part2_fnv(&[-6, 3, 8, 5, -6]), 5);
        assert_eq!(part2_fnv(&[7, 7, -2, -7, -4]), 14);
    }
}
