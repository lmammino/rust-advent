use models::Map;

mod models;

fn solve(input: &str, errors: usize) -> usize {
    input
        .split("\n\n")
        .map(|raw_map| {
            let map: Map = raw_map.parse().unwrap();
            let horizontal_reflection = map.find_reflection(errors).unwrap_or(0);
            let vertical_reflection = map.transpose().find_reflection(errors).unwrap_or(0);

            vertical_reflection + horizontal_reflection * 100
        })
        .sum()
}

pub fn part1(input: &str) -> usize {
    solve(input, 0)
}

pub fn part2(input: &str) -> usize {
    solve(input, 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");
    const EXAMPLE_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 405);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 30575);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 400);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 37478);
    }
}
