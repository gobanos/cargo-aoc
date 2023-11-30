#[macro_use]
extern crate criterion;
extern crate {CRATE_SLUG};
extern crate aoc_runner;

use {CRATE_SLUG}::*;
use aoc_runner::ArcStr;
use criterion::Criterion;
use std::fmt::Display;

#[inline]
fn black_box(t: &dyn Display) {
    criterion::black_box(t);
}

fn aoc_benchmark(c: &mut Criterion) {
    {INPUTS}

    {PARTS}
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn input_benchmark(c: &mut Criterion) {
    {INPUTS}

    {GENS}
}

criterion_group!(benches, {BENCHMARKS});
criterion_main!(benches);
