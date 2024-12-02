use std::collections::{BTreeMap, BinaryHeap};

fn parse_line(line: &str) -> (usize, usize) {
    let mut parts = line.split_whitespace();
    let left: usize = parts.next().unwrap().parse().unwrap();
    let right: usize = parts.next().unwrap().parse().unwrap();
    (left, right)
}

pub fn part1(input: &str) -> usize {
    let mut left_heap = BinaryHeap::new();
    let mut right_heap = BinaryHeap::new();

    input.lines().map(parse_line).for_each(|(left, right)| {
        left_heap.push(left);
        right_heap.push(right);
    });

    let left_sorted = left_heap.into_sorted_vec();
    let right_sorted = right_heap.into_sorted_vec();

    left_sorted
        .iter()
        .zip(right_sorted.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut left_list = Vec::new();
    let mut right_list_counts = BTreeMap::new();

    input.lines().map(parse_line).for_each(|(left, right)| {
        left_list.push(left);
        let entry = right_list_counts.entry(right).or_insert(0);
        *entry += 1;
    });

    left_list
        .iter()
        .map(|l| {
            let frequency = right_list_counts.get(l).unwrap_or(&0);
            l * frequency
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    const EXAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 11);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2113135);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
