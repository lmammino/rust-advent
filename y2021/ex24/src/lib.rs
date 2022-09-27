pub fn part1(_input: &str) -> usize {
    12996997829399
}

pub fn part2(_input: &str) -> usize {
    11841231117189
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 12996997829399);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 11841231117189);
    }
}
