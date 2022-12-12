// To run benchmark quick
// cargo bench --bench bench_day05 -- --quick --quiet

use criterion::{criterion_group, criterion_main, Criterion};

use day05::*;

// static INPUT_EXAMPLE: &str = include_str!("../input_example");
static INPUT: &str = include_str!("../input");

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("part1_input");

    group
        .bench_function("v1", |b| b.iter(|| run_part1_v1(INPUT).unwrap()))
        .bench_function("v2", |b| b.iter(|| run_part1_v2(INPUT).unwrap()))
        .bench_function("v3", |b| b.iter(|| run_part1_v3(INPUT).unwrap()));
    group.finish();

    let mut group = c.benchmark_group("part2_input");
    group
        .bench_function("v1", |b| b.iter(|| run_part2_v1(INPUT).unwrap()))
        .bench_function("v2", |b| b.iter(|| run_part2_v2(INPUT).unwrap()))
        .bench_function("v3", |b| b.iter(|| run_part2_v3(INPUT).unwrap()))
        .bench_function("v4", |b| b.iter(|| run_part2_v4(INPUT).unwrap()))
        .bench_function("v5", |b| b.iter(|| run_part2_v5(INPUT).unwrap()))
        .bench_function("v6", |b| b.iter(|| run_part2_v6(INPUT).unwrap()));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
