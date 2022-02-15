pub fn part1(input: &str) -> i32 {
    let mut positions: Vec<i32> = input
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    positions.sort_unstable();
    let median = *positions.get(positions.len() / 2).unwrap();
    positions.iter().map(|x| (x - median).abs()).sum()
}

fn sum_n(n: i32) -> i32 {
    // Gauss's formula to sum first n numbers
    // n / 2 * (n + 1)
    n * (n + 1) / 2
}

pub fn part2(input: &str) -> i32 {
    let positions: Vec<i32> = input
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    let avg = positions.iter().sum::<i32>() / positions.len() as i32;
    positions.iter().map(|x| sum_n((x - avg).abs())).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 347011);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 98363777);
    }
}
