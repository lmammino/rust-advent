use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::TryInto,
    str::FromStr,
};

#[derive(Debug, Clone)]
struct Tile<const N: usize> {
    id: u16,
    cells: [[char; N]; N],
    borders: [[char; N]; 8],
}

struct Tiles<const N: usize>(HashMap<u16, Tile<N>>);

const TOP: usize = 0;
const RTOP: usize = 1;
const BOTTOM: usize = 2;
const RBOTTOM: usize = 3;
const LEFT: usize = 4;
const RLEFT: usize = 5;
const RIGHT: usize = 6;
const RRIGHT: usize = 7;

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

    // rotating 90 deg clockwise
    fn rotate(&self) -> Self {
        let rotated_cells: [[char; N]; N] = self
            .cells
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let mut col: [char; N] = [' '; N];
                for j in 0..N {
                    col[N - 1 - j] = self.cells[j][i];
                }
                col
            })
            .collect::<Vec<[char; N]>>()
            .try_into()
            .unwrap();

        Tile::new(self.id, rotated_cells)
    }

    fn flip_horiz(&self) -> Self {
        let flipped_cells: [[char; N]; N] = self
            .cells
            .iter()
            .map(|row| {
                let mut rev_row = *row;
                rev_row.reverse();
                rev_row
            })
            .collect::<Vec<[char; N]>>()
            .try_into()
            .unwrap();

        Tile::new(self.id, flipped_cells)
    }

    fn flip_vert(&self) -> Self {
        let mut flipped_cells = self.cells;
        flipped_cells.reverse();

        Tile::new(self.id, flipped_cells)
    }
}

fn fit_tile_right<const N: usize>(left: &Tile<N>, right: &Tile<N>) -> Option<Tile<N>> {
    let right_tile_overlapping_border = right
        .borders
        .iter()
        .position(|x| *x == left.borders[RIGHT])?;

    Some(match right_tile_overlapping_border {
        LEFT => right.clone(),
        RLEFT => right.flip_vert(),
        RIGHT => right.flip_horiz(),
        RRIGHT => right.flip_horiz().flip_vert(),
        TOP => right.rotate().flip_horiz(),
        RTOP => right.rotate().rotate().rotate(),
        BOTTOM => right.rotate(),
        RBOTTOM => right.rotate().flip_vert(),
        _ => unreachable!(),
    })
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
                return Some(*key);
            }

            None
        })
        .collect::<Vec<u16>>();

    let edges = neighbours
        .iter()
        .filter_map(|(key, val)| {
            if val.len() == 3 {
                return Some(*key);
            }

            None
        })
        .collect::<HashSet<u16>>();

    let mut tilemap: [[u16; 12]; 12] = [[0; 12]; 12];
    // we take an arbitrary corner (the first one) and we place it at 0,0 in the tilemap
    let first_corner = *corners.get(0).unwrap();
    tilemap[0][0] = first_corner;
    // same for the 2 neighbours of the first corner
    let neighbours_of_first_corner = neighbours
        .get(&first_corner)
        .unwrap()
        .iter()
        .collect::<Vec<&u16>>();
    tilemap[0][1] = *neighbours_of_first_corner[0];
    tilemap[1][0] = *neighbours_of_first_corner[1];

    let mut stack: VecDeque<(u16, usize, usize)> = VecDeque::new();
    stack.push_back((tilemap[0][1], 0, 1));
    stack.push_back((tilemap[1][0], 1, 0));

    while !stack.is_empty() {
        let (tileid, row, col) = stack.pop_front().unwrap();

        let mut local_neighbours = neighbours.get(&tileid).unwrap().clone();
        // we try to remove the neighbours that we have already placed
        if col > 0 {
            local_neighbours.remove(&tilemap[row][col - 1]);
        }
        if row > 0 {
            local_neighbours.remove(&tilemap[row - 1][col]);
        }
        if row < 11 && tilemap[row + 1][col] != 0 {
            local_neighbours.remove(&tilemap[row + 1][col]);
        }
        if col < 11 && tilemap[row][col + 1] != 0 {
            local_neighbours.remove(&tilemap[row][col + 1]);
        }

        // if we 2 neighbours left we need to decide which one goes right and below
        if local_neighbours.len() == 2 {
            for n in local_neighbours {
                if corners.contains(&n) || edges.contains(&n) {
                    tilemap[row][col + 1] = n;
                } else {
                    tilemap[row + 1][col] = n;
                }
            }
            stack.push_back((tilemap[row][col + 1], row, col + 1));
            stack.push_back((tilemap[row + 1][col], row + 1, col));
        } else if local_neighbours.len() == 1 {
            let n = *local_neighbours.iter().next().unwrap();
            tilemap[row + 1][col] = n;
            stack.push_back((n, row + 1, col));
        }
    }

    dbg!(tilemap);

    // now that we have a tilemap of relative tiles
    // we need to make sure that all the tiles are rotated/flipped correctly
    // to do that we start with the first tile (top/left corner).
    // For this one we need to find a possible rotation that fits its bottom and right tile.

    let zero_zero = tiles.0.get(&tilemap[0][0]).unwrap();
    let zero_one = tiles.0.get(&tilemap[0][1]).unwrap();

    let possible_zero_zero: Vec<Tile<10>> = vec![
        fit_tile_right(zero_zero, zero_one),
        fit_tile_right(&zero_zero.rotate(), zero_one),
        fit_tile_right(&zero_zero.rotate().rotate(), zero_one),
        fit_tile_right(&zero_zero.rotate().rotate().rotate(), zero_one),
        fit_tile_right(&zero_zero.flip_horiz(), zero_one),
        fit_tile_right(&zero_zero.flip_horiz().rotate(), zero_one),
        fit_tile_right(&zero_zero.flip_horiz().rotate().rotate(), zero_one),
        fit_tile_right(&zero_zero.flip_horiz().rotate().rotate().rotate(), zero_one),
    ]
    .into_iter()
    .flatten()
    .collect();

    // We expect 2 possible tiles
    dbg!(possible_zero_zero.len());

    // TODO: do the same with the tile below
    // we should find other 2 possible combinations and there will be only one in common

    2006
}

