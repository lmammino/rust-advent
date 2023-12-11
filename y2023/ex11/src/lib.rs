use std::collections::HashSet;

type Galaxy = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Universe(HashSet<Galaxy>);

impl Universe {
    fn find_empty_space(&self) -> (Vec<usize>, Vec<usize>) {
        let mut xs: Vec<usize> = self.0.iter().map(|(x, _y)| *x).collect();
        xs.sort();
        let mut ys: Vec<usize> = self.0.iter().map(|(_x, y)| *y).collect();
        ys.sort();

        let mut missing_xs = vec![];
        for i in 0..xs.len() - 1 {
            for j in (xs[i] + 1)..(xs[i + 1]) {
                missing_xs.push(j);
            }
        }

        let mut missing_ys = vec![];
        for i in 0..ys.len() - 1 {
            for j in (ys[i] + 1)..(ys[i + 1]) {
                missing_ys.push(j);
            }
        }

        (missing_xs, missing_ys)
    }

    fn expand(&self, expansion_rate: usize) -> Self {
        let mut new_universe = HashSet::new();

        let (missing_xs, missing_ys) = self.find_empty_space();
        for galaxy in self.0.iter() {
            let (x, y) = galaxy;
            let num_gaps_left = missing_xs
                .iter()
                .filter(|missing_x| **missing_x < *x)
                .count();
            let num_gaps_above = missing_ys
                .iter()
                .filter(|missing_y| **missing_y < *y)
                .count();
            let new_galaxy = (
                x + num_gaps_left * (expansion_rate - 1),
                y + num_gaps_above * (expansion_rate - 1),
            );
            new_universe.insert(new_galaxy);
        }

        Universe(new_universe)
    }

    fn galaxies_combined(&self) -> impl Iterator<Item = (&Galaxy, &Galaxy)> + '_ {
        let combined_galaxies = self.0.iter().enumerate().flat_map(|(i, galaxy1)| {
            let inner_iter = self
                .0
                .iter()
                .skip(i + 1)
                .map(move |galaxy2| (galaxy1, galaxy2));

            inner_iter.clone()
        });

        combined_galaxies
    }
}

fn parse_universe(input: &str) -> Universe {
    let mut set = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                set.insert((x, y));
            }
        }
    }
    Universe(set)
}

fn solve(input: &str, expansion_rate: usize) -> usize {
    let mut universe = parse_universe(input);
    universe = universe.expand(expansion_rate);
    universe
        .galaxies_combined()
        .map(|((xa, ya), (xb, yb))| {
            let x_diff = (*xa as isize - *xb as isize).abs();
            let y_diff = (*ya as isize - *yb as isize).abs();
            let distance = x_diff + y_diff;
            distance as usize
        })
        .sum()
}

pub fn part1(input: &str) -> usize {
    solve(input, 2)
}

pub fn part2(input: &str) -> usize {
    solve(input, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");
    const EXAMPLE_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_parse() {
        let universe = parse_universe(EXAMPLE_INPUT);
        assert_eq!(
            universe,
            Universe(
                [
                    (3, 0),
                    (7, 1),
                    (0, 2),
                    (6, 4),
                    (1, 5),
                    (9, 6),
                    (7, 8),
                    (0, 9),
                    (4, 9)
                ]
                .into_iter()
                .collect()
            )
        );
    }

    #[test]
    fn test_find_empty_space() {
        let universe = parse_universe(EXAMPLE_INPUT);
        assert_eq!(universe.find_empty_space(), (vec![2, 5, 8], vec![3, 7]));
    }

    #[test]

    fn test_expand() {
        let universe = parse_universe(EXAMPLE_INPUT);
        let expanded = universe.expand(2);
        assert_eq!(
            expanded,
            Universe(
                [
                    (9, 10),
                    (4, 0),
                    (12, 7),
                    (5, 11),
                    (0, 2),
                    (1, 6),
                    (9, 1),
                    (8, 5),
                    (0, 11)
                ]
                .into_iter()
                .collect()
            )
        );
    }

    #[test]
    fn test_combined() {
        let universe = parse_universe(EXAMPLE_INPUT);
        let combined_galaxies = universe.galaxies_combined();
        assert_eq!(combined_galaxies.count(), 36);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 374);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 9536038);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve(EXAMPLE_INPUT, 10), 1030);
        assert_eq!(solve(EXAMPLE_INPUT, 100), 8410);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 447744640566);
    }
}
