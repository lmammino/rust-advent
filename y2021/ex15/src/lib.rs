use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fmt::Display,
    str::FromStr,
};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Edge {
    position: (usize, usize),
    cost: usize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct CaveMap<const N: usize> {
    cave: [[u8; N]; N],
}

impl<const N: usize> FromStr for CaveMap<N> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cave: [[u8; N]; N] = [[0; N]; N];

        for (row_id, row) in s.lines().enumerate() {
            for (col_id, cell) in row.chars().enumerate() {
                let cell_value: u8 = cell.to_digit(10).unwrap() as u8;
                cave[row_id][col_id] = cell_value;
            }
        }

        Ok(CaveMap { cave })
    }
}

impl<const N: usize> Display for CaveMap<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.cave {
            let line: String = row.iter().map(|v| v.to_string()).collect();
            writeln!(f, "{}", line)?;
        }

        Ok(())
    }
}

impl<const N: usize> CaveMap<N> {
    fn expand_map(&mut self, tile_size: usize) {
        for tile_r in 0_usize..5 {
            for tile_c in 0_usize..5 {
                // skips the referece (first) tile
                if tile_r == 0 && tile_c == 0 {
                    continue;
                }

                for rel_r in 0..tile_size {
                    for rel_c in 0..tile_size {
                        let r = rel_r + tile_size * tile_r;
                        let c = rel_c + tile_size * tile_c;

                        // for the first tile row look left, otherwise look up for the
                        // reference cell
                        let r_ref = if tile_r == 0 { r } else { r - tile_size };
                        let c_ref = if tile_r == 0 { c - tile_size } else { c };

                        let mut new_value = self.cave[r_ref][c_ref];
                        if new_value == 9 {
                            new_value = 1;
                        } else {
                            new_value += 1;
                        }
                        self.cave[r][c] = new_value;
                    }
                }
            }
        }
    }

    fn shortest_path_to_bottom_right(&self) -> usize {
        // implements Dijkstra alg
        let mut distance: HashMap<(usize, usize), usize> = Default::default();
        let mut active_nodes: BinaryHeap<Edge> = Default::default();
        distance.insert((0, 0), 0);
        let initial_path = Edge {
            cost: 0,
            position: (0, 0),
        };
        active_nodes.push(initial_path);

        while let Some(Edge { position, cost }) = active_nodes.pop() {
            // found the bottom right corner
            if position == (N - 1, N - 1) {
                return cost;
            }

            // check if we already found a better path
            let curr_dist = distance.entry(position).or_insert(usize::MAX);
            if cost > *curr_dist {
                continue;
            }

            // check the neighbours
            for (neighbor_pos, neighbor_cost) in self.neighbours_of(&position) {
                let next = Edge {
                    cost: cost + neighbor_cost as usize,
                    position: neighbor_pos,
                };

                let neighbor_dist = distance.entry(neighbor_pos).or_insert(usize::MAX);
                if next.cost < *neighbor_dist {
                    active_nodes.push(next);
                    *neighbor_dist = next.cost;
                }
            }
        }

        unreachable!()
    }

    fn neighbours_of(&self, cell: &(usize, usize)) -> Vec<((usize, usize), u8)> {
        let mut neighbours: Vec<((usize, usize), u8)> = vec![];
        let (row_id, col_id) = *cell;

        if row_id > 0 {
            let top = (row_id - 1, col_id);
            let top_value = self.cave[row_id - 1][col_id];
            neighbours.push((top, top_value));
        }
        if col_id < N - 1 {
            let right = (row_id, col_id + 1);
            let right_value = self.cave[row_id][col_id + 1];
            neighbours.push((right, right_value));
        }
        if row_id < N - 1 {
            let bottom = (row_id + 1, col_id);
            let bottom_value = self.cave[row_id + 1][col_id];
            neighbours.push((bottom, bottom_value));
        }
        if col_id > 0 {
            let left = (row_id, col_id - 1);
            let left_value = self.cave[row_id][col_id - 1];
            neighbours.push((left, left_value));
        }

        neighbours
    }
}

pub fn part1(input: &str) -> usize {
    let cave: CaveMap<100> = input.parse().unwrap();
    cave.shortest_path_to_bottom_right()
}

pub fn part2(input: &str) -> usize {
    let mut cave: CaveMap<500> = input.parse().unwrap();
    cave.expand_map(100);
    cave.shortest_path_to_bottom_right()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        let cave: CaveMap<10> = input.parse().unwrap();
        assert_eq!(cave.shortest_path_to_bottom_right(), 40);
    }

    #[test]
    fn test_part2_example() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        let mut cave: CaveMap<50> = input.parse().unwrap();
        cave.expand_map(10);
        assert_eq!(cave.shortest_path_to_bottom_right(), 315);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 687);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 2957);
    }
}
