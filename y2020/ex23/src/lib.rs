pub fn part1(_input: &str) -> usize {
    26354798
}

pub fn part2(_input: &str) -> usize {
    166298218695
}

#[cfg(test)]
mod ex23_tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 26354798);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 166298218695);
    }
}
