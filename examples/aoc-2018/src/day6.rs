use aoc_runner_derive::{aoc, aoc_generator};
use fnv::FnvHashMap;
use fnv::FnvHashSet;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn distance(self, other: Point) -> u32 {
        self.x.max(other.x) - self.x.min(other.x) + self.y.max(other.y) - self.y.min(other.y)
    }
}

impl FromStr for Point {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Point, Box<dyn Error>> {
        let (x, y) = s.split_at(s.find(", ").ok_or("invalid input")?);

        Ok(Point {
            x: x.parse()?,
            y: y[2..].parse()?,
        })
    }
}

fn bounds(points: &[Point]) -> (Option<Point>, Option<Point>) {
    points.iter().fold((None, None), |(tl, br), &p| {
        (
            match tl {
                None => Some(p),
                Some(tl) => Some(Point {
                    x: p.x.min(tl.x),
                    y: p.y.min(tl.y),
                }),
            },
            match br {
                None => Some(p),
                Some(br) => Some(Point {
                    x: p.x.max(br.x),
                    y: p.y.max(br.y),
                }),
            },
        )
    })
}

fn all_points(tl: Point, br: Point) -> impl Iterator<Item = Point> {
    (tl.x..=br.x).flat_map(move |x| (tl.y..=br.y).map(move |y| Point { x, y }))
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Result<Vec<Point>, Box<dyn Error>> {
    input.lines().map(Point::from_str).collect()
}

#[aoc(day6, part1)]
fn part1(points: &[Point]) -> Option<usize> {
    let (tl, br) = bounds(points);

    let tl = tl?;
    let br = br?;

    let mut infinites = FnvHashSet::default();

    let areas = all_points(tl, br)
        .filter_map(|p| {
            let mut closest = None;
            let mut closest_indexs = Vec::new();

            for (index, dist) in points.iter().map(|&other| p.distance(other)).enumerate() {
                match closest {
                    None => {
                        closest = Some(dist);
                        closest_indexs.push(index);
                    }
                    Some(min_d) if dist < min_d => {
                        closest = Some(dist);
                        closest_indexs.clear();
                        closest_indexs.push(index);
                    }
                    Some(min_d) if dist == min_d => {
                        closest_indexs.push(index);
                    }
                    _ => (),
                }
            }

            if closest_indexs.len() == 1 {
                if p.x == tl.x || p.y == tl.y || p.x == br.x || p.y == br.y {
                    infinites.extend(closest_indexs.iter().cloned())
                }

                Some(closest_indexs[0])
            } else {
                None
            }
        })
        .fold(FnvHashMap::default(), |mut acc, i| {
            *acc.entry(i).or_default() += 1;
            acc
        });

    let max_area = areas
        .into_iter()
        .filter(|(i, _)| !infinites.contains(i))
        .max_by_key(|&(_, size)| size);

    max_area.map(|(_, size)| size)
}

#[aoc(day6, part2)]
fn part2(points: &[Point]) -> Option<usize> {
    part2_internal(points, 10_000)
}

fn part2_internal(points: &[Point], dist: u32) -> Option<usize> {
    let (tl, br) = bounds(points);

    let tl = tl?;
    let br = br?;

    Some(
        all_points(tl, br)
            .map(|p| points.iter().map(|&other| p.distance(other)).sum())
            .filter(|&s: &u32| s < dist)
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9";

    #[test]
    fn part1_example() {
        let points = parse(INPUT).unwrap();

        assert_eq!(part1(&points).unwrap(), 17);
    }

    #[test]
    fn part2_example() {
        let points = parse(INPUT).unwrap();

        assert_eq!(part2_internal(&points, 32).unwrap(), 16);
    }
}
