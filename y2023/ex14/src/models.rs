use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Wall,
    Stone,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Wall,
            'O' => Self::Stone,
            _ => panic!("Invalid tile: {}", c),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Empty => '.',
            Self::Wall => '#',
            Self::Stone => 'O',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Map {
    pub data: Vec<Vec<Tile>>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();

        Ok(Self { data })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.data {
            for tile in line {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    pub fn tilt_north(&mut self) {
        for y in 1..self.data.len() {
            for x in 0..self.data[y].len() {
                if self.data[y][x] == Tile::Stone {
                    // rolls up until reaches the edge, or hits a wall or a stone
                    let mut curr_y = y;
                    loop {
                        let can_move_up = curr_y > 0 && self.data[curr_y - 1][x] == Tile::Empty;

                        if can_move_up {
                            self.data[curr_y - 1][x] = Tile::Stone;
                            self.data[curr_y][x] = Tile::Empty;
                            curr_y -= 1;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    pub fn rotate_right(&mut self) {
        let h = self.data.len();
        let mut new_data: Vec<Vec<Tile>> =
            vec![vec![Tile::Empty; self.data.len()]; self.data[0].len()];
        for y in 0..self.data.len() {
            for (x, _) in self.data[y].iter().enumerate() {
                new_data[x][h - 1 - y] = self.data[y][x];
            }
        }
        self.data = new_data;
    }

    pub fn cycle(&mut self) {
        self.tilt_north();
        self.rotate_right();
        self.tilt_north();
        self.rotate_right();
        self.tilt_north();
        self.rotate_right();
        self.tilt_north();
        self.rotate_right();
    }

    pub fn total_load(&self) -> usize {
        self.data
            .iter()
            .rev()
            .enumerate()
            .map(|(weight, line)| {
                let num_stones = line.iter().filter(|&&tile| tile == Tile::Stone).count();
                num_stones * (weight + 1)
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_tilt_north() {
        let expected = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

        let mut map: Map = EXAMPLE_INPUT.parse().unwrap();
        let expected: Map = expected.parse().unwrap();

        map.tilt_north();
        assert_eq!(map, expected);
        assert_eq!(map.total_load(), 136);
    }

    #[test]
    fn test_rotate_right() {
        let map = "O.#O.#
.#..#O
#.OO.#";

        let expected = "#.O
.#.
O.#
O.O
.#.
#O#";

        let mut map: Map = map.parse().unwrap();
        let expected: Map = expected.parse().unwrap();

        map.rotate_right();
        assert_eq!(map, expected);
    }

    #[test]
    fn test_cycle() {
        let expected_1_cycle = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";

        let expected_2_cycles = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O";

        let expected_3_cycles = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";

        let mut map: Map = EXAMPLE_INPUT.parse().unwrap();
        let expected_1_cycle: Map = expected_1_cycle.parse().unwrap();
        let expected_2_cycles: Map = expected_2_cycles.parse().unwrap();
        let expected_3_cycles: Map = expected_3_cycles.parse().unwrap();

        map.cycle();
        assert_eq!(map, expected_1_cycle);
        map.cycle();
        assert_eq!(map, expected_2_cycles);
        map.cycle();
        assert_eq!(map, expected_3_cycles);
    }
}
