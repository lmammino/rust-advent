use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub mod rectangle;
use rectangle::*;

#[derive(Debug)]
struct Command {
    on: bool,
    x_range: (isize, isize),
    y_range: (isize, isize),
    z_range: (isize, isize),
}

impl Command {
    fn in_cube(&self, max: isize) -> bool {
        let min = -max;

        self.x_range.0 >= min
            && self.x_range.1 <= max
            && self.y_range.0 >= min
            && self.y_range.1 <= max
            && self.z_range.0 >= min
            && self.z_range.1 <= max
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (on_raw, ranges_raw) = s.split_once(' ').unwrap();
        let on = on_raw == "on";
        let mut range_parts = ranges_raw.split(',');
        let (x_range_raw, y_range_raw, z_range_raw) = (
            range_parts.next().unwrap(),
            range_parts.next().unwrap(),
            range_parts.next().unwrap(),
        );
        let mut x_range_iter = x_range_raw[2..]
            .split("..")
            .map(|x| x.parse::<isize>().unwrap())
            .take(2);
        let x_range = (x_range_iter.next().unwrap(), x_range_iter.next().unwrap());
        let mut y_range_iter = y_range_raw[2..]
            .split("..")
            .map(|y| y.parse::<isize>().unwrap())
            .take(2);
        let y_range = (y_range_iter.next().unwrap(), y_range_iter.next().unwrap());
        let mut z_range_iter = z_range_raw[2..]
            .split("..")
            .map(|z| z.parse::<isize>().unwrap())
            .take(2);
        let z_range = (z_range_iter.next().unwrap(), z_range_iter.next().unwrap());

        Ok(Command {
            on,
            x_range,
            y_range,
            z_range,
        })
    }
}

pub fn part1(input: &str) -> usize {
    let mut space: HashSet<(isize, isize, isize)> = Default::default();

    let commands = input
        .lines()
        .map(|l| l.parse::<Command>().unwrap())
        .filter(|c| c.in_cube(50));

    for command in commands {
        for x in command.x_range.0..=command.x_range.1 {
            for y in command.y_range.0..=command.y_range.1 {
                for z in command.z_range.0..=command.z_range.1 {
                    match command.on {
                        true => space.insert((x, y, z)),
                        false => space.remove(&(x, y, z)),
                    };
                }
            }
        }
    }

    space.len()
}

pub fn part2(input: &str) -> usize {
    let commands = input.lines().map(|l| l.parse::<Command>().unwrap());
    let commands2 = commands.clone();

    let mut z_ranges: HashSet<isize> = Default::default();
    for command in commands {
        z_ranges.insert(command.z_range.0);
        z_ranges.insert(command.z_range.1 + 1);
    }
    let mut z_ranges_sorted: Vec<isize> = z_ranges.into_iter().collect();
    z_ranges_sorted.sort();

    let mut z_tickness: HashMap<isize, usize> = Default::default();

    for (a, b) in z_ranges_sorted.iter().tuple_windows() {
        z_tickness.insert(*a, (b - a) as usize);
    }

    let mut planes: HashMap<isize, RectangleList> =
        HashMap::from_iter(z_tickness.iter().map(|(z, _)| (*z, RectangleList::new())));

    for command in commands2 {
        for z in z_ranges_sorted.iter() {
            if command.z_range.0 <= *z && command.z_range.1 >= *z {
                let rectangles = planes.get(z).unwrap();
                let rectangle_to_subtract = Rectangle::new(
                    command.x_range.0,
                    command.y_range.0,
                    command.x_range.1 + 1,
                    command.y_range.1 + 1,
                );
                let mut diffed = diff(rectangles, &rectangle_to_subtract);
                if command.on {
                    diffed.push(rectangle_to_subtract);
                }
                planes.insert(*z, diffed);
            }
        }
    }

    let mut num_cubes = 0;

    for (z, rectangles) in planes {
        let tickness = z_tickness.get(&z).unwrap();
        for rectangle in rectangles {
            num_cubes += rectangle.area() * tickness;
        }
    }

    num_cubes
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 561032);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1322825263376414);
    }
}
