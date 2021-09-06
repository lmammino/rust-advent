use std::collections::HashSet;

type Point3D = (i32, i32, i32);
type Universe3D = HashSet<Point3D>;

type Point4D = (i32, i32, i32, i32);
type Universe4D = HashSet<Point4D>;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Point<const D: usize>([i32; D]);

#[derive(Debug)]
struct Universe<const D: usize>(HashSet<Point<D>>);

fn neighbours_of_point<const D: usize>(point: &Point<D>) -> Vec<Point<D>> {
    let mut points = vec![];

    for i in 0..3_i32.pow(D as u32) {
        let mut relative_coords = [0; D];
        let mut n = i;
        for j in 0..D {
            relative_coords[j] = (n % 3) - 1;
            n /= 3;
        }

        if relative_coords != [0; D] {
            let mut coords = [0; D];
            for j in 0..D {
                coords[j] = relative_coords[j] + point.0[j];
            }
            points.push(Point(coords));
        }
    }

    points
}

fn neighbours_of_point_3d(point: &Point3D) -> [Point3D; 26] {
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

fn neighbours_of_point_4d(point: &Point4D) -> [Point4D; 80] {
    let mut points = [(0, 0, 0, 0); 80];
    let mut index = 0;
    for z in -1..=1 {
        for y in -1..=1 {
            for x in -1..=1 {
                for w in -1..=1 {
                    if (z, y, x, w) != (0, 0, 0, 0) {
                        points[index] = (x + point.0, y + point.1, z + point.2, w + point.3);
                        index += 1;
                    }
                }
            }
        }
    }

    points
}

fn count_active_neighbours_3d(point: &Point3D, cube: &Universe3D) -> usize {
    neighbours_of_point_3d(point)
        .iter()
        .filter(|p| cube.contains(p))
        .count()
}

fn count_active_neighbours_4d(point: &Point4D, cube: &Universe4D) -> usize {
    neighbours_of_point_4d(point)
        .iter()
        .filter(|p| cube.contains(p))
        .count()
}

fn count_active_neighbours<const D: usize>(point: &Point<D>, universe: &Universe<D>) -> usize {
    neighbours_of_point(point)
        .iter()
        .filter(|p| universe.0.contains(p))
        .count()
}

fn next_state_3d(old_universe: Universe3D) -> Universe3D {
    let mut points_to_check: HashSet<Point3D> = HashSet::new();
    for point in old_universe.iter() {
        points_to_check.insert(*point);
        for p in neighbours_of_point_3d(point).iter() {
            points_to_check.insert(*p);
        }
    }

    points_to_check
        .into_iter()
        .filter(|point| {
            let active_neighbours = count_active_neighbours_3d(point, &old_universe);
            let point_is_alive = old_universe.contains(point);
            matches!((point_is_alive, active_neighbours), (_, 3) | (true, 2))
        })
        .collect()
}

fn next_state_4d(old_universe: Universe4D) -> Universe4D {
    let mut points_to_check: HashSet<Point4D> = HashSet::new();
    for point in old_universe.iter() {
        points_to_check.insert(*point);
        for p in neighbours_of_point_4d(point).iter() {
            points_to_check.insert(*p);
        }
    }

    points_to_check
        .into_iter()
        .filter(|point| {
            let active_neighbours = count_active_neighbours_4d(point, &old_universe);
            let point_is_alive = old_universe.contains(point);
            matches!((point_is_alive, active_neighbours), (_, 3) | (true, 2))
        })
        .collect()
}

fn next_state<const D: usize>(old_universe: Universe<D>) -> Universe<D> {
    let mut points_to_check: HashSet<Point<D>> = HashSet::new();
    for point in old_universe.0.iter() {
        points_to_check.insert(point.clone());
        for p in neighbours_of_point(point).iter() {
            points_to_check.insert(p.clone());
        }
    }

    Universe(
        points_to_check
            .into_iter()
            .filter(|point| {
                let active_neighbours = count_active_neighbours(point, &old_universe);
                let point_is_alive = old_universe.0.contains(point);
                matches!((point_is_alive, active_neighbours), (_, 3) | (true, 2))
            })
            .collect(),
    )
}

pub fn part1(input: &str) -> u32 {
    let mut cube = Universe3D::new();

    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            let new_point = (x as i32, y as i32, 0);
            if cell == '#' {
                cube.insert(new_point);
            }
        }
    }

    for _ in 0..6 {
        cube = next_state_3d(cube);
    }

    cube.len() as u32
}

pub fn part1_gen(input: &str) -> u32 {
    let mut data: HashSet<Point<3>> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            let new_point = Point([x as i32, y as i32, 0]);
            if cell == '#' {
                data.insert(new_point);
            }
        }
    }

    let mut universe = Universe(data);
    for _ in 0..6 {
        universe = next_state(universe);
    }

    universe.0.len() as u32
}

pub fn part2(input: &str) -> u32 {
    let mut universe = Universe4D::new();

    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            let new_point = (x as i32, y as i32, 0, 0);
            if cell == '#' {
                universe.insert(new_point);
            }
        }
    }

    for _ in 0..6 {
        universe = next_state_4d(universe);
    }

    universe.len() as u32
}

pub fn part2_gen(input: &str) -> u32 {
    let mut data: HashSet<Point<4>> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            let new_point = Point([x as i32, y as i32, 0, 0]);
            if cell == '#' {
                data.insert(new_point);
            }
        }
    }

    let mut universe = Universe(data);
    for _ in 0..6 {
        universe = next_state(universe);
    }

    universe.0.len() as u32
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
    fn part_1_gen() {
        let input = include_str!("../input.txt");
        assert_eq!(part1_gen(input), 280);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1696);
    }

    #[test]
    fn part_2_gen() {
        let input = include_str!("../input.txt");
        assert_eq!(part2_gen(input), 1696);
    }

    #[test]
    fn test_neighbours() {
        let result = neighbours_of_point_3d(&(0, 0, 0));
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
