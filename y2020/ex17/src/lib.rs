use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Point<const D: usize>([i32; D]);

#[derive(Debug)]
struct Universe<const D: usize>(HashSet<Point<D>>);

#[derive(Debug)]
struct Game<const D: usize> {
    universe: Universe<D>,
    relative_neighbours: Vec<[i32; D]>,
}

impl<const D: usize> Game<D> {
    fn from_input(input: &str) -> Self {
        let mut data: HashSet<Point<D>> = HashSet::new();

        for (y, line) in input.lines().enumerate() {
            for (x, cell) in line.chars().enumerate() {
                let mut point_data = [0; D];
                point_data[0] = x as i32;
                point_data[1] = y as i32;
                let new_point = Point(point_data);
                if cell == '#' {
                    data.insert(new_point);
                }
            }
        }
        let universe = Universe(data);

        let mut relative_neighbours = vec![];

        for i in 0..3_i32.pow(D as u32) {
            let mut relative_coords = [0; D];
            let mut n = i;
            for coord in relative_coords.iter_mut() {
                *coord = (n % 3) - 1;
                n /= 3;
            }
            if relative_coords != [0; D] {
                relative_neighbours.push(relative_coords);
            }
        }

        Game {
            universe,
            relative_neighbours,
        }
    }
}

impl<const D: usize> Iterator for Game<D> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut points_to_check: HashSet<Point<D>> = HashSet::new();
        for point in self.universe.0.iter() {
            points_to_check.insert(point.clone());
            for p in neighbours_of_point(point, &self.relative_neighbours) {
                points_to_check.insert(p.clone());
            }
        }

        let new_universe = Universe(
            points_to_check
                .into_iter()
                .filter(|point| {
                    let active_neighbours =
                        count_active_neighbours(point, &self.relative_neighbours, &self.universe);
                    let point_is_alive = &self.universe.0.contains(point);
                    matches!((point_is_alive, active_neighbours), (_, 3) | (true, 2))
                })
                .collect(),
        );

        self.universe = new_universe;
        Some(self.universe.0.len())
    }
}

fn neighbours_of_point<'a, const D: usize>(
    point: &'a Point<D>,
    relative_neighbours: &'a [[i32; D]],
) -> impl Iterator<Item = Point<D>> + 'a {
    relative_neighbours.iter().map(move |neighbour| {
        let mut data = [0; D];
        for (i, c) in data.iter_mut().enumerate() {
            *c = point.0[i] + neighbour[i];
        }
        Point(data)
    })
}

fn count_active_neighbours<const D: usize>(
    point: &Point<D>,
    relative_neighbours: &[[i32; D]],
    universe: &Universe<D>,
) -> usize {
    neighbours_of_point(point, relative_neighbours)
        .filter(|p| universe.0.contains(p))
        .count()
}

pub fn part1(input: &str) -> u32 {
    let mut game = Game::<3>::from_input(input);
    game.nth(5).unwrap() as u32
}

pub fn part2(input: &str) -> u32 {
    let mut game = Game::<4>::from_input(input);
    game.nth(5).unwrap() as u32
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
        let game = Game::<3>::from_input(".");
        assert_eq!(
            game.relative_neighbours,
            [
                [-1, -1, -1],
                [0, -1, -1],
                [1, -1, -1],
                [-1, 0, -1],
                [0, 0, -1],
                [1, 0, -1],
                [-1, 1, -1],
                [0, 1, -1],
                [1, 1, -1],
                [-1, -1, 0],
                [0, -1, 0],
                [1, -1, 0],
                [-1, 0, 0],
                [1, 0, 0],
                [-1, 1, 0],
                [0, 1, 0],
                [1, 1, 0],
                [-1, -1, 1],
                [0, -1, 1],
                [1, -1, 1],
                [-1, 0, 1],
                [0, 0, 1],
                [1, 0, 1],
                [-1, 1, 1],
                [0, 1, 1],
                [1, 1, 1],
            ]
        );
    }
}
