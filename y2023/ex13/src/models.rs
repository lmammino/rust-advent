use std::{cmp::min, fmt::Display, str::FromStr};

#[derive(Debug, Clone)]
pub struct Map {
    data: Vec<String>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.data {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<String> = s.lines().map(|line| line.to_owned()).collect();
        Ok(Map { data })
    }
}

impl Map {
    pub fn transpose(self) -> Map {
        let mut transposed = vec![Vec::new(); self.data[0].len()];
        for row in self.data {
            for (col, c) in row.chars().enumerate() {
                transposed[col].push(c);
            }
        }
        let data: Vec<String> = transposed
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect();
        Map { data }
    }

    pub fn find_reflection(&self, exact_num_errors: usize) -> Option<usize> {
        'outer: for (line_idx, _line) in self.data.iter().enumerate().skip(1) {
            let mut errors_found = 0;

            // find the two halves to compare by splitting between line_idx - 1 and line_idx
            let edge_dist = min(self.data.len() - line_idx, line_idx);
            let upper_half_range = line_idx - edge_dist..line_idx;
            let lower_half_range = (line_idx..(line_idx + edge_dist)).rev();

            // compare the data in the 2 halves
            let to_compare = upper_half_range.zip(lower_half_range);

            for (line_a_idx, line_b_idx) in to_compare {
                let line_a = &self.data[line_a_idx];
                let line_b = &self.data[line_b_idx];
                let mut line_a_chars = line_a.chars();
                let mut line_b_chars = line_b.chars();

                while let (Some(c_a), Some(c_b)) = (line_a_chars.next(), line_b_chars.next()) {
                    if c_a != c_b {
                        errors_found += 1;
                        if errors_found > exact_num_errors {
                            continue 'outer;
                        }
                    }
                }
            }
            if errors_found == exact_num_errors {
                return Some(line_idx);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_reflection() {
        let raw_map = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let map: Map = raw_map.parse().unwrap();
        let horizontal_reflection = map.find_reflection(0);
        let vertical_reflection = map.transpose().find_reflection(0);
        assert_eq!(vertical_reflection.unwrap(), 5);
        assert!(horizontal_reflection.is_none());
    }

    #[test]
    fn test_find_reflection2() {
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
        let horizontal_reflection = map.find_reflection(0);
        let transposed = map.transpose();
        let vertical_reflection = transposed.find_reflection(0);
        assert_eq!(vertical_reflection.unwrap(), 13);
        assert!(horizontal_reflection.is_none());
    }

    #[test]
    fn test_find_reflection3() {
        let raw_map = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let map: Map = raw_map.parse().unwrap();
        let horizontal_reflection = map.find_reflection(0);
        let vertical_reflection = map.transpose().find_reflection(0);
        assert_eq!(horizontal_reflection.unwrap(), 4);
        assert!(vertical_reflection.is_none());
    }

    #[test]
    fn test_find_reflection4() {
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
        let horizontal_reflection = map.find_reflection(0);
        let vertical_reflection = map.transpose().find_reflection(0);
        assert_eq!(horizontal_reflection.unwrap(), 1);
        assert!(vertical_reflection.is_none());
    }

    #[test]
    fn test_find_reflection_with_smudge() {
        let raw_map = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let map: Map = raw_map.parse().unwrap();
        let horizontal_reflection = map.find_reflection(1);
        let vertical_reflection = map.transpose().find_reflection(1);
        assert_eq!(horizontal_reflection.unwrap(), 3);
        assert!(vertical_reflection.is_none());
    }

    #[test]
    fn test_find_reflection_with_smudge2() {
        let raw_map = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let map: Map = raw_map.parse().unwrap();
        let horizontal_reflection = map.find_reflection(1);
        let vertical_reflection = map.transpose().find_reflection(1);
        assert_eq!(horizontal_reflection.unwrap(), 1);
        assert!(vertical_reflection.is_none());
    }
}
