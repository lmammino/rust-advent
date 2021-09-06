use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ex17::{part1, part1_gen, part2, part2_gen};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");
    c.bench_function("ex17::part1", |b| b.iter(|| part1(black_box(input))));
    c.bench_function("ex17::part1_gen", |b| {
        b.iter(|| part1_gen(black_box(input)))
    });
    c.bench_function("ex17::part2", |b| b.iter(|| part2(black_box(input))));
    c.bench_function("ex17::part2_gen", |b| {
        b.iter(|| part2_gen(black_box(input)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
