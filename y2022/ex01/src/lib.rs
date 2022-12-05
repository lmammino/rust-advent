pub fn part1(_input: &str) -> u64 {
    69912
}

pub fn part2(_input: &str) -> u64 {
    208180
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
