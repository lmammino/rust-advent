use itertools::Itertools;

/// Streaming iterator that groups and sums the elements of the underlying iterator.
/// Every time an empty line is found (or the end of the source iterator is reached) the current sum is returned.
struct GroupSumIter<'a, I: Iterator<Item = &'a str>> {
    iter: I,
    exausted: bool,
}

impl<'a, I: Iterator<Item = &'a str>> GroupSumIter<'a, I> {
    fn new(iter: I) -> Self {
        Self {
            iter,
            exausted: false,
        }
    }
}

impl<'a, I: Iterator<Item = &'a str>> Iterator for GroupSumIter<'a, I> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exausted {
            return None;
        }

        let mut current_sum = 0;
        loop {
            match self.iter.next() {
                Some(line) => {
                    if line.is_empty() {
                        return Some(current_sum);
                    }
                    current_sum += line.parse::<u64>().unwrap();
                }
                None => {
                    self.exausted = true;
                    return Some(current_sum);
                }
            }
        }
    }
}

/// extracts the top N items from the iterator
fn top<const N: usize>(iter: impl Iterator<Item = u64>) -> [u64; N] {
    let mut top = [0; N];
    for value in iter {
        for i in 0..N {
            if value > top[i] {
                top[i..].rotate_right(1);
                top[i] = value;
                break;
            }
        }
    }
    top
}

trait TopN<T> {
    fn top_n(self, n: usize) -> Vec<T>;
}

impl<T: PartialOrd, U: IntoIterator<Item = T>> TopN<T> for U {
    fn top_n(self, n: usize) -> Vec<T> {
        let mut top = Vec::with_capacity(n);
        for value in self.into_iter() {
            for i in 0..n {
                if let Some(top_value) = top.get(i) {
                    if value > *top_value {
                        top[i..].rotate_right(1);
                        top[i] = value;
                        break;
                    }
                } else {
                    top.push(value);
                    break;
                }
            }
        }
        top
    }
}

trait Top<T> {
    fn top<const N: usize>(self) -> [T; N];
}

impl<T: Default + Copy + PartialOrd, U: IntoIterator<Item = T>> Top<T> for U {
    fn top<const N: usize>(self) -> [T; N] {
        // Note: This implementation is not perfect:
        // - What if there are less than N items in the iterator?
        // - What if there are only negative numbers?
        // A more resilient implementation could have been done with [Option<T>; N]
        // but it would have been less ergonomic for our current use case...
        let mut top = [Default::default(); N];
        for value in self.into_iter() {
            for i in 0..N {
                let top_value = top[i];
                if value > top_value {
                    top[i..].rotate_right(1);
                    top[i] = value;
                    break;
                }
            }
        }
        top
    }
}

pub fn part1(input: &str) -> u64 {
    GroupSumIter::new(input.lines()).max().unwrap()
}

pub fn part1_classic(input: &str) -> u64 {
    let mut max = 0;
    for batch in input.split("\n\n") {
        let mut total = 0;
        for line in batch.lines() {
            let value = line.parse::<u64>().unwrap();
            total += value;
        }
        if total > max {
            max = total;
        }
    }
    max
}

pub fn part1_combinators(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|batch| {
            batch
                .lines()
                .map(|line| line.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> u64 {
    top::<3>(GroupSumIter::new(input.lines())).iter().sum()
}

pub fn part2_combinators_itertools(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|batch| {
            batch
                .lines()
                .map(|line| line.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum()
}

pub fn part2_combinators_no_sort(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|batch| {
            batch
                .lines()
                .map(|line| line.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .top_n(3)
        .iter()
        .sum()
}

pub fn part2_combinators_no_sort_const(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|batch| {
            batch
                .lines()
                .map(|line| line.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .top::<3>()
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 69912);
    }

    #[test]
    fn test_part1_classic() {
        assert_eq!(part1_classic(INPUT), 69912);
    }

    #[test]
    fn test_part1_combinators() {
        assert_eq!(part1_combinators(INPUT), 69912);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 208180);
    }

    #[test]
    fn test_part2_combinators() {
        assert_eq!(part2_combinators_itertools(INPUT), 208180);
    }

    #[test]
    fn test_part2_combinators_no_sort() {
        assert_eq!(part2_combinators_no_sort(INPUT), 208180);
    }

    #[test]
    fn test_part2_combinators_no_sort_const() {
        assert_eq!(part2_combinators_no_sort_const(INPUT), 208180);
    }
}
