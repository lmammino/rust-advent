use anyhow::{Context, Result};
use std::{cmp::Ordering, str::FromStr};

#[derive(Debug)]
struct RectangularRange {
    min_x: i32,
    max_x: i32,
    max_y: i32,
    x: i32,
    y: i32,
}

impl RectangularRange {
    fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Self {
        RectangularRange {
            min_x,
            max_x,
            max_y,
            x: min_x,
            y: min_y,
        }
    }
}

impl Iterator for RectangularRange {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        // for x in min_x..=max_x {
        //      for y in min_y..=max_y {
        if self.x > self.max_x {
            self.x = self.min_x;
            self.y += 1;

            if self.y > self.max_y {
                return None;
            }
        }

        let item = (self.x, self.y);
        self.x += 1;

        Some(item)
    }
}

#[derive(Debug)]
struct TargetArea {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl TargetArea {
    fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }
}

impl FromStr for TargetArea {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<TargetArea> {
        let error_msg = "Input string not conform to spec: `target area: x={min_x}..{max_x}, y={min_y}..{max_y}`";

        let (_, s) = s.split_once("target area: ").context(error_msg)?;
        let (xrange, yrange) = s.split_once(", ").context(error_msg)?;
        let (xstart, xend) = &xrange[2..].split_once("..").context(error_msg)?;
        let (ystart, yend) = &yrange[2..].split_once("..").context(error_msg)?;
        let (min_x, max_x) = (
            xstart.parse().context("cannot parse min_x to i32")?,
            xend.parse().context("cannot parse max_x to i32")?,
        );
        let (min_y, max_y) = (
            ystart.parse().context("cannot parse min_y to i32")?,
            yend.parse().context("cannot parse max_y to i32")?,
        );

        Ok(TargetArea {
            min_x,
            max_x,
            min_y,
            max_y,
        })
    }
}

fn probe_hits_target(vx: i32, vy: i32, target: &TargetArea) -> bool {
    let max_x = target.max_x;
    let min_y = target.min_y;

    let mut x = 0;
    let mut y = 0;
    let mut vx = vx;
    let mut vy = vy;

    loop {
        // is hit, returns true
        if target.contains(x, y) {
            return true;
        }

        // simulates another step
        let new_x = x + vx;
        let new_y = y + vy;
        let new_vx = vx
            + match vx.cmp(&0) {
                Ordering::Equal => 0,
                Ordering::Greater => -1,
                Ordering::Less => 1,
            };
        let new_vy = vy - 1;
        x = new_x;
        y = new_y;
        vx = new_vx;
        vy = new_vy;

        // if overshoot it is missed, breaks and return false
        if x > max_x || y < min_y {
            break;
        }
    }

    false
}

pub fn part1(input: &str) -> i32 {
    // we don't care about x because we can decouple the 2 axis and we focus only on finding max y
    //
    // starting from 0 we will eventually reach max y.
    // If we pick a random y velocity, let's say 5 we end up with the following y coordinates
    // for every step:
    //
    //
    // 0,5,9,12,14,15,14,12,9,5,0,-5,-9
    //             ^--- this is the point where we start to go down (y velocity is zero)
    //
    //
    // if we take initial vy as -min_y - 1 we will reach the highest y and when we reach 0
    // we will have the highest velocity that will still allow to hit the rectangle (this will be in 1 final step).
    let target: TargetArea = input.parse().unwrap();
    let min_y = target.min_y;
    let vy = -min_y - 1;
    vy * (vy + 1) / 2
}

pub fn part2(input: &str) -> usize {
    let target: TargetArea = input.parse().unwrap();
    let min_y = target.min_y;

    RectangularRange::new(0, target.min_y, target.max_x, -min_y - 1)
        .filter(|(x, y)| probe_hits_target(*x, *y, &target))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 11175);
    }

    #[test]
    fn test_part1_roberto() {
        let input = "target area: x=25..67, y=-260..-200";
        assert_eq!(part1(input), 33670);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 3540);
    }

    #[test]
    fn test_rectangular_range() {
        let range = RectangularRange::new(1, 1, 3, 4);
        let values: Vec<(i32, i32)> = range.collect();
        let expected = vec![
            (1, 1),
            (2, 1),
            (3, 1),
            (1, 2),
            (2, 2),
            (3, 2),
            (1, 3),
            (2, 3),
            (3, 3),
            (1, 4),
            (2, 4),
            (3, 4),
        ];
        assert_eq!(values, expected);
    }
}
