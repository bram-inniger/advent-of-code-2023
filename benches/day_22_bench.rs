use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;

use advent_of_code_2023::solutions::*;

/// Run this benchmark using
/// ```shell
/// $ cargo bench
/// ```
fn day22(c: &mut Criterion) {
    let input = include_str!("../inputs/day_22.txt")
        .trim()
        .lines()
        .collect_vec();
    let mut group = c.benchmark_group("day22");

    group.bench_function("part1", |b| {
        b.iter(|| day_22::solve_1(&input));
    });

    group.bench_function("part2", |b| {
        b.iter(|| day_22::solve_2(&input));
    });

    group.bench_function("combined", |b| {
        b.iter(|| {
            day_22::solve_1(&input);
            day_22::solve_2(&input);
        })
    });
}

criterion_group!(benches, day22);
criterion_main!(benches);
