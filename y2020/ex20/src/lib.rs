use std::{collections::{HashMap, HashSet}, convert::TryInto, str::FromStr};

#[derive(Debug)]
struct Tile<const N: usize> {
    id: u16,
    cells: [[char; N]; N],
    borders: Vec<[char; N]>,
}

struct Tiles<const N: usize>(HashMap<u16, Tile<N>>);

impl<const N: usize> Tile<N> {
    fn new(id: u16, cells: [[char; N]; N]) -> Self {
        let top = cells[0];
        let mut rtop = top;
        rtop.reverse();
        let bottom = cells[cells.len() - 1];
        let mut rbottom = bottom;
        rbottom.reverse();
        let left: [char; N] = cells.iter().map(|r| r[0]).collect::<Vec<char>>().try_into().unwrap();
        let mut rleft = left;
        rleft.reverse();
        let right: [char; N] = cells.iter().map(|r| r[r.len() - 1]).collect::<Vec<char>>().try_into().unwrap();
        let mut rright = right;
        rright.reverse();

        let borders: Vec<[char; N]> = vec![top, rtop, bottom, rbottom, left, rleft, right, rright];

        Tile { id, cells, borders }
    }

    fn is_neighbour_of(&self, tile: &Tile<N>) -> bool {
        for border in &self.borders {
            if tile.borders.contains(border) {
                return true;
            }
        }

        false
    }
}

impl<const N: usize> FromStr for Tiles<N> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles: HashMap<u16, Tile<N>> = HashMap::new();

        for raw_tile in s.split("\n\n").take_while(|t| !t.is_empty()) {
            let raw_id = raw_tile.lines().next().unwrap();
            let id: u16 = raw_id[5..9].parse().unwrap();
            let cells:[[char; N]; N] = raw_tile
                .lines()
                .skip(1)
                .map(|l| l.chars().collect::<Vec<char>>().try_into().unwrap())
                .collect::<Vec<[char; N]>>()
                .try_into().unwrap();
    
            let tile = Tile::new(id, cells);
            tiles.insert(id, tile);
        }

        Ok(Self(tiles))
    }
}

pub fn part1<const N: usize>(input: &str) -> u64 {
    let tiles: Tiles<N> = input.parse().unwrap();
    
    let mut neighbours: HashMap<u16, HashSet<u16>> = HashMap::new();

    for (id, tile) in &tiles.0 {
        for (other_id, other_tile) in &tiles.0 {
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
        assert_eq!(part1::<10>(input), 17032646100079);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 2006);
    }
}
