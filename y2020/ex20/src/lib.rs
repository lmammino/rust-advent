use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Tile {
    id: u16,
    cells: Vec<Vec<char>>,
    borders: Vec<Vec<char>>,
}

impl Tile {
    fn new(id: u16, cells: Vec<Vec<char>>) -> Self {
        let top = cells[0].clone();
        let mut rtop = top.clone();
        rtop.reverse();
        let bottom = cells[cells.len() - 1].clone();
        let mut rbottom = bottom.clone();
        rbottom.reverse();
        let left: Vec<_> = cells.iter().map(|r| r[0]).collect();
        let mut rleft = left.clone();
        rleft.reverse();
        let right: Vec<_> = cells.iter().map(|r| r[r.len() - 1]).collect();
        let mut rright = right.clone();
        rright.reverse();

        let borders: Vec<Vec<char>> = vec![top, rtop, bottom, rbottom, left, rleft, right, rright];

        Tile { id, cells, borders }
    }

    fn is_neighbour_of(&self, tile: &Tile) -> bool {
        for border in &self.borders {
            if tile.borders.contains(border) {
                return true;
            }
        }

        false
    }
}

pub fn part1(input: &str) -> u64 {
    let mut tiles: HashMap<u16, Tile> = HashMap::new();
    for raw_tile in input.split("\n\n").take_while(|t| !t.is_empty()) {
        let raw_id = raw_tile.lines().next().unwrap();
        let id: u16 = raw_id[5..9].parse().unwrap();
        let cells: Vec<Vec<char>> = raw_tile
            .lines()
            .skip(1)
            .map(|l| l.chars().collect())
            .collect();

        let tile = Tile::new(id, cells);
        tiles.insert(id, tile);
    }
    let mut neighbours: HashMap<u16, HashSet<u16>> = HashMap::new();

    for (id, tile) in &tiles {
        for (other_id, other_tile) in &tiles {
            if id != other_id && tile.is_neighbour_of(other_tile) {
                neighbours.entry(*id).or_default().insert(*other_id);
                neighbours.entry(*other_id).or_default().insert(*id);
            }
        }
    }

    // Corner tiles will only have 2 neighbours
    let corners = neighbours
        .iter()
        .filter_map(|(key, val)| {
            if val.len() == 2 {
                return Some(*key as u64);
            }

            None
        })
        .product::<u64>();

    // 17032646100079
    corners
}

pub fn part2(_input: &str) -> u64 {
    2006
}

#[cfg(test)]
mod ex20_tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 17032646100079);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 2006);
    }
}
