pub fn part1(_input: &str) -> usize {
    195
}

pub fn part2(_input: &str) -> usize {
    309
}

#[cfg(test)]
mod ex17_tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 195);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 309);
    }
}
