use aoc_runner_derive::{aoc, aoc_generator};
use fnv::FnvHashSet;
use std::error::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Claim {
    id: u32,
    rect: Rectangle,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Rectangle {
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl Rectangle {
    fn overlaps(&self, other: &Rectangle) -> Option<Rectangle> {
        let top = u32::max(self.top, other.top);
        let left = u32::max(self.left, other.left);

        let bottom = u32::min(self.top + self.height, other.top + other.height);
        let right = u32::min(self.left + self.width, other.left + other.width);

        if let (Some(height), Some(width)) = (bottom.checked_sub(top), right.checked_sub(left)) {
            if height > 0 && width > 0 {
                Some(Rectangle {
                    top,
                    left,
                    width,
                    height,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Result<Vec<Claim>, Box<dyn Error>> {
    input
        .lines()
        .map(|l| {
            // #123 @ 3,2: 5x4
            let (id, rect) = l.split_at(l.find('@').ok_or("@ not found")?);
            let id: u32 = id.get(1..).ok_or("id not found")?.trim().parse()?;
            let rect = rect.get(1..).ok_or("rect not found")?;
            let (pos, size) = rect.split_at(rect.find(':').ok_or(": not found")?);
            let pos = pos.trim();

            let (left, top) = pos.split_at(pos.find(',').ok_or(", not found")?);
            let left: u32 = left.trim().parse()?;
            let top: u32 = top.get(1..).ok_or("top not found")?.trim().parse()?;

            let size = size.get(1..).ok_or("size not found")?.trim();
            let (width, height) = size.split_at(size.find('x').ok_or("x not found")?);
            let width: u32 = width.trim().parse()?;
            let height: u32 = height.get(1..).ok_or("height not found")?.trim().parse()?;

            Ok(Claim {
                id,
                rect: Rectangle {
                    left,
                    top,
                    width,
                    height,
                },
            })
        })
        .collect()
}

#[aoc(day3, part1)]
fn part1(claims: &[Claim]) -> usize {
    let mut overlaps = FnvHashSet::default();

    for (i, claim) in claims.iter().enumerate() {
        for other in claims.iter().take(i) {
            if let Some(overlap) = claim.rect.overlaps(&other.rect) {
                for x in overlap.left..overlap.left + overlap.width {
                    for y in overlap.top..overlap.top + overlap.height {
                        overlaps.insert((x, y));
                    }
                }
            }
        }
    }

    overlaps.len()
}

#[aoc(day3, part2)]
fn part2(claims: &[Claim]) -> Option<u32> {
    claims.iter().find_map(|c| {
        if claims
            .iter()
            .filter(|&o| o != c)
            .all(|o| o.rect.overlaps(&c.rect).is_none())
        {
            Some(c.id)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
    const CLAIM_1: Claim = Claim {
        id: 1,
        rect: Rectangle {
            left: 1,
            top: 3,
            width: 4,
            height: 4,
        },
    };
    const CLAIM_2: Claim = Claim {
        id: 2,
        rect: Rectangle {
            left: 3,
            top: 1,
            width: 4,
            height: 4,
        },
    };
    const CLAIM_3: Claim = Claim {
        id: 3,
        rect: Rectangle {
            left: 5,
            top: 5,
            width: 2,
            height: 2,
        },
    };

    #[test]
    fn parse_example() {
        assert_eq!(parse(INPUT).unwrap(), vec![CLAIM_1, CLAIM_2, CLAIM_3,]);
    }

    #[test]
    fn overlaps() {
        assert_eq!(
            CLAIM_1.rect.overlaps(&CLAIM_2.rect),
            Some(Rectangle {
                top: 3,
                left: 3,
                width: 2,
                height: 2,
            })
        );
        assert_eq!(CLAIM_1.rect.overlaps(&CLAIM_3.rect), None);
        assert_eq!(CLAIM_2.rect.overlaps(&CLAIM_3.rect), None);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&[CLAIM_1, CLAIM_2, CLAIM_3,]), 4);
    }

    #[test]
    fn part3_example() {
        assert_eq!(part2(&[CLAIM_1, CLAIM_2, CLAIM_3,]).unwrap(), 3);
    }
}
