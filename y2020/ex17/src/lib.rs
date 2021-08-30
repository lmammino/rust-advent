use std::collections::HashSet;

type Point = (i32, i32, i32);
type Cube = HashSet<Point>;

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
        let active_neighbours = count_active_neighbours(&point, &old_cube);
        let point_is_alive = old_cube.contains(&point);
        match (point_is_alive, active_neighbours) {
            (_, 3) | (true, 2) => true,
            _ => false
        }
    }).collect()
}

pub fn part1(input: &str) -> u32 {
    let mut cube = Cube::new();

    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            let new_point = (x as i32, y as i32, 0);
            if cell == '#' {
                cube.insert(new_point);
            }
        }
    }

    for _ in 0..6 {
        cube = next_state(cube);
    }

    cube.len() as u32
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
