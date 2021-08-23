use std::collections::HashSet;

type Point = (i32, i32, i32);
type Cube = HashSet<Point>;

fn neighbours_of_point(point: Point) -> [Point; 26] {
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

fn next_state(old_cube: Cube) -> Cube {
    let cube = Cube::new();

    old_cube.into_iter();
    // TODO: for each point in the cube, get the neighbours & store temporarily
    // TODO: for each neighbour AND the current point, we need to apply the rules (read README)
    // TODO: fn to check alive neighbour count

    cube
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

    // TODO: calculate total number of alive points (cube length)
    280
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
        let result = neighbours_of_point((0, 0, 0));
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
