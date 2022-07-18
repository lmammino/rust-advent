use criterion::{black_box, criterion_group, criterion_main, Criterion};
use y2021ex18::{part1, part2, part2_itertools, part2_permutator};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");
    c.bench_function("y2021ex18::part1", |b| b.iter(|| part1(black_box(input))));
    c.bench_function("y2021ex18::part2", |b| b.iter(|| part2(black_box(input))));
    c.bench_function("y2021ex18::part2_permutator", |b| {
        b.iter(|| part2_permutator(black_box(input)))
    });
    c.bench_function("y2021ex18::part2_itertools", |b| {
        b.iter(|| part2_itertools(black_box(input)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
