extern crate day_14;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_14::part2::solve;

fn criterion_benchmark(c: &mut Criterion) {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    c.bench_function("solver part2", |b| b.iter(|| solve(black_box(input), 10)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
