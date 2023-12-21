use std::fmt::Display;

pub type Position = (usize, usize);

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Plot,
    Rock,
}

#[derive(Debug, Clone)]
pub struct Map<const W: usize, const H: usize> {
    pub map: [[Tile; W]; H],
    pub start: Position,
}

impl<const W: usize, const H: usize> Map<W, H> {
    pub fn new(input: &str) -> Self {
        let mut map = [[Tile::Plot; W]; H];
        let mut start = (0, 0);
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => map[y][x] = Tile::Plot,
                    '#' => map[y][x] = Tile::Rock,
                    'S' => {
                        map[y][x] = Tile::Plot;
                        start = (x, y);
                    }
                    _ => panic!("Invalid character '{}' found at ({}, {})", c, x, y),
                };
            }
        }
        Self { map, start }
    }
}

impl<const W: usize, const H: usize> Display for Map<W, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.map.iter() {
            for tile in line.iter() {
                match tile {
                    Tile::Plot => write!(f, ".")?,
                    Tile::Rock => write!(f, "#")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
