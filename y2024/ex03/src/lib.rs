use matcher::{Match, MatcherIter};
mod matcher;

pub fn part1(input: &str) -> u64 {
    let iter = MatcherIter::new(input);
    let mut sum = 0;

    for m in iter {
        if let Match::Mul(l, r) = m {
            sum += l * r;
        }
    }

    sum
}

pub fn part2(input: &str) -> u64 {
    let iter = MatcherIter::new(input);

    let mut sum = 0;
    let mut enabled = true;

    for m in iter {
        match m {
            Match::Do => enabled = true,
            Match::Dont => enabled = false,
            Match::Mul(l, r) => {
                if enabled {
                    sum += l * r;
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 185797128);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 89798695);
    }
}
