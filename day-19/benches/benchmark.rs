extern crate day_19;
use std::fs::read_to_string;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_19::part2::solve;

fn criterion_benchmark(c: &mut Criterion) {
    let input = read_to_string("res/input").unwrap();

    c.bench_function("solver part1", |b| b.iter(|| solve(black_box(&input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
