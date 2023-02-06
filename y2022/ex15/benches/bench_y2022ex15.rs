use criterion::{black_box, criterion_group, criterion_main, Criterion};
use y2022ex15::{
    part1, part2,
    readings::{parse, parse_regex, parse_regex_lazy, Pos},
};

fn parse_and_collect_input<
    'a,
    R: Iterator<Item = (Pos, Pos, i64)> + 'a,
    F: FnOnce(&'a str) -> R,
>(
    input: &'a str,
    f: F,
) {
    let _data: Vec<_> = f(input).collect();
}

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");
    c.bench_function("y2022ex15::part1", |b| b.iter(|| part1(black_box(input))));
    c.bench_function("y2022ex15::part2", |b| b.iter(|| part2(black_box(input))));
    c.bench_function("y2022ex15::readings::parse", |b| {
        b.iter(|| parse_and_collect_input(black_box(input), parse))
    });
    c.bench_function("y2022ex15::readings::parse_regex", |b| {
        b.iter(|| parse_and_collect_input(black_box(input), parse_regex))
    });
    c.bench_function("y2022ex15::readings::parse_regex_lazy", |b| {
        b.iter(|| parse_and_collect_input(black_box(input), parse_regex_lazy))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
