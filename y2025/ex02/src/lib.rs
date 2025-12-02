use crate::{
    parser::parse_input,
    utils::{is_id_invalid, is_id_invalid_p2},
};
mod parser;
mod utils;

pub fn part1(input: &str) -> u64 {
    let ranges = parse_input(&mut &*input).expect("Failed to parse input");
    ranges
        .iter()
        .flat_map(|r| r.clone())
        .filter(|id| is_id_invalid(*id))
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let ranges = parse_input(&mut &*input).expect("Failed to parse input");
    ranges
        .iter()
        .flat_map(|r| r.clone())
        .filter(|id| is_id_invalid_p2(*id))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");
    const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 1227775554);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 30608905813);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 4174379265);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 31898925685);
    }
}
