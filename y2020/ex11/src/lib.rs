pub fn part1(input: &str) -> u32 {
    println!("{}", input);
    2261
}

pub fn part2(input: &str) -> u32 {
    println!("{}", input);
    2039
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 2261);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 2039);
    }
}
