use aoc_runner_derive::{aoc, aoc_generator};

type Data = usize;

pub struct Node {
    children: Vec<Node>,
    metadata: Vec<Data>,
}

impl Node {
    fn from_iter<I>(iter: &mut I) -> Option<Node>
    where
        I: Iterator<Item = Data>,
    {
        let nb_children = iter.next()?;
        let nb_metadata = iter.next()?;

        let mut children = Vec::with_capacity(nb_children);
        for _ in 0..nb_children {
            children.push(Node::from_iter(iter)?);
        }

        let metadata = iter.take(nb_metadata).collect();

        Some(Node { children, metadata })
    }

    fn checksum(&self) -> Data {
        let children_sum: Data = self.children.iter().map(|c| c.checksum()).sum();
        let self_sum: Data = self.metadata.iter().sum();

        children_sum + self_sum
    }

    fn value(&self) -> Data {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .iter()
                .filter_map(|i| match i {
                    0 => None,
                    _ => self.children.get(i - 1).map(|c| c.value()),
                })
                .sum()
        }
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Option<Node> {
    Node::from_iter(&mut input.split_whitespace().map(|s| s.parse().unwrap()))
}

#[aoc(day = "8", part = "1")]
fn part1(root: &Node) -> Data {
    root.checksum()
}

#[aoc(day = "8", part = "2")]
fn part2(root: &Node) -> Data {
    root.value()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn part1_example() {
        let root = parse(INPUT).unwrap();

        assert_eq!(part1(&root), 138);
    }

    #[test]
    fn part2_example() {
        let root = parse(INPUT).unwrap();

        assert_eq!(part2(&root), 66);
    }
}
