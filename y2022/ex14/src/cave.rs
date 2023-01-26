use std::{collections::HashMap, str::FromStr};

use nom::{bytes::complete::tag, multi::separated_list1, sequence::separated_pair, IResult};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(crate) struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn down_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn down_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }

    fn try_move(&self, cave: &Cave) -> Option<Pos> {
        let down = self.down();
        let down_left = self.down_left();
        let down_right = self.down_right();

        let mut res: Option<Pos> = None;
        if !cave.tiles.contains_key(&down) {
            res = Some(down)
        } else if !cave.tiles.contains_key(&down_left) {
            res = Some(down_left)
        } else if !cave.tiles.contains_key(&down_right) {
            res = Some(down_right)
        }

        match (res, cave.floor_level) {
            (Some(pos), None) => Some(pos),
            (Some(next_pos), Some(floor_level)) => {
                if floor_level == next_pos.y {
                    None
                } else {
                    Some(next_pos)
                }
            }
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Block {
    Wall,
    Sand,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct Cave {
    pub tiles: HashMap<Pos, Block>,
    pub sand_emitter: Pos,
    pub floor_level: Option<usize>,
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    let (input, num) = nom::character::complete::digit1(input)?;
    Ok((input, num.parse().unwrap()))
}

fn parse_pos(input: &str) -> IResult<&str, Pos> {
    let (input, (x, y)) = separated_pair(parse_usize, tag(","), parse_usize)(input)?;
    Ok((input, Pos { x, y }))
}

fn parse_path(input: &str) -> IResult<&str, Vec<Pos>> {
    let (input, path) = separated_list1(tag(" -> "), parse_pos)(input)?;
    Ok((input, path))
}

fn parse_paths(input: &str) -> impl Iterator<Item = Vec<Pos>> + '_ {
    input.split('\n').map(|s| parse_path(s).unwrap().1)
}

fn expand_path(path: Vec<Pos>, tiles: &mut HashMap<Pos, Block>) {
    for (a, b) in path.iter().zip(path.iter().skip(1)) {
        let (x1, y1) = (a.x, a.y);
        let (x2, y2) = (b.x, b.y);
        if x1 == x2 {
            let (min_y, max_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            for y in min_y..=max_y {
                tiles.insert(Pos { x: x1, y }, Block::Wall);
            }
        } else {
            let (min_x, max_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            for x in min_x..=max_x {
                tiles.insert(Pos { x, y: y1 }, Block::Wall);
            }
        }
    }
}

impl FromStr for Cave {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = HashMap::new();
        let sand_emitter = Pos { x: 500, y: 0 };

        for path in parse_paths(s) {
            expand_path(path, &mut tiles);
        }

        Ok(Cave {
            tiles,
            sand_emitter,
            floor_level: None,
        })
    }
}

pub(crate) struct GrainMoves<'a> {
    pub pos: Pos,
    cave: &'a Cave,
}

impl<'a> Iterator for GrainMoves<'a> {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let next_pos = self.pos.try_move(self.cave)?;
        self.pos = next_pos.clone();
        Some(next_pos)
    }
}

impl Cave {
    pub(crate) fn next_grain(&self) -> GrainMoves {
        let pos = self.sand_emitter.clone();
        GrainMoves { pos, cave: self }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cave_from_str() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        let cave: Cave = input.parse().unwrap();
        assert_eq!(
            cave,
            Cave {
                sand_emitter: Pos { x: 500, y: 0 },
                floor_level: None,
                tiles: HashMap::from([
                    (Pos { x: 498, y: 4 }, Block::Wall),
                    (Pos { x: 498, y: 5 }, Block::Wall),
                    (Pos { x: 498, y: 6 }, Block::Wall),
                    (Pos { x: 497, y: 6 }, Block::Wall),
                    (Pos { x: 496, y: 6 }, Block::Wall),
                    (Pos { x: 503, y: 4 }, Block::Wall),
                    (Pos { x: 502, y: 4 }, Block::Wall),
                    (Pos { x: 502, y: 5 }, Block::Wall),
                    (Pos { x: 502, y: 6 }, Block::Wall),
                    (Pos { x: 502, y: 7 }, Block::Wall),
                    (Pos { x: 502, y: 8 }, Block::Wall),
                    (Pos { x: 502, y: 9 }, Block::Wall),
                    (Pos { x: 501, y: 9 }, Block::Wall),
                    (Pos { x: 500, y: 9 }, Block::Wall),
                    (Pos { x: 499, y: 9 }, Block::Wall),
                    (Pos { x: 498, y: 9 }, Block::Wall),
                    (Pos { x: 497, y: 9 }, Block::Wall),
                    (Pos { x: 496, y: 9 }, Block::Wall),
                    (Pos { x: 495, y: 9 }, Block::Wall),
                    (Pos { x: 494, y: 9 }, Block::Wall),
                ])
            }
        )
    }
}
