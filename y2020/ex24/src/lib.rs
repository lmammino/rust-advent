use std::{
    collections::HashSet,
    str::{Chars, FromStr},
};

const DELTAS: [(i32, i32); 6] = [(2, 0), (-2, 0), (1, 1), (-1, -1), (-1, 1), (1, -1)];

#[derive(Hash, PartialEq, Eq, Clone, Default)]
struct Tile {
    x: i32,
    y: i32,
}

struct CommandParser<'a> {
    chars: Chars<'a>,
}

impl<'a> CommandParser<'a> {
    fn new(chars: Chars<'a>) -> Self {
        CommandParser { chars }
    }
}

impl Iterator for CommandParser<'_> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.next() {
            Some('e') => Some((2, 0)),
            Some('w') => Some((-2, 0)),
            Some('n') => match self.chars.next() {
                Some('e') => Some((1, -1)),
                Some('w') => Some((-1, -1)),
                _ => None,
            },
            Some('s') => match self.chars.next() {
                Some('e') => Some((1, 1)),
                Some('w') => Some((-1, 1)),
                _ => None,
            },
            _ => None,
        }
    }
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = CommandParser::new(s.chars())
            .fold((0, 0), |acc, curr| (acc.0 + curr.0, acc.1 + curr.1));
        Ok(Tile { x, y })
    }
}

impl Tile {
    fn neighbours(&self) -> [Tile; 6] {
        let mut ret: [Tile; 6] = Default::default();

        for (i, (dx, dy)) in DELTAS.iter().enumerate() {
            ret[i] = Tile {
                x: self.x + dx,
                y: self.y + dy,
            };
        }

        ret
    }
}

struct Lobby {
    tiles: HashSet<Tile>,
}

impl Lobby {
    fn neighbours(&self, tile: &Tile) -> usize {
        tile.neighbours()
            .into_iter()
            .filter(|tile| self.tiles.contains(tile))
            .count()
    }
}

impl FromStr for Lobby {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles: HashSet<Tile> = Default::default();
        for line in s.lines() {
            let tile: Tile = line.parse().unwrap();
            if tiles.contains(&tile) {
                tiles.remove(&tile);
            } else {
                tiles.insert(tile);
            }
        }
        Ok(Lobby { tiles })
    }
}

impl Iterator for Lobby {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new_tiles: HashSet<Tile> = Default::default();

        let mut to_check = self.tiles.clone();
        for tile in &self.tiles {
            for n_tile in tile.neighbours() {
                to_check.insert(n_tile);
            }
        }

        for tile in to_check {
            let n_neighbours = self.neighbours(&tile);
            if n_neighbours == 2 || (n_neighbours == 1 && self.tiles.contains(&tile)) {
                new_tiles.insert(tile);
            }
        }

        self.tiles = new_tiles;
        Some(self.tiles.len())
    }
}

pub fn part1(input: &str) -> usize {
    input.parse::<Lobby>().unwrap().tiles.len()
}

pub fn part2(input: &str) -> usize {
    input.parse::<Lobby>().unwrap().nth(99).unwrap()
}

#[cfg(test)]
mod ex24_tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 523);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 4225);
    }
}
