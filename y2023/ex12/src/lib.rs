use models::{parse_entry, CountCache};

mod models;

pub fn part1(input: &str) -> usize {
    let cache = CountCache::new();
    input
        .lines()
        .map(|line| {
            let (_, record) = parse_entry(line).unwrap();
            record.count_solutions(&cache)
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let cache = CountCache::new();
    input
        .lines()
        .map(|line| {
            let (_, record) = parse_entry(line).unwrap();
            let record = record.unfold(5);
            record.count_solutions(&cache)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");
    const EXAMPLE_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 21);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 7407);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 525152);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 30568243604962);
    }
}
