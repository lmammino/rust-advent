mod neighbours;

use neighbours::*;
use std::collections::HashSet;
use std::ops;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Point<const D: usize>([i32; D]);

impl<const D: usize> ops::Add<[i32; D]> for &Point<D> {
    type Output = Point<D>;

    fn add(self, rhs: [i32; D]) -> Point<D> {
        let mut res = [0; D];
        for (i, n) in self.0.iter().enumerate() {
            res[i] = n + rhs[i];
        }

        Point::<D>(res)
    }
}

#[derive(Debug)]
struct Universe<const D: usize> {
    points: HashSet<Point<D>>,
    neighbour_maker: NeighboursMaker<D>,
}

impl<const D: usize> Universe<D> {
    fn from_input(input: &str) -> Self {
        let mut points = HashSet::new();

        for (y, line) in input.lines().enumerate() {
            for (x, cell) in line.chars().enumerate() {
                let mut point_data = [0; D];
                point_data[0] = x as i32;
                point_data[1] = y as i32;
                let new_point = Point(point_data);
                if cell == '#' {
                    points.insert(new_point);
                }
            }
        }

        Universe {
            points,
            neighbour_maker: NeighboursMaker::<D>::new(),
        }
    }

    fn count_active_neighbours(&self, point: &Point<D>) -> usize {
        self.neighbour_maker
            .for_point(point)
            .filter(|p| self.points.contains(p))
            .count()
    }

    fn active_cells(&self) -> usize {
        self.points.len()
    }

    fn transition_next_state(&mut self) {
        let new_points: HashSet<Point<D>> = self
            .points
            .iter() // TODO: check if this could be made multi-thread
            .flat_map(|p| self.neighbour_maker.for_point_with_self(p))
            // TODO: check if we can make elements unique here
            .filter(|point| {
                let active_neighbours = self.count_active_neighbours(point);
                let point_is_alive = self.points.contains(point);
                matches!((point_is_alive, active_neighbours), (_, 3) | (true, 2))
            })
            .collect();

        self.points = new_points;
    }
}

struct Game<const D: usize> {
    universe: Universe<D>,
}

impl<const D: usize> Game<D> {
    fn new(input: &str) -> Self {
        let universe = Universe::<D>::from_input(input);
        Game { universe }
    }
}

impl<const D: usize> Iterator for Game<D> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.universe.transition_next_state();
        Some(self.universe.active_cells())
    }
}

pub fn part1(input: &str) -> usize {
    let mut game = Game::<3>::new(input);
    game.nth(5).unwrap()
}

pub fn part2(input: &str) -> usize {
    let mut game = Game::<4>::new(input);
    game.nth(5).unwrap()
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
}
