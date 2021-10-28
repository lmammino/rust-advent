mod tile;
mod tiles_index;

use std::collections::{HashMap, HashSet, VecDeque};
use tile::*;
use tiles_index::*;

pub fn part1(input: &str) -> u64 {
    let tiles: TilesIndex<10> = input.parse().unwrap();

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
    let tiles: TilesIndex<10> = input.parse().unwrap();

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

    let zero_zero = tiles.0.get(&tilemap[0][0]).unwrap(); // top left corner
    let zero_one = tiles.0.get(&tilemap[0][1]).unwrap(); // right
    let one_zero = tiles.0.get(&tilemap[1][0]).unwrap(); // bottom
    let possible_zero_zero: Vec<Tile<10>> = vec![
        *zero_zero,
        zero_zero.rotate(),
        zero_zero.rotate().rotate(),
        zero_zero.rotate().rotate().rotate(),
        zero_zero.flip_horiz(),
        zero_zero.flip_horiz().rotate(),
        zero_zero.flip_horiz().rotate().rotate(),
        zero_zero.flip_horiz().rotate().rotate().rotate(),
    ];

    let zero_zero_oriented_right: Vec<Tile<10>> = possible_zero_zero
        .into_iter()
        .filter(|x| fit_tile_right(x, zero_one).is_some() && fit_tile_bottom(x, one_zero).is_some())
        .collect();

    // We expect 1 possible tiles
    dbg!(zero_zero_oriented_right.len());
    // dbg!(&zero_zero_oriented_right[0]);

    let mut img: [[Tile<10>; 12]; 12] = [[zero_zero_oriented_right[0]; 12]; 12];
    // img[0][0] = zero_zero_oriented_right[0];
    for y in 0..12 {
        if y > 0 {
            let top = &img[y - 1][0];
            let bottom = tiles.0.get(&tilemap[y][0]).unwrap();
            img[y][0] = fit_tile_bottom(top, bottom).unwrap();
        }
        for x in 1..12 {
            let left = &img[y][x - 1];
            let right = tiles.0.get(&tilemap[y][x]).unwrap();
            img[y][x] = fit_tile_right(left, right).unwrap();
        }
    }

    // change 1..9 into 0..10 to see the image with border

    for row in img.iter() {
        for inner_row in 1..9 {
            for tile in row {
                print!(
                    "{}",
                    tile.cells[inner_row][1..9]
                        .iter()
                        .cloned()
                        .collect::<String>()
                );
            }
            println!();
        }
    }

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
