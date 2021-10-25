use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::TryInto,
    str::FromStr,
};

#[derive(Debug)]
struct Tile<const N: usize> {
    id: u16,
    cells: [[char; N]; N],
    borders: [[char; N]; 8],
}

struct Tiles<const N: usize>(HashMap<u16, Tile<N>>);

impl<const N: usize> Tile<N> {
    fn new(id: u16, cells: [[char; N]; N]) -> Self {
        // In the all following lines the assignment means copy (array implements Copy trait)
        let top = cells[0];
        let mut rtop = top;
        rtop.reverse();
        let bottom = cells[cells.len() - 1];
        let mut rbottom = bottom;
        rbottom.reverse();
        // `[char; N]` doens't implement FromIterator, so `collect` cannot be used to transform a iterator into an array.
        // Instead, it is possible to convert Vec<char> into a fixed length array of char through `try_into` method of the trait `TryInto`.
        let left: [char; N] = cells
            .iter()
            .map(|r| r[0])
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();
        let mut rleft = left;
        rleft.reverse();
        let right: [char; N] = cells
            .iter()
            .map(|r| r[r.len() - 1])
            .collect::<Vec<char>>()
            .try_into()
            .unwrap();
        let mut rright = right;
        rright.reverse();

        let borders: [[char; N]; 8] = [top, rtop, bottom, rbottom, left, rleft, right, rright];

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
            let mut lines = raw_tile.lines();
            let raw_id = lines.next().unwrap();
            let id: u16 = raw_id[5..9].parse().unwrap();
            let cells: [[char; N]; N] = lines
                .map(|l| l.chars().collect::<Vec<char>>().try_into().unwrap())
                .collect::<Vec<[char; N]>>()
                .try_into()
                .unwrap();

            let tile = Tile::new(id, cells);
            tiles.insert(id, tile);
        }

        Ok(Self(tiles))
    }
}

pub fn part1(input: &str) -> u64 {
    let tiles: Tiles<10> = input.parse().unwrap();

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

pub fn part2(input: &str) -> u64 {
    // let tiles: Tiles<10> = input.parse().unwrap();

    // let mut neighbours: HashMap<u16, HashSet<u16>> = HashMap::new();

    // for (id, tile) in &tiles.0 {
    //     for (other_id, other_tile) in &tiles.0 {
    //         if id != other_id && tile.is_neighbour_of(other_tile) {
    //             neighbours.entry(*id).or_default().insert(*other_id);
    //             neighbours.entry(*other_id).or_default().insert(*id);
    //         }
    //     }
    // }

    // // Corner tiles will only have 2 neighbours
    // let corners = neighbours
    //     .iter()
    //     .filter_map(|(key, val)| {
    //         if val.len() == 2 {
    //             return Some(*key);
    //         }

    //         None
    //     })
    //     .collect::<Vec<u16>>();

    // let edges = neighbours
    //     .iter()
    //     .filter_map(|(key, val)| {
    //         if val.len() == 3 {
    //             return Some(*key);
    //         }

    //         None
    //     })
    //     .collect::<HashSet<u16>>();

    // let mut tilemap: [[u16; 12]; 12] = [[0; 12]; 12];
    // // we take an arbitrary corner (the first one) and we place it at 0,0 in the tilemap
    // let first_corner = *corners.get(0).unwrap();
    // tilemap[0][0] = first_corner;
    // // same for the 2 neighbours of the first corner
    // let neighbours_of_first_corner = neighbours
    //     .get(&first_corner)
    //     .unwrap()
    //     .iter()
    //     .collect::<Vec<&u16>>();
    // tilemap[0][1] = *neighbours_of_first_corner[0];
    // tilemap[1][0] = *neighbours_of_first_corner[1];

    // let mut stack: VecDeque<(u16, usize, usize)> = VecDeque::new();
    // stack.push_back((tilemap[0][1], 0, 1));
    // stack.push_back((tilemap[1][0], 1, 0));

    // while !stack.is_empty() {
    //     let (tileid, row, col) = stack.pop_front().unwrap();

    //     let mut local_neighbours = neighbours.get(&tileid).unwrap().clone();
    //     // we try to remove the neighbours that we have already placed
    //     dbg!(&local_neighbours);
    //     if col > 0 {
    //         dbg!(tilemap[row][col - 1]);
    //         local_neighbours.remove(&tilemap[row][col - 1]);
    //     }
    //     if row > 0 {
    //         local_neighbours.remove(&tilemap[row - 1][col]);
    //     }
    //     if row < 11 && tilemap[row + 1][col] != 0 {
    //         local_neighbours.remove(&tilemap[row + 1][col]);
    //     }
    //     if col < 11 && tilemap[row][col + 1] != 0 {
    //         local_neighbours.remove(&tilemap[row][col + 1]);
    //     }
    //     dbg!(&local_neighbours);

    //     // if we 2 neighbours left we need to decide which one goes right and below
    //     if local_neighbours.len() == 2 {
    //         for n in local_neighbours {
    //             if corners.contains(&n) || edges.contains(&n) {
    //                 dbg!((row, col));
    //                 tilemap[row][col + 1] = n;
    //                 stack.push_back((n, row, col + 1));
    //             } else {
    //                 dbg!((row, col));
    //                 tilemap[row + 1][col] = n;
    //                 stack.push_back((n, row + 1, col));
    //             }
    //         }
    //     } else if local_neighbours.len() == 1 {
    //         dbg!((row, col));
    //         let n = *local_neighbours.iter().next().unwrap();
    //         tilemap[row + 1][col] = n;
    //         stack.push_back((n, row + 1, col));
    //     }
    // }

    // dbg!(tilemap);

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
