#[macro_use]
extern crate criterion;
extern crate {CRATE_SLUG};
extern crate aoc_runner;

use {CRATE_SLUG}::*;
use aoc_runner::ArcStr;
use criterion::Criterion;
use criterion::Fun;

fn aoc_benchmark(c: &mut Criterion) {
    {PARTS}
}

criterion_group!(benches, aoc_benchmark);
criterion_main!(benches);
