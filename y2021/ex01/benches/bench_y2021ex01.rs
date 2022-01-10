use criterion::{black_box, criterion_group, criterion_main, Criterion};
use y2021ex01::{part1, part1_iter, part1_zip, part2};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");
    c.bench_function("y2021ex01::part1", |b| b.iter(|| part1(black_box(input))));
    c.bench_function("y2021ex01::part1_iter", |b| {
        b.iter(|| part1_iter(black_box(input)))
    });
    c.bench_function("y2021ex01::part1_zip", |b| {
        b.iter(|| part1_zip(black_box(input)))
    });
    c.bench_function("y2021ex01::part2", |b| b.iter(|| part2(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
