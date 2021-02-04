use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let input_file_path = args.get(2).expect("Missing input file path argument");
    let default_part = String::from("part1");
    let part = args.get(3).unwrap_or(&default_part);

    let input_file = File::open(input_file_path).expect("Cannot open input file");
    let lines = io::BufReader::new(input_file).lines();
    let values: Vec<u32> = lines
        .map(|l| l.expect("Cannot read line from file"))
        .map(|l| {
            l.parse::<u32>()
                .expect(&format!("Cannot convert line '{:?}' to u32", l))
        })
        .collect();

    let result = match part.as_str() {
        "part1" => part1(values),
        "part2" => part2(values),
        _ => panic!("Invalid part"),
    };

    println!("{:?}", result);

    Ok(())
}

fn part1(values: Vec<u32>) -> u32 {
    0
}

fn part2(values: Vec<u32>) -> u32 {
    0
}
