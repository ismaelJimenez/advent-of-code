use criterion::{black_box, criterion_group, criterion_main, Criterion};
use y2022day01::{part1, part2_itertools, part2_no_sort};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");
    c.bench_function("y2022day01::part1", |b| {
        b.iter(|| part1(black_box(input)))
    });
    c.bench_function("y2022day01::part2_itertools", |b| {
        b.iter(|| part2_itertools(black_box(input)))
    });
    c.bench_function("y2022day01::part2_no_sort", |b| {
        b.iter(|| part2_no_sort(black_box(input)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
