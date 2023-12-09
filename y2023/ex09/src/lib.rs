use models::parse_history;

mod models;

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let (_, mut history) = parse_history(line).unwrap();
            history.expand();
            history.extrapolate_right()
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let (_, mut history) = parse_history(line).unwrap();
            history.expand();
            history.extrapolate_left()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");
    const EXAMPLE_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 114);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1731106378);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1087);
    }
}
