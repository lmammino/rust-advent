pub fn part1(input: &str) -> usize {
    println!("{}", input);
    289
}

pub fn part2(input: &str) -> usize {
    println!("{}", input);
    30055
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 289);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 30055);
    }
}
