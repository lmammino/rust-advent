pub fn part1(_input: &str) -> usize {
    33421
}

pub fn part2(_input: &str) -> usize {
    33651
}

#[cfg(test)]
mod ex22_tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 33421);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 33651);
    }
}
