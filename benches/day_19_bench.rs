use criterion::{Criterion, criterion_group, criterion_main};

use advent_of_code_2023::solutions::*;

/// Run this benchmark using
/// ```shell
/// $ cargo bench
/// ```
fn day19(c: &mut Criterion) {
    let input = include_str!("../inputs/day_19.txt").trim();
    let mut group = c.benchmark_group("day19");

    group.bench_function("part1", |b| {
        b.iter(|| day_19::solve_1(input));
    });

    group.bench_function("part2", |b| {
        b.iter(|| day_19::solve_2(input));
    });
}

criterion_group!(benches, day19);
criterion_main!(benches);
