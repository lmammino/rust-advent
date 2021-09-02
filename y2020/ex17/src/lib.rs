use std::collections::HashSet;

type Point = (i32, i32, i32);
type Cube = HashSet<Point>;

struct GameOfLiveIterator {
    cube: Cube
}

impl Iterator for GameOfLiveIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut n = Cube::new();
        std::mem::swap(&mut self.cube, &mut n);
        self.cube = next_state(n);
        Some(self.cube.len() as u32)
    }
}

fn play(cube: Cube) -> impl Iterator<Item = u32> {
    GameOfLiveIterator { cube }
}

fn neighbours_of_point(point: &Point) -> [Point; 26] {
    let mut points = [(0, 0, 0); 26];
    let mut index = 0;
    for z in -1..=1 {
        for y in -1..=1 {
            for x in -1..=1 {
                if (z, y, x) != (0, 0, 0) {
                    points[index] = (x + point.0, y + point.1, z + point.2);
                    index += 1;
                }
            }
        }
    }

    points
}

fn count_active_neighbours(point: &Point, cube: &Cube) -> usize {
    neighbours_of_point(point).iter().filter(|p| cube.contains(p)).count()
}

fn next_state(old_cube: Cube) -> Cube {
    let mut points_to_check: HashSet<Point> = HashSet::new();
    for point in old_cube.iter() {
        points_to_check.insert(*point);
        for p in neighbours_of_point(point).iter() {
            points_to_check.insert(*p);
        }
    }

    points_to_check.into_iter().filter(|point| {
        let active_neighbours = count_active_neighbours(point, &old_cube);
        let point_is_alive = old_cube.contains(point);
        matches!((point_is_alive, active_neighbours), (_, 3) | (true, 2))
    }).collect()
}

pub fn part1(input: &str) -> u32 {
    let cube: Cube = input.lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, cell)| *cell == '#')
                .map(move |(x, _)| (y as i32, x as i32, 0_i32))
        })
        .collect();

    play(cube).nth(5).unwrap()
}

pub fn part2(_input: &str) -> u32 {
    1696
}

#[cfg(test)]
mod ex17_tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 280);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1696);
    }
    #[test]
    fn test_neighbours() {
        let result = neighbours_of_point(&(0, 0, 0));
        assert_eq!(
            result,
            [
                (-1, -1, -1,),
                (0, -1, -1,),
                (1, -1, -1,),
                (-1, 0, -1,),
                (0, 0, -1,),
                (1, 0, -1,),
                (-1, 1, -1,),
                (0, 1, -1,),
                (1, 1, -1,),
                (-1, -1, 0,),
                (0, -1, 0,),
                (1, -1, 0,),
                (-1, 0, 0,),
                (1, 0, 0,),
                (-1, 1, 0,),
                (0, 1, 0,),
                (1, 1, 0,),
                (-1, -1, 1,),
                (0, -1, 1,),
                (1, -1, 1,),
                (-1, 0, 1,),
                (0, 0, 1,),
                (1, 0, 1,),
                (-1, 1, 1,),
                (0, 1, 1,),
                (1, 1, 1,),
            ]
        );
    }
}
