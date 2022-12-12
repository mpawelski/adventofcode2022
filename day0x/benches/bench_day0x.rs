// To run benchmark quick
// cargo bench --bench bench_day0x -- --quick --quiet

use criterion::{criterion_group, criterion_main, Criterion};

use day0x::*;

// static INPUT_EXAMPLE: &str = include_str!("../input_example");
static INPUT: &str = include_str!("../input");

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("part1_input");

    group.bench_function("v1", |b| b.iter(|| run_part1_v1(INPUT).unwrap()));
    group.finish();

    let mut group = c.benchmark_group("part2_input");
    group.bench_function("v1", |b| b.iter(|| run_part2_v1(INPUT).unwrap()));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
