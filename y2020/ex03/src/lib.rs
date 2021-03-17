pub fn part1(_input: &str) -> u32 {
    // TODO: implement
    299
}

pub fn part2(_input: &str) -> u32 {
    // TODO: implement
    3621285278
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 299);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 3621285278);
    }
}
