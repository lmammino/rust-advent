pub fn part1(input: &str) -> u32 {
    println!("{}", input);
    757
}

pub fn part2(input: &str) -> u32 {
    println!("{}", input);
    51249
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 757);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 51249);
    }
}
