// To run benchmark quick
// cargo bench --bench bench_day07 -- --quick --quiet

use criterion::{criterion_group, criterion_main, Criterion};

use day07::*;

// static INPUT_EXAMPLE: &str = include_str!("../input_example");
static INPUT: &str = include_str!("../input");

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("part1_input");

    group.bench_function("v1", |b| b.iter(|| v1::run_part1_v1(INPUT).unwrap()));
    group.finish();

    let mut group = c.benchmark_group("part2_input");
    group.bench_function("v1", |b| b.iter(|| v1::run_part2_v1(INPUT).unwrap()));
    group.bench_function("v2", |b| b.iter(|| v2::run_part2_v2(INPUT).unwrap()));
    group.bench_function("v2_smartstring", |b| {
        b.iter(|| v2_smartstring::run_part2_v2_smartstring(INPUT).unwrap())
    });
    group.bench_function("v3", |b| b.iter(|| v3::run_part2_v3(INPUT).unwrap()));
    group.bench_function("v4", |b| b.iter(|| v4::run_part2_v4(INPUT).unwrap()));
    group.bench_function("v5", |b| b.iter(|| v5::run_part2_v5(INPUT).unwrap()));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

/*
> # Results:
> critcmp new -g '(part\d)' --list
part1
-----
new/_input/v1     1.00     108.7±2.07µs       ? ?/sec

part2
-----
new/_input/v2                 1.00     110.1±3.76µs       ? ?/sec
new/_input/v3                 1.00     110.5±1.77µs       ? ?/sec
new/_input/v1                 1.00     110.6±3.88µs       ? ?/sec
new/_input/v2_smartstring     1.03     113.7±2.81µs       ? ?/sec
new/_input/v4                 2.66     292.7±5.61µs       ? ?/sec
new/_input/v5                 2.82     311.1±7.28µs       ? ?/sec

*/
