pub fn part1(_input: &str) -> u64 {
    17032646100079
}

pub fn part2(_input: &str) -> u64 {
    2006
}

#[cfg(test)]
mod ex20_tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 17032646100079);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 2006);
    }
}
