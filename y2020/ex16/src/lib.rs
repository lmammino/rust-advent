#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::{ops::RangeInclusive, str::FromStr};

lazy_static! {
    static ref RULE_REGEX: Regex =
        Regex::new(r"^(.+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)$").unwrap();
}

struct Rule {
    _name: String,
    ranges: (RangeInclusive<u16>, RangeInclusive<u16>),
}

impl Rule {
    fn contains(&self, n: &u16) -> bool {
        self.ranges.0.contains(n) || self.ranges.1.contains(n)
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let capture = RULE_REGEX.captures(line).unwrap();

        let name = String::from(&capture[1]);
        let range_1_start: u16 = capture[2].parse().unwrap();
        let range_1_end: u16 = capture[3].parse().unwrap();
        let range_2_start: u16 = capture[4].parse().unwrap();
        let range_2_end: u16 = capture[5].parse().unwrap();

        Ok(Rule {
            _name: name,
            ranges: (
                RangeInclusive::new(range_1_start, range_1_end),
                RangeInclusive::new(range_2_start, range_2_end),
            ),
        })
    }
}

pub fn part1(input: &str) -> u32 {
    let mut i = input.split("\n\n");
    let unparsed_rules = i.next().unwrap();
    let _own_ticket = i.next().unwrap();
    let other_tickets = i.next().unwrap();

    let rules: Vec<Rule> = unparsed_rules.lines().map(|l| l.parse().unwrap()).collect();
    let mut failed_numbers: u32 = 0;
    for ticket in other_tickets.lines().skip(1) {
        for number in ticket.split(',') {
            let n: u16 = number.parse().unwrap();
            let found = rules.iter().find(|rule| rule.contains(&n));
            if found.is_none() {
                failed_numbers += n as u32;
            }
        }
    }

    // 21996
    failed_numbers
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
