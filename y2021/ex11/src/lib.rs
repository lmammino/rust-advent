use std::{collections::HashSet, fmt::Display, str::FromStr};

const NEIGHBOURS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug)]
struct OctoGrid<const N: usize>([[u8; N]; N]);

impl<const N: usize> Display for OctoGrid<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0 {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }

        std::fmt::Result::Ok(())
    }
}

impl<const N: usize> FromStr for OctoGrid<N> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: [[u8; N]; N] = [[0; N]; N];
        for (row_id, line) in s.lines().enumerate() {
            for (col_id, c) in line.chars().enumerate() {
                grid[row_id][col_id] = c.to_digit(10).unwrap() as u8;
            }
        }

        Ok(OctoGrid(grid))
    }
}

impl<const N: usize> Iterator for OctoGrid<N> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut will_flash: Vec<(usize, usize)> = Vec::with_capacity(N * N);
        let mut flashed: HashSet<(usize, usize)> = HashSet::with_capacity(N * N);

        // increase all the counters by 1
        for (row_id, row) in self.0.iter_mut().enumerate() {
            for (col_id, cell) in row.iter_mut().enumerate() {
                if *cell < 10 {
                    *cell += 1;
                }
                if *cell == 10 {
                    will_flash.push((row_id, col_id));
                }
            }
        }

        while let Some((row_id, col_id)) = will_flash.pop() {
            if flashed.insert((row_id, col_id)) {
                // if this cell did not flash already
                for (delta_r, delta_c) in NEIGHBOURS {
                    let (new_row_id, new_col_id): (isize, isize) =
                        (row_id as isize + delta_r, col_id as isize + delta_c);

                    // makes sure the neighbour is inside the grid
                    let boundaries = 0..N as isize;
                    if boundaries.contains(&new_row_id) && boundaries.contains(&new_col_id) {
                        let (new_row_id, new_col_id) = (new_row_id as usize, new_col_id as usize);

                        if self.0[new_row_id][new_col_id] < 10 {
                            self.0[new_row_id][new_col_id] += 1;
                        }
                        if self.0[new_row_id][new_col_id] == 10 {
                            will_flash.push((new_row_id, new_col_id));
                        }
                    }
                }
            }
        }

        // makes sure that all the flashed ones are set to 0
        for (row_id, col_id) in flashed.iter() {
            self.0[*row_id][*col_id] = 0;
        }

        Some(flashed.len())
    }
}

pub fn part1(input: &str) -> usize {
    let grid: OctoGrid<10> = input.parse().unwrap();
    grid.take(100).sum()
}

pub fn part2(input: &str) -> usize {
    let mut grid: OctoGrid<10> = input.parse().unwrap();

    let mut rounds = 1_usize;
    // we check against 100 because we have a 10x10 grid (of 100 octopi)
    // each octopus can only flash once per round, and our grid returns how many octopi flashed
    while grid.next().unwrap() < 100 {
        rounds += 1
    }

    rounds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flashes() {
        let input = "11111
19991
19191
19991
11111";

        let step1 = "34543
40004
50005
40004
34543";

        let step2 = "45654
51115
61116
51115
45654";

        let mut grid: OctoGrid<5> = input.parse().unwrap();
        let after_step1: OctoGrid<5> = step1.parse().unwrap();
        let after_step2: OctoGrid<5> = step2.parse().unwrap();
        let flashed = grid.next().unwrap();
        assert_eq!(flashed, 9);
        assert_eq!(grid.0, after_step1.0);
        let flashed = grid.next().unwrap();
        assert_eq!(flashed, 0);
        assert_eq!(grid.0, after_step2.0);
    }

    #[test]
    fn test_part1_example() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

        let grid: OctoGrid<10> = input.parse().unwrap();
        assert_eq!(grid.take(100).sum::<usize>(), 1656);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 1723);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 327);
    }
}
