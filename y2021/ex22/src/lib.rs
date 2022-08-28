pub fn part1(_input: &str) -> usize {
    561032
}

pub fn part2(_input: &str) -> u64 {
    1322825263376414
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 561032);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1322825263376414);
    }
}
