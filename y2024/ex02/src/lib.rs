fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn is_safe(reports: &[u32]) -> bool {
    let is_ascending = reports.windows(2).all(|w| w[0] <= w[1]);
    let is_descending = reports.windows(2).all(|w| w[0] >= w[1]);
    if !is_ascending && !is_descending {
        return false;
    }

    for w in reports.windows(2) {
        let a = w[0];
        let b = w[1];
        let diff = a.abs_diff(b);
        if !(1..=3).contains(&diff) {
            return false;
        }
    }

    true
}

fn is_safe_with_dampening(reports: &[u32]) -> bool {
    is_safe(reports)
        || (0..reports.len()).any(|i| {
            let mut reports_no_i = reports.to_vec();
            reports_no_i.remove(i);
            is_safe(&reports_no_i)
        })
}

pub fn part1(input: &str) -> usize {
    input.lines().map(parse_line).filter(|r| is_safe(r)).count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(parse_line)
        .filter(|r| {
            println!("{:?}: {:?}", r, is_safe_with_dampening(r));
            is_safe_with_dampening(r)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");
    const EXAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 2);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 371);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 426);
    }
}
