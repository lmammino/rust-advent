use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug, Clone)]
pub struct Map {
    width: usize,
    height: usize,
    rows: HashMap<usize, HashSet<usize>>,
    cols: HashMap<usize, HashSet<usize>>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();
        let mut rows: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut cols: HashMap<usize, HashSet<usize>> = HashMap::new();

        for (row, line) in s.lines().enumerate() {
            for (col, tile) in line.chars().enumerate() {
                if tile == '#' {
                    rows.entry(row).or_default().insert(col);
                    cols.entry(col).or_default().insert(row);
                }
            }
        }

        Ok(Map {
            width,
            height,
            rows,
            cols,
        })
    }
}

impl Map {
    fn has_reflection_at(
        &self,
        idx: usize,
        len: usize,
        ref_index: &HashMap<usize, HashSet<usize>>,
    ) -> bool {
        let min_distance_to_edge = min(idx, len - idx);
        for delta in 0..=(min_distance_to_edge + 1) {
            if idx + delta >= len {
                continue;
            }
            let points_a = ref_index.get(&(idx + delta));
            let points_b = ref_index.get(&(idx + 1 - delta));
            let matching = points_a == points_b;
            if !matching {
                return false;
            }
        }

        true
    }

    fn has_reflection_with_smudge_at(
        &self,
        idx: usize,
        len: usize,
        ref_index: &HashMap<usize, HashSet<usize>>,
    ) -> bool {
        let min_distance_to_edge = min(idx, len - idx);
        let mut found_smudge = false;

        let mut compared = HashSet::new();

        for delta in 0..=(min_distance_to_edge + 1) {
            if idx + delta >= len || idx + 1 - delta >= len {
                continue;
            }

            let points_a_idx = idx + delta;
            let points_b_idx = idx + 1 - delta;
            if compared.contains(&(points_a_idx, points_b_idx)) {
                continue;
            }

            let points_a = ref_index.get(&points_a_idx).unwrap();
            let points_b = ref_index.get(&points_b_idx).unwrap();
            compared.insert((points_a_idx, points_b_idx));
            compared.insert((points_b_idx, points_a_idx));
            let matching = points_a == points_b;

            let mut a = points_a.iter().collect::<Vec<_>>();
            a.sort();
            let mut b = points_b.iter().collect::<Vec<_>>();
            b.sort();
            println!(
                "idx: {}, delta: {}, matching: {}, a ({}): {:?}, b ({}): {:?}",
                idx,
                delta,
                matching,
                idx + delta,
                a,
                idx + 1 - delta,
                b
            );

            if !matching && found_smudge {
                println!("idx: {}, delta: {} --- found smudge twice!", idx, delta);
                // cannot have more than 1 smudge
                return false;
            }
            // Check if there's a smudge.
            // There must be only 1 difference between the sets, which means that the instersection
            // should contain the same number of elements of the set with the least amount of elements.
            let len_diff = (points_a.len() as isize - points_b.len() as isize).abs();
            if points_a.intersection(points_b).count() == min(points_a.len(), points_b.len())
                && len_diff == 1
            {
                println!("idx: {}, delta: {} --- found smudge!", idx, delta);
                found_smudge = true;
            }
        }

        println!("idx: {} --- EXITING WITH {}", idx, found_smudge);

        found_smudge
    }

    pub fn find_vertical_reflection(&self) -> Option<usize> {
        (0..self.width).find(|&col| self.has_reflection_at(col, self.width, &self.cols))
    }

    pub fn find_horizontal_reflection(&self) -> Option<usize> {
        (0..self.height).find(|&row| self.has_reflection_at(row, self.height, &self.rows))
    }

    pub fn find_vertical_reflection_with_smudge(&self) -> Option<usize> {
        (0..self.width).find(|&col| self.has_reflection_with_smudge_at(col, self.width, &self.cols))
    }

    pub fn find_horizontal_reflection_with_smudge(&self) -> Option<usize> {
        (0..self.height)
            .find(|&row| self.has_reflection_with_smudge_at(row, self.height, &self.rows))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_vertical_reflection() {
        let raw_map = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let map: Map = raw_map.parse().unwrap();
        let vertical_reflection = map.find_vertical_reflection().unwrap();
        assert_eq!(vertical_reflection, 4);
        let horizontal_reflection = map.find_horizontal_reflection();
        assert!(horizontal_reflection.is_none());
    }

    #[test]
    fn test_find_vertical_reflection2() {
        let raw_map = "####.##.#######
.####..####....
####.##.#######
.###.##.###.##.
.##..##..##....
.#...##...#.##.
#..######..####
#..#....#..####
.###.##.###.##.
#.#..##.##.####
#.#.#..#.#.#..#
####....#######
.##......##.##.";
        let map: Map = raw_map.parse().unwrap();
        let vertical_reflection = map.find_vertical_reflection().unwrap();
        assert_eq!(vertical_reflection, 12);
        let horizontal_reflection = map.find_horizontal_reflection();
        assert!(horizontal_reflection.is_none());
    }

    #[test]
    fn test_find_horizontal_reflection() {
        let raw_map = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let map: Map = raw_map.parse().unwrap();
        let horizontal_reflection = map.find_horizontal_reflection().unwrap();
        assert_eq!(horizontal_reflection, 3);
        let vertical_reflection = map.find_vertical_reflection();
        assert!(vertical_reflection.is_none());
    }

    #[test]
    fn test_find_horizontal_reflection2() {
        let raw_map = "#.###.#..
#.###.#..
..##.#.##
#.####.#.
#####...#
#.##.##..
..##.#...
##...###.
.#....#.#
#..#...##
#..#.#.##";
        let map: Map = raw_map.parse().unwrap();
        let horizontal_reflection = map.find_horizontal_reflection().unwrap();
        assert_eq!(horizontal_reflection, 0);
    }

    #[test]
    fn test_find_horizontal_reflection_with_smudge() {
        let raw_map = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let map: Map = raw_map.parse().unwrap();
        let horizontal_reflection = map.find_horizontal_reflection_with_smudge().unwrap();
        assert_eq!(horizontal_reflection, 2);
    }

    #[test]
    fn test_find_horizontal_reflection_with_smudge2() {
        let raw_map = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let map: Map = raw_map.parse().unwrap();
        let horizontal_reflection = map.find_horizontal_reflection_with_smudge().unwrap();
        assert_eq!(horizontal_reflection, 0);
    }
}
