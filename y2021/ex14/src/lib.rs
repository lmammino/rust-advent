use std::{collections::HashMap, fmt::Debug, str::FromStr};

#[derive(Debug)]
struct Polymer {
    rules: HashMap<(char, char), char>,
    chars_count: HashMap<char, usize>,
    segments_count: HashMap<(char, char), usize>,
    next_segments_count: HashMap<(char, char), usize>,
}

impl FromStr for Polymer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_seq, raw_rules) = s.split_once("\n\n").unwrap();
        let mut chars_count: HashMap<char, usize> = Default::default();
        let mut segments_count: HashMap<(char, char), usize> = Default::default();

        for c in raw_seq.chars() {
            let entry = chars_count.entry(c).or_default();
            *entry += 1;
        }

        let chars1 = raw_seq.chars();
        let chars2 = chars1.clone().skip(1);
        for segment_id in chars1.zip(chars2) {
            let count = segments_count.entry(segment_id).or_default();
            *count += 1;
        }

        let mut rules: HashMap<(char, char), char> = Default::default();
        for line in raw_rules.lines() {
            let (pair, subst) = line.split_once(" -> ").unwrap();
            let mut pair = pair.chars();
            let pair1 = pair.next().unwrap();
            let pair2 = pair.next().unwrap();
            let subst = subst.chars().next().unwrap();
            rules.insert((pair1, pair2), subst);
        }

        let next_segments_count = Default::default();

        Ok(Polymer {
            rules,
            chars_count,
            segments_count,
            next_segments_count,
        })
    }
}

impl Iterator for Polymer {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        self.next_segments_count.clear();

        for (segment, count) in &self.segments_count {
            let new_element = self.rules.get(segment).unwrap();

            let element_count = self.chars_count.entry(*new_element).or_default();
            *element_count += count;

            let new_segment1 = (segment.0, *new_element);
            let new_segment2 = (*new_element, segment.1);

            let entry1 = self.next_segments_count.entry(new_segment1).or_default();
            *entry1 += count;
            let entry2 = self.next_segments_count.entry(new_segment2).or_default();
            *entry2 += count;
        }

        std::mem::swap(&mut self.segments_count, &mut self.next_segments_count);

        Some(())
    }
}

impl Polymer {
    fn solution(&self) -> usize {
        self.chars_count.values().max().unwrap() - self.chars_count.values().min().unwrap()
    }
}

pub fn part1(input: &str) -> usize {
    let mut polymer: Polymer = input.parse().unwrap();
    polymer.nth(9);
    polymer.solution()
}

pub fn part2(input: &str) -> usize {
    let mut polymer: Polymer = input.parse().unwrap();
    polymer.nth(39);
    polymer.solution()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

        let mut polymer: Polymer = input.parse().unwrap();
        polymer.nth(9);
        assert_eq!(polymer.solution(), 1749 - 161);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 3058);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 3447389044530);
    }
}
