use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator({DAY})]
fn parse(input: &str) -> String {
    todo!()
}

#[aoc({DAY}, part1)]
fn part1(input: &str) -> String {
    todo!()
}

#[aoc({DAY}, part2)]
fn part2(input: &str) -> String {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}