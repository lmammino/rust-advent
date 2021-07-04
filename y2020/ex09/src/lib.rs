use std::collections::{HashSet, VecDeque};

fn check(value: &u64, set: &HashSet<u64>) -> bool {
    for a in set {
        if a>value {
            continue;
        }
        let b = value - a;
        if set.contains(&b) {
            return true;
        }
    }
    false
}


pub fn part1(input: &str) -> u64 {
    let mut values = input
            .lines()
            .map(|l| {
                l.parse::<u64>()
                    .unwrap_or_else(|_| panic!("Cannot convert line '{:?}' to u64", l))
            });

    let mut idx: HashSet<u64> = HashSet::new();
    let mut queue: VecDeque<u64> = VecDeque::new();

    for _ in 0..25 {
        let next_value = values.next().expect("no enougth data to start");
        idx.insert(next_value);
        queue.push_back(next_value);
    }

    for value in values {
        if !check(&value, &idx) {
            return value;
        }
        let remove = queue.pop_front().unwrap();
        queue.push_back(value);
        idx.remove(&remove);
        idx.insert(value);
    }

    0
}

pub fn part2(input: &str) -> u64 {
    let to_find = part1(input);
    let values: Vec<u64> = input
            .lines()
            .map(|l| {
                l.parse::<u64>()
                    .unwrap_or_else(|_| panic!("Cannot convert line '{:?}' to u64", l))
            }).collect();
    for window in 2..values.len() {
        for start in 0..(values.len()-window) {
            let segment = &values[start..start+window];
            let sum: u64 = segment.iter().sum();
            if to_find==sum {
                let min = segment.iter().min().unwrap();
                let max = segment.iter().max().unwrap();
                return min+max;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 1492208709);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 238243506);
    }
}
