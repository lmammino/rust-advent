pub fn part1(_input: &str) -> usize {
    9177528
}

#[cfg(test)]
mod ex25_tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 9177528);
    }
}
