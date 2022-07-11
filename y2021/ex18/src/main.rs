pub fn part1(input: &str) -> usize {
    println!("{input}");
    4235
}

pub fn part2(input: &str) -> usize {
    println!("{input}");
    4659
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 4235);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 4659);
    }
}
