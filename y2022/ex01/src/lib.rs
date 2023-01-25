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

pub fn part1(input: &str) -> u64 {
    GroupSumIter::new(input.lines()).max().unwrap()
}

pub fn part2(input: &str) -> u64 {
    top::<3>(GroupSumIter::new(input.lines())).iter().sum()
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
    fn test_part2() {
        assert_eq!(part2(INPUT), 208180);
    }
}
