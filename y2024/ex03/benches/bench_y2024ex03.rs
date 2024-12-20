use criterion::{black_box, criterion_group, criterion_main, Criterion};
use y2024ex03::{part1, part2};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");
    c.bench_function("y2024ex03::part1", |b| b.iter(|| part1(black_box(input))));
    c.bench_function("y2024ex03::part2", |b| b.iter(|| part2(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
