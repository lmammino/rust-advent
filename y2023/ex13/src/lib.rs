use models::Map;

mod models;

pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|raw_map| {
            let map: Map = raw_map.parse().unwrap();
            let vertical_reflection = match map.find_vertical_reflection() {
                Some(col) => col + 1,
                None => 0,
            };
            let horizontal_reflection = match map.find_horizontal_reflection() {
                Some(row) => row + 1,
                None => 0,
            };

            vertical_reflection + horizontal_reflection * 100
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|raw_map| {
            let map: Map = raw_map.parse().unwrap();
            let vertical_reflection = match map.find_vertical_reflection_with_smudge() {
                Some(col) => col + 1,
                None => 0,
            };
            let horizontal_reflection = match map.find_horizontal_reflection_with_smudge() {
                Some(row) => row + 1,
                None => 0,
            };

            let result = vertical_reflection + horizontal_reflection * 100;

            println!(
                "{}\n{} (vert: {}, horiz: {})\n",
                raw_map, result, vertical_reflection, horizontal_reflection
            );

            result
        })
        .sum()
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
        assert_eq!(part2(INPUT), 0);
    }
}
