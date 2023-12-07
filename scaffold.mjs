#!/usr/bin/env zx
import fs from 'fs/promises'

const year = process.argv[3]
const day = process.argv[4]

if (!year || !day) {
    console.log('Usage: scaffold.mjs <year> <day>')
    process.exit(1)
}

const yearDir = `y${year}`
const dayDir = `ex${day.padStart(2, '0')}`

// create year folder
await $`mkdir -p ./${yearDir}`

// Create rust project
// await $`cd ./y${year} && cargo new --lib ${dayDir}`

// update Cargo.toml
const cargoToml = `[package]
name = "${yearDir}${dayDir}"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

[[bench]]
name = "bench_${yearDir}${dayDir}"
harness = false

[dev-dependencies]
criterion = "0.5.1"
`
await fs.writeFile(`./${yearDir}/${dayDir}/Cargo.toml`, cargoToml)

// Scaffold lib.rs
const libRs = `
pub fn part1(input: &str) -> usize {
    0
}

pub fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
`
await fs.writeFile(`./${yearDir}/${dayDir}/src/lib.rs`, libRs)

// Create input.txt
await fs.writeFile(`./${yearDir}/${dayDir}/input.txt`, '')

// Create benchmark files
const benchRs = `
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ${yearDir}${dayDir}::{part1, part2};

fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../input.txt");
    c.bench_function("${yearDir}${dayDir}::part1", |b| b.iter(|| part1(black_box(input))));
    c.bench_function("${yearDir}${dayDir}::part2", |b| b.iter(|| part2(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
`
await $`mkdir -p ./${yearDir}/${dayDir}/benches`
await fs.writeFile(`./${yearDir}/${dayDir}/benches/bench_${yearDir}${dayDir}.rs`, benchRs)

// updates Cargo.toml workspace
const mainCargoToml = await fs.readFile('./Cargo.toml', 'utf8')
const updatedCargoToml = mainCargoToml.replace(`]

[profile.bench]
debug = true`,
    `    "${yearDir}/${dayDir}",
]

[profile.bench]
debug = true`);
await fs.writeFile('./Cargo.toml', updatedCargoToml)

// format files
await $`cd ./${yearDir}/${dayDir} && cargo fmt`

// create README.md
const readme = `# Day ${day}: TODO: ADD TITLE HERE

[Check it out on adventofcode.com](https://adventofcode.com/${year}/day/${day})

## Part One

TODO: ADD DESCRIPTION HERE

Your puzzle answer was \`?\`. (TODO: )

## Part Two

TODO: ADD DESCRIPTION HERE

Your puzzle answer was \`?\`. (TODO: )`
fs.writeFile(`./${yearDir}/${dayDir}/README.md`, readme)
