use aoc_runner_derive::aoc;
use fnv::FnvHashMap;
use std::collections::HashMap;

#[aoc(day2, part1)]
fn part1(input: &str) -> u32 {
    let (nb_double, nb_triple) = input
        .lines()
        .map(|l| {
            let mut map = HashMap::with_capacity(l.len());

            l.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);

            let twice = map.values().any(|&n| n == 2);
            let three_times = map.values().any(|&n| n == 3);

            (twice, three_times)
        })
        .fold((0, 0), |acc, n| match n {
            (true, true) => (acc.0 + 1, acc.1 + 1),
            (true, false) => (acc.0 + 1, acc.1),
            (false, true) => (acc.0, acc.1 + 1),
            (false, false) => acc,
        });

    nb_double * nb_triple
}

#[aoc(day2, part1, Fnv)]
fn part1_fnv(input: &str) -> u32 {
    let (nb_double, nb_triple) = input
        .lines()
        .map(|l| {
            let mut map = FnvHashMap::default();
            map.reserve(l.len());

            l.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);

            let twice = map.values().any(|&n| n == 2);
            let three_times = map.values().any(|&n| n == 3);

            (twice, three_times)
        })
        .fold((0, 0), |acc, n| match n {
            (true, true) => (acc.0 + 1, acc.1 + 1),
            (true, false) => (acc.0 + 1, acc.1),
            (false, true) => (acc.0, acc.1 + 1),
            (false, false) => acc,
        });

    nb_double * nb_triple
}

#[aoc(day2, part2)]
fn part2(input: &str) -> String {
    let lines = input.lines();

    for (i, l1) in lines.clone().enumerate() {
        for (_, l2) in lines.clone().enumerate().filter(|&(j, _)| i != j) {
            let filtered: String = l1
                .chars()
                .zip(l2.chars())
                .filter_map(|(a, b)| if a == b { Some(a) } else { None })
                .collect();

            if filtered.len() == l1.len() - 1 {
                return filtered;
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_PART1: &str = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
    const INPUT_PART2: &str = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT_PART1), 12);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT_PART2), "fgij");
    }
}
