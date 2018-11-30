#[macro_use]
extern crate criterion;
extern crate advent_of_code_2015;
extern crate aoc_runner;

use advent_of_code_2015::*;
use criterion::Criterion;
use criterion::Fun;

fn aoc_benchmark(c: &mut Criterion) {
    use aoc_runner::ArcStr;

    let mut day1_part1 = Vec::new();
    {
        let runner = Factory::day1_part1_bytes(ArcStr::from(include_str!("../../../../input/2015/day1.txt")));
        let fun = Fun::new("Bytes", move |b, _| b.iter(|| runner.run()));
        day1_part1.push(fun);
    }
    {
        let runner = Factory::day1_part1_char(ArcStr::from(include_str!("../../../../input/2015/day1.txt")));
        let fun = Fun::new("Char", move |b, _| b.iter(|| runner.run()));
        day1_part1.push(fun);
    }

    c.bench_functions("Day1 - Part1", day1_part1, ());

    let mut day1_part2 = Vec::new();
    {
        let runner = Factory::day1_part2(ArcStr::from(include_str!("../../../../input/2015/day1.txt")));
        let fun = Fun::new("(default)", move |b, _| b.iter(|| runner.run()));
        day1_part2.push(fun);
    }
    c.bench_functions("Day1 - Part2", day1_part2, ());
}

criterion_group!(benches, aoc_benchmark);
criterion_main!(benches);
