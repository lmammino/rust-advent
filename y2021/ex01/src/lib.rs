use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .tuple_windows()
        .filter(|(prev, next)| next > prev)
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .tuple_windows()
        .map(|(v1, v2, v3)| v1 + v2 + v3)
        .tuple_windows()
        .filter(|(prev, next)| next > prev)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "199
200
208
210
200
207
240
269
260
263";
        assert_eq!(part1(input), 7);
    }

    #[test]
    fn test_part2_example() {
        let input = "607
618
618
617
647
716
769
792";
        assert_eq!(part2(input), 5);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 1292);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1262);
    }
}
