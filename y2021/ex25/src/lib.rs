use std::{fmt::Display, mem, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Right,
    Down,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            'v' => Cell::Down,
            '>' => Cell::Right,
            _ => panic!("Invalid cell: {c}"),
        }
    }
}

#[derive(Debug)]
struct Grid<const W: usize, const H: usize> {
    cells: [[Option<Cell>; W]; H],
}

impl<const W: usize, const H: usize> FromStr for Grid<W, H> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells = [[None; W]; H];
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                cells[y][x] = match c {
                    '.' => None,
                    _ => Some(c.into()),
                };
            }
        }
        Ok(Grid { cells })
    }
}

impl<const W: usize, const H: usize> Display for Grid<W, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for cell in row {
                match cell {
                    None => write!(f, ".")?,
                    Some(Cell::Right) => write!(f, ">")?,
                    Some(Cell::Down) => write!(f, "v")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const W: usize, const H: usize> Grid<W, H> {
    fn new() -> Self {
        Self {
            cells: [[None; W]; H],
        }
    }

    fn step(&self, final_grid: &mut Grid<W, H>) -> bool {
        let mut changed = false;
        let mut intermediate_grid: Self = Self::new();

        // moves everything right first (intermediate grid)
        for y in 0..H {
            for x in 0..W {
                final_grid.cells[y][x] = None;
                if let Some(Cell::Right) = self.cells[y][x] {
                    let next_cell = self.cells[y][(x + 1) % W];
                    if next_cell.is_none() {
                        intermediate_grid.cells[y][(x + 1) % W] = Some(Cell::Right);
                        changed = true;
                    } else {
                        intermediate_grid.cells[y][x] = Some(Cell::Right);
                    }
                } else if let Some(Cell::Down) = self.cells[y][x] {
                    // copy the cell unchanged
                    intermediate_grid.cells[y][x] = self.cells[y][x];
                }
            }
        }

        // moves everything down (final grid)
        for y in 0..H {
            for x in 0..W {
                if let Some(Cell::Down) = intermediate_grid.cells[y][x] {
                    let next_cell = intermediate_grid.cells[(y + 1) % H][x];
                    if next_cell.is_none() {
                        final_grid.cells[(y + 1) % H][x] = Some(Cell::Down);
                        changed = true;
                    } else {
                        final_grid.cells[y][x] = Some(Cell::Down);
                    }
                } else if let Some(Cell::Right) = intermediate_grid.cells[y][x] {
                    // copy the cell unchanged
                    final_grid.cells[y][x] = intermediate_grid.cells[y][x];
                }
            }
        }

        changed
    }
}

pub fn part1(input: &str) -> usize {
    let mut grid1: Grid<139, 137> = input.parse().unwrap();
    let mut grid2 = Grid::<139, 137>::new();
    let mut g1 = &mut grid1;
    let mut g2 = &mut grid2;
    let mut counter = 0;
    loop {
        counter += 1;
        let changed = g1.step(g2);
        mem::swap(&mut g1, &mut g2);

        if !changed {
            break;
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_example_grid() {
        use Cell::*;
        let input = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
        let grid: Grid<10, 9> = input.parse().unwrap();
        assert_eq!(
            grid.cells[0],
            [
                Some(Down),
                None,
                None,
                None,
                Some(Right),
                Some(Right),
                None,
                Some(Down),
                Some(Down),
                Some(Right)
            ]
        );
    }

    #[test]
    fn test_example() {
        let input = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
        let mut grid1: Grid<10, 9> = input.parse().unwrap();
        let mut grid2 = Grid::<10, 9>::new();
        let mut g1 = &mut grid1;
        let mut g2 = &mut grid2;

        let mut counter = 0;
        loop {
            counter += 1;
            println!("{counter} ---\n\n {g1}\n");
            let changed = g1.step(g2);
            mem::swap(&mut g1, &mut g2);

            if !changed {
                break;
            }
        }
        assert_eq!(counter, 58);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 507);
    }
}
