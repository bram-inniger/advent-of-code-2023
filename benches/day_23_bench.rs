use criterion::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;

use advent_of_code_2023::solutions::*;

/// Run this benchmark using
/// ```shell
/// $ cargo bench
/// ```
fn day23(c: &mut Criterion) {
    let input = include_str!("../inputs/day_23.txt")
        .trim()
        .lines()
        .collect_vec();
    let mut group = c.benchmark_group("day23");

    group.bench_function("part1", |b| {
        b.iter(|| day_23::solve_1(&input));
    });

    group.bench_function("part2", |b| {
        b.iter(|| day_23::solve_2(&input));
    });

    group.bench_function("combined", |b| {
        b.iter(|| {
            day_23::solve_1(&input);
            day_23::solve_2(&input);
        })
    });
}

criterion_group!(benches, day23);
criterion_main!(benches);
