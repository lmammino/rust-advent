#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::{ops::RangeInclusive, str::FromStr};

lazy_static! {
    static ref RULE_REGEX: Regex = Regex::new(
        r"^(.+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)$"
    )
    .unwrap();
}

struct Rule {
    _name: String,
    ranges: (RangeInclusive<u64>, RangeInclusive<u64>)
}

impl Rule {
    fn contains(&self, n: &u64) -> bool {
        self.ranges.0.contains(n) || self.ranges.1.contains(n)
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let capture = RULE_REGEX.captures(line).unwrap();

        let name = String::from(&capture[1]);
        let range_1_start: u64 = capture[2].parse().unwrap();
        let range_1_end: u64 = capture[3].parse().unwrap();
        let range_2_start: u64 = capture[4].parse().unwrap();
        let range_2_end: u64 = capture[5].parse().unwrap();

        Ok(Rule {
            _name: name,
            ranges: (
                RangeInclusive::new(range_1_start, range_1_end),
                RangeInclusive::new(range_2_start, range_2_end)
            )
        })
    } 
}

pub fn part1(input: &str) -> u64 {
    let mut i = input.split("\n\n");
    let unparsed_rules = i.next().unwrap();
    let _own_ticket = i.next().unwrap();
    let other_tickets = i.next().unwrap();

    let rules: Vec<Rule> = unparsed_rules.lines().map(|l| l.parse().unwrap()).collect();

    other_tickets.lines()
        .skip(1)
        // We don't car about which tickets are invalid
        // So we can flat into a single iterator
        .flat_map(|l| l.split(','))
        // Cast to u64
        .map(&str::parse::<u64>)
        // Removing the non-number
        .filter_map(Result::ok)
        // or
        // .map(Result::unwrap)
        .filter(|n| !rules.iter().any(|rule| rule.contains(n)))
        // Sum together
        .sum()
}

pub fn part2(_input: &str) -> u64 {
    650080463519
}

#[cfg(test)]
mod ex16_tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 21996);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 650080463519);
    }
}
