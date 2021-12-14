mod charlist;

use charlist::*;
use std::{collections::HashMap, fmt::Debug, str::FromStr};

#[derive(Debug)]
struct Polymer {
    seq: CharList,
    rules: HashMap<String, char>,
}

impl FromStr for Polymer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_seq, raw_rules) = s.split_once("\n\n").unwrap();
        let seq = raw_seq.into();
        let mut rules: HashMap<String, char> = Default::default();
        for line in raw_rules.lines() {
            let (pair, subst) = line.split_once(" -> ").unwrap();
            let subst = subst.chars().next().unwrap();
            rules.insert(String::from(pair), subst);
        }

        Ok(Polymer { seq, rules })
    }
}

impl Polymer {
    fn grow(&mut self) {
        let mut current_index = Some(0_usize);
        while let Some(index) = current_index {
            if let Some(current_node) = self.seq.get(index) {
                if let Some(next_index) = current_node.next {
                    if let Some(next_node) = self.seq.next(index) {
                        let rule = format!("{}{}", current_node.value, next_node.value);
                        let new_el = self.rules.get(&rule).unwrap();
                        let new_node_index = self.seq.insert_value_with_next(*new_el, next_index);
                        self.seq.update_next(index, new_node_index);
                        current_index = Some(next_index);
                        continue;
                    }
                }
            }
            current_index = None;
        }
    }

    fn min_max_elements(&self) -> ((char, usize), (char, usize)) {
        let mut el_count: HashMap<char, usize> = Default::default();

        let mut min_el = '_';
        let mut max_el = '_';

        for el in self.seq.chars() {
            let count = {
                let count = el_count.entry(el).or_default();
                *count += 1;
                *count
            };

            // update min
            if min_el == '_' {
                min_el = el;
            } else {
                let min_el_count = *(el_count.get(&min_el).unwrap());
                if count < min_el_count {
                    min_el = el;
                }
            }

            // update min
            if max_el == '_' {
                max_el = el;
            } else {
                let max_el_count = el_count.get(&max_el).unwrap();
                if count > *max_el_count {
                    max_el = el;
                }
            }
        }

        (
            (min_el, *el_count.get(&min_el).unwrap()),
            (max_el, *el_count.get(&max_el).unwrap()),
        )
    }
}

pub fn part1(input: &str) -> usize {
    let mut polymer: Polymer = input.parse().unwrap();

    for _ in 0..10 {
        polymer.grow();
    }

    let ((_, min), (_, max)) = polymer.min_max_elements();

    max - min
}

pub fn part2(input: &str) -> usize {
    let mut polymer: Polymer = input.parse().unwrap();

    for i in 0..40 {
        // TODO: too slow, needs linked lists
        println!("{}", i);
        polymer.grow();
    }

    let ((_, min), (_, max)) = polymer.min_max_elements();

    max - min
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
        // step 1
        polymer.grow();
        assert_eq!(polymer.seq.to_string(), "NCNBCHB");
        // step 2
        polymer.grow();
        assert_eq!(polymer.seq.to_string(), "NBCCNBBBCBHCB");
        // step 3
        polymer.grow();
        assert_eq!(polymer.seq.to_string(), "NBBBCNCCNBBNBNBBCHBHHBCHB");
        // step 4
        polymer.grow();
        assert_eq!(
            polymer.seq.to_string(),
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
        );
        // ... step 10
        polymer.grow(); // 5
        polymer.grow(); // 6
        polymer.grow(); // 7
        polymer.grow(); // 8
        polymer.grow(); // 9
        polymer.grow(); // 10

        assert_eq!(polymer.seq.to_string().len(), 3073);
        assert_eq!(polymer.min_max_elements(), (('H', 161), ('B', 1749)));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 3058);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 3058);
    }
}
