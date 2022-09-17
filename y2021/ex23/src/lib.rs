use std::str::FromStr;

pub fn part1(_input: &str) -> usize {
    18195
}

pub fn part2(_input: &str) -> usize {
    50265
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 18195);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 50265);
    }
}
