use criterion::{black_box, criterion_group, criterion_main, Criterion};
use y2022ex01::{
    part1, part1_classic, part1_combinators, part2, part2_combinators_itertools,
    part2_combinators_no_sort, part2_combinators_no_sort_const,
    part2_combinators_no_sort_const_custom_iter,
};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");
    c.bench_function("y2022ex01::part1", |b| b.iter(|| part1(black_box(input))));
    c.bench_function("y2022ex01::part1_classic", |b| {
        b.iter(|| part1_classic(black_box(input)))
    });
    c.bench_function("y2022ex01::part1_combinators", |b| {
        b.iter(|| part1_combinators(black_box(input)))
    });
    c.bench_function("y2022ex01::part2", |b| b.iter(|| part2(black_box(input))));
    c.bench_function("y2022ex01::part2_combinators_itertools", |b| {
        b.iter(|| part2_combinators_itertools(black_box(input)))
    });
    c.bench_function("y2022ex01::part2_combinators_no_sort", |b| {
        b.iter(|| part2_combinators_no_sort(black_box(input)))
    });
    c.bench_function("y2022ex01::part2_combinators_no_sort_const", |b| {
        b.iter(|| part2_combinators_no_sort_const(black_box(input)))
    });
    c.bench_function(
        "y2022ex01::part2_combinators_no_sort_const_custom_iter",
        |b| b.iter(|| part2_combinators_no_sort_const_custom_iter(black_box(input))),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
