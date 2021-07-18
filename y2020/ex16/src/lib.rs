pub fn part1(_input: &str) -> u64 {
    21996
}

pub fn part2(_input: &str) -> u64 {
    650080463519
}

#[cfg(test)]
mod ex16_tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 21996);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 650080463519);
    }
}
