use std::fs::File;
use std::io::{self, BufRead};
use std::{collections::HashSet, env};

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let input_file_path = args.get(2).expect("Missing input file path argument");
    let part = args.get(3).unwrap_or(&String::from("part1")).clone();

    let input_file = File::open(input_file_path).expect("Cannot open input file");
    let lines = io::BufReader::new(input_file).lines();
    let values: Vec<u32> = lines
        .map(|l| l.expect("Cannot read line from file"))
        .map(|l| {
            l.parse::<u32>()
                .unwrap_or_else(|_| panic!("Cannot convert line '{:?}' to u32", l))
        })
        .collect();

    match part.as_str() {
        "part1" => assert!(part1(values) == 866436),
        "part2" => assert!(part2(values) == 276650720),
        _ => panic!("Invalid part"),
    };

    Ok(())
}

fn part1(values: Vec<u32>) -> u32 {
    let idx: HashSet<u32> = values.iter().cloned().collect();
    for v in values {
        let complement = 2020 - v;
        if idx.contains(&complement) {
            return v * complement;
        }
    }
    panic!("Solution not found");
}

fn part2(values: Vec<u32>) -> u32 {
    let idx: HashSet<u32> = values.iter().cloned().collect();
    for (i, v) in values.iter().enumerate() {
        for k in values.iter().skip(i) {
            if v + k > 2020 {
                continue;
            }
            let complement = 2020 - (v + k);
            if idx.contains(&complement) {
                return complement * v * k;
            }
        }
    }
    panic!("Solution not found");
}
