/// Run this benchmark using
/// ```shell
/// $ cargo bench
/// ```

use criterion::{criterion_group, criterion_main, Criterion};
use advent_of_code_2023::solutions::*;

fn day18(c: &mut Criterion) {
    let input = include_str!("../inputs/day_18.txt").trim();
    let mut group = c.benchmark_group("day18");

    group.bench_function("part1", |b| {
        b.iter(|| day_18::solve_1(input));
    });

    group.bench_function("part2", |b| {
        b.iter(|| day_18::solve_2(input));
    });
}

criterion_group!(benches, day18);
criterion_main!(benches);
