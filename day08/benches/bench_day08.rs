// To run benchmark quick
// cargo bench --bench bench_day08 -- --quick --quiet

use criterion::{criterion_group, criterion_main, Criterion};

use day08::*;

// static INPUT_EXAMPLE: &str = include_str!("../input_example");
static INPUT: &str = include_str!("../input");

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("part1_input");
    group.bench_function("v1", |b| b.iter(|| v1::run_part1_v1(INPUT).unwrap()));
    group.bench_function("v2", |b| b.iter(|| v2::run_part1_v2(INPUT).unwrap()));
    group.bench_function("v3", |b| b.iter(|| v3::run_part1_v3(INPUT).unwrap()));
    group.bench_function("v4", |b| b.iter(|| v4::run_part1_v4(INPUT).unwrap()));
    group.bench_function("v5", |b| b.iter(|| v5::run_part1_v5(INPUT).unwrap()));
    group.finish();

    let mut group = c.benchmark_group("part2_input");
    group.bench_function("v1", |b| b.iter(|| v1::run_part2_v1(INPUT).unwrap()));
    group.bench_function("v2", |b| b.iter(|| v2::run_part2_v2(INPUT).unwrap()));
    group.bench_function("v3", |b| b.iter(|| v3::run_part2_v3(INPUT).unwrap()));
    group.bench_function("v4", |b| b.iter(|| v4::run_part2_v4(INPUT).unwrap()));
    group.bench_function("v5", |b| b.iter(|| v5::run_part2_v5(INPUT).unwrap()));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

// Results:
// $ cargo bench --bench bench_day08
// $ critcmp new -g '(part\d)' --list
/*
part1
-----
new/_input/v2     1.00    728.7±22.95µs       ? ?/sec
new/_input/v1     1.08   790.6±107.66µs       ? ?/sec
new/_input/v4     2.16   1572.5±29.68µs       ? ?/sec
new/_input/v5     3.07       2.2±0.13ms       ? ?/sec
new/_input/v3   651.68     474.9±7.11ms       ? ?/sec

part2
-----
new/_input/v2     1.00     206.2±8.57µs       ? ?/sec
new/_input/v1     1.00     207.0±8.49µs       ? ?/sec
new/_input/v4     1.19     244.9±8.10µs       ? ?/sec
new/_input/v5     1.36     280.1±5.75µs       ? ?/sec
new/_input/v3  2044.51     421.5±7.14ms       ? ?/sec
 */
