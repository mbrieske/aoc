extern crate day_12;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_12::part2::solve;

fn criterion_benchmark(c: &mut Criterion) {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    c.bench_function("solver part2", |b| b.iter(|| solve(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
