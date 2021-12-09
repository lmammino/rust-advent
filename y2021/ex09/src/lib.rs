use std::{
    collections::{BinaryHeap, HashSet, VecDeque},
    ops::Deref,
    str::FromStr,
};

#[derive(Debug)]
struct Space<const W: usize, const H: usize>([[u8; W]; H]);

impl<const W: usize, const H: usize> Deref for Space<W, H> {
    type Target = [[u8; W]; H];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const W: usize, const H: usize> FromStr for Space<W, H> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data: [[u8; W]; H] = [[0; W]; H];
        for (row_id, line) in s.lines().enumerate() {
            for (col_id, value) in line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .enumerate()
            {
                data[row_id][col_id] = value;
            }
        }

        Ok(Space(data))
    }
}

impl<const W: usize, const H: usize> Space<W, H> {
    fn find_low_points(&self) -> Vec<(usize, usize)> {
        let mut low_points: Vec<(usize, usize)> = vec![];

        for (row_id, row) in self.0.iter().enumerate() {
            for (col_id, cell) in row.iter().enumerate() {
                let mut neighbours: Vec<u8> = Vec::with_capacity(4);
                // top neighbour
                if row_id > 0 {
                    neighbours.push(self.0[row_id - 1][col_id]);
                }
                // right neighbour
                if col_id < W - 1 {
                    neighbours.push(self.0[row_id][col_id + 1]);
                }
                // bottom neighbour
                if row_id < H - 1 {
                    neighbours.push(self.0[row_id + 1][col_id]);
                }
                // left neighbour
                if col_id > 0 {
                    neighbours.push(self.0[row_id][col_id - 1]);
                }

                if neighbours.iter().all(|n| cell < n) {
                    low_points.push((row_id, col_id));
                }
            }
        }

        low_points
    }

    fn basin_size(&self, pos: (usize, usize)) -> usize {
        // row_id, col_id
        let mut visited: HashSet<(usize, usize)> = Default::default();
        // row_id, col_id
        let mut to_visit: VecDeque<(usize, usize)> = Default::default();
        to_visit.push_front((pos.0, pos.1));

        while !to_visit.is_empty() {
            let (row_id, col_id) = to_visit.pop_front().unwrap();
            // mark this cell as visited
            visited.insert((row_id, col_id));

            let cell_value = self.0[row_id][col_id];
            // checks which adjacent values (not visited yet) should be visited

            // top neighbour
            if row_id > 0
                && cell_value + 1 < 9
                && self.0[row_id - 1][col_id] < 9
                && !visited.contains(&(row_id - 1, col_id))
            {
                to_visit.push_front((row_id - 1, col_id));
            }

            // right neighbour
            if col_id < W - 1
                && cell_value + 1 < 9
                && self.0[row_id][col_id + 1] < 9
                && !visited.contains(&(row_id, col_id + 1))
            {
                to_visit.push_front((row_id, col_id + 1));
            }

            // bottom neighbour
            if row_id < H - 1
                && cell_value + 1 < 9
                && self.0[row_id + 1][col_id] < 9
                && !visited.contains(&(row_id + 1, col_id))
            {
                to_visit.push_front((row_id + 1, col_id));
            }

            // left neighbour
            if col_id > 0
                && cell_value + 1 < 9
                && self.0[row_id][col_id - 1] < 9
                && !visited.contains(&(row_id, col_id - 1))
            {
                to_visit.push_front((row_id, col_id - 1));
            }
        }

        visited.len()
    }
}

pub fn part1(input: &str) -> usize {
    let space: Space<100, 100> = input.parse().unwrap();

    let low_points = space.find_low_points();
    low_points
        .iter()
        .map(|(row_id, col_id)| {
            let risk_level = space[*row_id][*col_id] + 1;
            risk_level as usize
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let space: Space<100, 100> = input.parse().unwrap();

    let mut basins: BinaryHeap<usize> = space
        .find_low_points()
        .into_iter()
        .map(|p| space.basin_size(p))
        .collect();

    let top1 = basins.pop().unwrap();
    let top2 = basins.pop().unwrap();
    let top3 = basins.pop().unwrap();

    top1 * top2 * top3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 448);
    }

    #[test]
    fn test_low_points_and_basin_size() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";

        let space: Space<10, 5> = input.parse().unwrap();
        let low_points = space.find_low_points();

        assert_eq!(low_points, vec![(0, 1), (0, 9), (2, 2), (4, 6)]);

        let basins: Vec<usize> = low_points
            .iter()
            .map(|pos| space.basin_size(*pos))
            .collect();

        assert_eq!(basins, vec![3, 9, 14, 9]);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1417248);
    }
}
