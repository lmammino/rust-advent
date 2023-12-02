use std::collections::HashMap;

const SPELLED_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

type MatcherNodes = HashMap<char, Vec<(&'static str, u32)>>;

trait Matcher {
    fn matches(&self, s: &str) -> Option<u32>;
}

#[derive(Debug)]
struct MatchFromLeft {
    nodes: MatcherNodes,
}

impl MatchFromLeft {
    fn new() -> Self {
        let mut nodes = HashMap::new();
        // - 'o' -> [("ne", 1)]
        // - 't' -> [("wo", 2), ("hree", 3)]
        // - 'f' -> [("our", 4), ("ive", 5)]
        // - 's' -> [("ix", 6), ("even", 7)]
        // - 'e' -> [("ight", 8)]
        // - 'n' -> [("ine", 9)]
        for (spelled_i, spelled) in SPELLED_DIGITS.iter().enumerate() {
            let first_char = spelled.chars().next().unwrap();
            nodes
                .entry(first_char)
                .or_insert(Vec::new())
                .push((&spelled[1..], (spelled_i + 1) as u32));
        }

        Self { nodes }
    }

    fn matches(&self, s: &str) -> Option<u32> {
        for (i, c) in s.char_indices() {
            if let Some(v) = c.to_digit(10) {
                return Some(v);
            }
            if i == s.len() - 1 {
                return None;
            }
            if let Some(matchers) = self.nodes.get(&c) {
                for (suffix, value) in matchers {
                    if s[i + 1..].starts_with(suffix) {
                        return Some(*value);
                    }
                }
            }
        }

        None
    }
}

#[derive(Debug)]
struct MatchFromRight {
    nodes: MatcherNodes,
}

impl MatchFromRight {
    fn new() -> Self {
        let mut nodes = HashMap::new();
        // - 'e' -> [("on", 1), ("thre", 3), ("fiv", 5), ("nin", 3)]
        // - 'o' -> [("tw", 2)]
        // - 'r' -> [("fou", 4)]
        // - 'x' -> [("si", 6)]
        // - 'n' -> [("seve", 7)]
        // - 't' -> [("eigh", 8)]
        for (spelled_i, spelled) in SPELLED_DIGITS.iter().enumerate() {
            let last_char = spelled.chars().last().unwrap();
            nodes
                .entry(last_char)
                .or_insert(Vec::new())
                .push((&spelled[..&spelled.len() - 1], (spelled_i + 1) as u32));
        }

        Self { nodes }
    }

    fn matches(&self, s: &str) -> Option<u32> {
        for (i, c) in s.char_indices().rev() {
            if let Some(v) = c.to_digit(10) {
                return Some(v);
            }
            if i == 0 {
                return None;
            }
            if let Some(matchers) = self.nodes.get(&c) {
                for (prefix, value) in matchers {
                    if s[0..i].ends_with(prefix) {
                        return Some(*value);
                    }
                }
            }
        }

        None
    }
}

fn parse_line_simple(line: &str) -> u32 {
    let left = line.chars().find_map(|c| c.to_digit(10)).unwrap();
    let right = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
    left * 10 + right
}

fn parse_line_advanced(
    line: &str,
    match_left: &MatchFromLeft,
    match_right: &MatchFromRight,
) -> u32 {
    let left = match_left.matches(line).unwrap();
    let right = match_right.matches(line).unwrap();
    left * 10 + right
}

pub fn part1(input: &str) -> u32 {
    input.lines().map(parse_line_simple).sum()
}

pub fn part2(input: &str) -> u32 {
    let match_left = MatchFromLeft::new();
    let match_right = MatchFromRight::new();
    input
        .lines()
        .map(|line| parse_line_advanced(line, &match_left, &match_right))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_parse_line_simple() {
        assert_eq!(parse_line_simple("1abc2"), 12);
        assert_eq!(parse_line_simple("pqr3stu8vwx"), 38);
        assert_eq!(parse_line_simple("a1b2c3d4e5f"), 15);
        assert_eq!(parse_line_simple("treb7uchet"), 77);
    }

    #[test]
    fn test_match_from_left() {
        let match_left = MatchFromLeft::new();
        assert_eq!(match_left.matches("0"), Some(0));
        assert_eq!(match_left.matches("1"), Some(1));
        assert_eq!(match_left.matches("2"), Some(2));
        assert_eq!(match_left.matches("3"), Some(3));
        assert_eq!(match_left.matches("4"), Some(4));
        assert_eq!(match_left.matches("5"), Some(5));
        assert_eq!(match_left.matches("6"), Some(6));
        assert_eq!(match_left.matches("7"), Some(7));
        assert_eq!(match_left.matches("8"), Some(8));
        assert_eq!(match_left.matches("9"), Some(9));
        assert_eq!(match_left.matches("one_"), Some(1));
        assert_eq!(match_left.matches("two__"), Some(2));
        assert_eq!(match_left.matches("three_"), Some(3));
        assert_eq!(match_left.matches("four_"), Some(4));
        assert_eq!(match_left.matches("five___"), Some(5));
        assert_eq!(match_left.matches("six_"), Some(6));
        assert_eq!(match_left.matches("seven_"), Some(7));
        assert_eq!(match_left.matches("eight_"), Some(8));
        assert_eq!(match_left.matches("nine_"), Some(9));
        assert_eq!(match_left.matches("_one_two9"), Some(1));
    }

    #[test]
    fn test_match_from_right() {
        let match_right = MatchFromRight::new();
        assert_eq!(match_right.matches("0"), Some(0));
        assert_eq!(match_right.matches("1"), Some(1));
        assert_eq!(match_right.matches("2"), Some(2));
        assert_eq!(match_right.matches("3"), Some(3));
        assert_eq!(match_right.matches("4"), Some(4));
        assert_eq!(match_right.matches("5"), Some(5));
        assert_eq!(match_right.matches("6"), Some(6));
        assert_eq!(match_right.matches("7"), Some(7));
        assert_eq!(match_right.matches("8"), Some(8));
        assert_eq!(match_right.matches("9"), Some(9));
        assert_eq!(match_right.matches("_one"), Some(1));
        assert_eq!(match_right.matches("__two"), Some(2));
        assert_eq!(match_right.matches("_three"), Some(3));
        assert_eq!(match_right.matches("four"), Some(4));
        assert_eq!(match_right.matches("___five"), Some(5));
        assert_eq!(match_right.matches("six"), Some(6));
        assert_eq!(match_right.matches("seven"), Some(7));
        assert_eq!(match_right.matches("eight"), Some(8));
        assert_eq!(match_right.matches("nine"), Some(9));
        assert_eq!(match_right.matches("two1nine"), Some(9));
    }

    #[test]
    fn test_parse_line_advanced() {
        let match_left = MatchFromLeft::new();
        let match_right = MatchFromRight::new();
        assert_eq!(
            parse_line_advanced("two1nine", &match_left, &match_right),
            29
        );
        assert_eq!(
            parse_line_advanced("eightwothree", &match_left, &match_right),
            83
        );
        assert_eq!(
            parse_line_advanced("abcone2threexyz", &match_left, &match_right),
            13
        );
        assert_eq!(
            parse_line_advanced("xtwone3four", &match_left, &match_right),
            24
        );
        assert_eq!(
            parse_line_advanced("4nineeightseven2", &match_left, &match_right),
            42
        );
        assert_eq!(
            parse_line_advanced("zoneight234", &match_left, &match_right),
            14
        );
        assert_eq!(
            parse_line_advanced("7pqrstsixteen", &match_left, &match_right),
            76
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 54953);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
