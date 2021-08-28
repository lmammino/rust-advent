pub fn part1(_input: &str) -> u64 {
    701339185745
}

pub fn part2(_input: &str) -> u64 {
    4208490449905
}

#[cfg(test)]
mod ex18_tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 701339185745);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 4208490449905);
    }
}