#[cfg(test)]
mod ex20_tests {
    use super::*;

    #[test]
    fn rotate_tile() {
        let cells = [
            ['a', 'b', 'c', 'd'],
            ['e', 'f', 'g', 'h'],
            ['i', 'j', 'k', 'l'],
            ['m', 'n', 'o', 'p'],
        ];

        let tile = Tile::new(0, cells);

        let rotated_tile = tile.rotate();

        let expected = [
            ['m', 'i', 'e', 'a'],
            ['n', 'j', 'f', 'b'],
            ['o', 'k', 'g', 'c'],
            ['p', 'l', 'h', 'd'],
        ];

        assert_eq!(rotated_tile.cells, expected)
    }

    #[test]
    fn flip_horiz_tile() {
        let cells = [
            ['a', 'b', 'c', 'd'],
            ['e', 'f', 'g', 'h'],
            ['i', 'j', 'k', 'l'],
            ['m', 'n', 'o', 'p'],
        ];

        let tile = Tile::new(0, cells);

        let flipped_tile = tile.flip_horiz();

        let expected = [
            ['d', 'c', 'b', 'a'],
            ['h', 'g', 'f', 'e'],
            ['l', 'k', 'j', 'i'],
            ['p', 'o', 'n', 'm'],
        ];

        assert_eq!(flipped_tile.cells, expected)
    }

    #[test]
    fn flip_vert_tile() {
        let cells = [
            ['a', 'b', 'c', 'd'],
            ['e', 'f', 'g', 'h'],
            ['i', 'j', 'k', 'l'],
            ['m', 'n', 'o', 'p'],
        ];

        let tile = Tile::new(0, cells);

        let flipped_tile = tile.flip_vert();

        let expected = [
            ['m', 'n', 'o', 'p'],
            ['i', 'j', 'k', 'l'],
            ['e', 'f', 'g', 'h'],
            ['a', 'b', 'c', 'd'],
        ];

        assert_eq!(flipped_tile.cells, expected)
    }

    #[test]
    fn test_fit_tile_right() {
        let left = [
            ['l', 'l', 'l', '1'],
            ['l', 'l', 'l', '2'],
            ['l', 'l', 'l', '3'],
            ['l', 'l', 'l', '4'],
        ];
        let left_tile = Tile::new(0, left);

        let test_data = [
            // LEFT
            [
                ['1', 'x', 'x', 'x'],
                ['2', 'x', 'x', 'x'],
                ['3', 'x', 'x', 'x'],
                ['4', 'x', 'x', 'x'],
            ],
            // RLEFT
            [
                ['4', 'x', 'x', 'x'],
                ['3', 'x', 'x', 'x'],
                ['2', 'x', 'x', 'x'],
                ['1', 'x', 'x', 'x'],
            ],
            // RIGHT
            [
                ['x', 'x', 'x', '1'],
                ['x', 'x', 'x', '2'],
                ['x', 'x', 'x', '3'],
                ['x', 'x', 'x', '4'],
            ],
            // RRIGHT
            [
                ['x', 'x', 'x', '4'],
                ['x', 'x', 'x', '3'],
                ['x', 'x', 'x', '2'],
                ['x', 'x', 'x', '1'],
            ],
            // TOP
            [
                ['1', '2', '3', '4'],
                ['x', 'x', 'x', 'x'],
                ['x', 'x', 'x', 'x'],
                ['x', 'x', 'x', 'x'],
            ],
            // RTOP
            [
                ['4', '3', '2', '1'],
                ['x', 'x', 'x', 'x'],
                ['x', 'x', 'x', 'x'],
                ['x', 'x', 'x', 'x'],
            ],
            // BOTTOM
            [
                ['x', 'x', 'x', 'x'],
                ['x', 'x', 'x', 'x'],
                ['x', 'x', 'x', 'x'],
                ['1', '2', '3', '4'],
            ],
            // RBOTTOM
            [
                ['x', 'x', 'x', 'x'],
                ['x', 'x', 'x', 'x'],
                ['x', 'x', 'x', 'x'],
                ['4', '3', '2', '1'],
            ],
        ];

        let expected = [
            ['1', 'x', 'x', 'x'],
            ['2', 'x', 'x', 'x'],
            ['3', 'x', 'x', 'x'],
            ['4', 'x', 'x', 'x'],
        ];

        for right in test_data.iter() {
            let right_tile = Tile::new(1, *right);
            let test_tile = fit_tile_right::<4>(&left_tile, &right_tile).unwrap();
            assert_eq!(test_tile.cells, expected);
        }
    }

    #[test]
    fn test_fit_tile_right_not_matching() {
        let left = [
            ['x', 'x', 'x', 'x'],
            ['x', 'x', 'x', 'x'],
            ['x', 'x', 'x', 'x'],
            ['1', '2', '3', '4'],
        ];
        let left_tile = Tile::new(0, left);

        let right = [
            ['1', 'y', 'y', 'y'],
            ['2', 'y', 'y', 'y'],
            ['3', 'y', 'y', 'y'],
            ['4', 'y', 'y', 'y'],
        ];
        let right_tile = Tile::new(0, right);

        // these two tiles cannot fit, so we expect a None
        let result = fit_tile_right::<4>(&left_tile, &right_tile);

        assert!(result.is_none());
    }

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
