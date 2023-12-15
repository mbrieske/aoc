extern crate day_15;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_15::part1::solve;

fn criterion_benchmark(c: &mut Criterion) {
    let input = "";

    c.bench_function("solver part1", |b| b.iter(|| solve(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
