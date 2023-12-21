use std::{collections::HashSet, vec};

use models::{Map, Position, Tile};

mod models;

fn solve_part1<const W: usize, const H: usize>(input: &str, cycles: usize) -> usize {
    let map: Map<W, H> = Map::new(input);

    let mut queue: Vec<(usize, Position)> = vec![(0, map.start)];
    let mut seen_positions: HashSet<(usize, Position)> = HashSet::new();
    let mut final_positions: HashSet<Position> = HashSet::new();
    loop {
        let next_item = queue.pop();
        if next_item.is_none() {
            break;
        }
        let (num_cycle, position) = next_item.unwrap();
        if num_cycle == cycles {
            final_positions.insert(position);
            continue;
        }
        if num_cycle >= cycles {
            // we can stop simulating because we have covered all the necessary steps
            break;
        }
        if seen_positions.contains(&(num_cycle, position)) {
            continue;
        }
        seen_positions.insert((num_cycle, position));

        for (dx, dy) in [(0_isize, -1_isize), (1, 0), (0, 1), (-1, 0)] {
            let (new_pos_x, new_pos_y) = (position.0 as isize + dx, position.1 as isize + dy);
            if new_pos_x < 0 || new_pos_x >= W as isize || new_pos_y < 0 || new_pos_y >= H as isize
            {
                continue;
            }
            let (new_pos_x, new_pos_y) = (new_pos_x as usize, new_pos_y as usize);
            if seen_positions.contains(&(num_cycle + 1, (new_pos_x, new_pos_y))) {
                continue;
            }
            match map.map[new_pos_y][new_pos_x] {
                Tile::Plot => {
                    queue.push((num_cycle + 1, (new_pos_x, new_pos_y)));
                }
                Tile::Rock => {}
            }
        }
    }

    final_positions.len()
}

pub fn part1(input: &str) -> usize {
    solve_part1::<131, 131>(input, 64)
}

pub fn part2(_input: &str) -> usize {
    // TODO:
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");
    const EXAMPLE_INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1::<11, 11>(EXAMPLE_INPUT, 1), 2);
        assert_eq!(solve_part1::<11, 11>(EXAMPLE_INPUT, 2), 4);
        assert_eq!(solve_part1::<11, 11>(EXAMPLE_INPUT, 3), 6);
        assert_eq!(solve_part1::<11, 11>(EXAMPLE_INPUT, 6), 16);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 3615);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
