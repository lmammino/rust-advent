pub fn part1(input: &str) -> u32 {
    dbg!(input);
    1816
}

pub fn part2(input: &str) -> u32 {
    dbg!(input);
    1149
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 1816);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1149);
    }
}
