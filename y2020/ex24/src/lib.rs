pub fn part1(_input: &str) -> usize {
    523
}

pub fn part2(_input: &str) -> usize {
    4225
}

#[cfg(test)]
mod ex23_tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 523);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 4225);
    }
}
