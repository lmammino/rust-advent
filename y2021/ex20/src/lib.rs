use std::{collections::HashSet, fmt::Display, str::FromStr};

#[derive(Debug)]
struct Iea(HashSet<usize>);

impl FromStr for Iea {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let light_pixels: HashSet<usize> = s
            .chars()
            .enumerate()
            .filter_map(|(i, c)| if c == '#' { Some(i) } else { None })
            .collect();

        Ok(Iea(light_pixels))
    }
}

impl Iea {
    fn is_light(&self, pos: usize) -> bool {
        self.0.contains(&pos)
    }
}

#[derive(Debug)]
struct Image {
    min_row: isize,
    max_row: isize,
    min_col: isize,
    max_col: isize,
    default: bool,
    pixels: HashSet<(isize, isize)>,
}

impl FromStr for Image {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pixels: HashSet<(isize, isize)> = Default::default();
        let mut max_row = 0_isize;
        let mut max_col = 0_isize;
        for (row_id, line) in s.lines().enumerate() {
            max_row += 1;
            max_col = 0;
            for (col_id, c) in line.chars().enumerate() {
                max_col += 1;
                if c == '#' {
                    pixels.insert((row_id as isize, col_id as isize));
                }
            }
        }

        Ok(Image {
            pixels,
            min_col: 0,
            min_row: 0,
            max_col,
            max_row,
            default: false,
        })
    }
}

impl Image {
    fn enhance_pixel(&self, row_id: isize, col_id: isize, iea: &Iea) -> bool {
        let mut i = 0;
        let mut vals: [bool; 9] = [false; 9];
        for delta_row in -1..=1 {
            let new_row: isize = row_id + delta_row;
            for delta_col in -1..=1 {
                let new_col: isize = col_id + delta_col;
                // if out of bounds (in infinite space) use the default
                if new_row < self.min_row
                    || new_row >= self.max_row
                    || new_col < self.min_col
                    || new_col >= self.max_col
                {
                    vals[i] = self.default;
                } else {
                    // else check the current value
                    vals[i] = self.pixels.contains(&(new_row, new_col));
                }
                i += 1;
            }
        }

        let iea_idx = vals
            .iter()
            .rev()
            .enumerate()
            .map(|(i, v)| (*v as usize) << i)
            .sum();

        iea.is_light(iea_idx)
    }

    fn enhance(&self, iea: &Iea) -> Image {
        let mut pixels: HashSet<(isize, isize)> = Default::default();
        let min_row = self.min_row - 1;
        let max_row = self.max_row + 1;
        let min_col = self.min_col - 1;
        let max_col = self.max_col + 1;
        for row_id in min_row..max_row {
            for col_id in min_col..max_col {
                if self.enhance_pixel(row_id, col_id, iea) {
                    pixels.insert((row_id, col_id));
                }
            }
        }

        let default_iea_idx = [self.default; 9]
            .iter()
            .rev()
            .enumerate()
            .map(|(i, v)| (*v as usize) << i)
            .sum();
        let default = iea.is_light(default_iea_idx);

        Image {
            pixels,
            min_row,
            max_row,
            min_col,
            max_col,
            default,
        }
    }

    fn count_light_pixels(&self) -> usize {
        self.pixels.len()
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row_id in self.min_row..self.max_row {
            for col_id in self.min_col..self.max_col {
                if self.pixels.contains(&(row_id, col_id)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn part1(input: &str) -> usize {
    let (raw_iea, raw_image) = input.split_once("\n\n").unwrap();
    let iea: Iea = raw_iea.parse().unwrap();
    let image: Image = raw_image.parse().unwrap();
    let image = image.enhance(&iea);
    let image = image.enhance(&iea);

    image.count_light_pixels()
}

pub fn part2(input: &str) -> usize {
    let (raw_iea, raw_image) = input.split_once("\n\n").unwrap();
    let iea: Iea = raw_iea.parse().unwrap();
    let mut image: Image = raw_image.parse().unwrap();

    for _ in 0..50 {
        image = image.enhance(&iea);
    }

    image.count_light_pixels()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_example() {
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

        let (raw_iea, raw_image) = input.split_once("\n\n").unwrap();
        let iea: Iea = raw_iea.parse().unwrap();
        let image: Image = raw_image.parse().unwrap();

        let image = image.enhance(&iea);
        let mut image = image.enhance(&iea);

        // after 2 times
        assert_eq!(image.count_light_pixels(), 35);

        for _ in 0..48 {
            image = image.enhance(&iea);
        }
        // after 50 times
        assert_eq!(image.count_light_pixels(), 3351);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 4917);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 16389);
    }
}
