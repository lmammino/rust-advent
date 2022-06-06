use std::{cmp::Ordering, str::FromStr};

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
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, s) = s.split_once("target area: ").unwrap();
        let (xrange, yrange) = s.split_once(", ").unwrap();
        let (xstart, xend) = &xrange[2..].split_once("..").unwrap();
        let (ystart, yend) = &yrange[2..].split_once("..").unwrap();
        let (min_x, max_x) = (xstart.parse().unwrap(), xend.parse().unwrap());
        let (min_y, max_y) = (ystart.parse().unwrap(), yend.parse().unwrap());

        Ok(TargetArea {
            min_x,
            max_x,
            min_y,
            max_y,
        })
    }
}

#[derive(Debug)]
struct Probe {
    vx: i32,
    vy: i32,
    x: i32,
    y: i32,
}

impl Probe {
    fn new(vx: i32, vy: i32) -> Self {
        Probe { vx, vy, x: 0, y: 0 }
    }

    fn hits_target(&mut self, target: &TargetArea) -> bool {
        let max_x = target.max_x;
        let min_y = target.min_y;

        loop {
            // is hit, returns true
            if target.contains(self.x, self.y) {
                return true;
            }

            // simulates another step
            self.next();

            // if overshoot it is missed, breaks and return false
            if self.x > max_x || self.y < min_y {
                break;
            }
        }

        false
    }
}

impl Iterator for Probe {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let new_x = self.x + self.vx;
        let new_y = self.y + self.vy;
        let new_vx = self.vx
            + match self.vx.cmp(&0) {
                Ordering::Equal => 0,
                Ordering::Greater => -1,
                Ordering::Less => 1,
            };
        let new_vy = self.vy - 1;
        self.x = new_x;
        self.y = new_y;
        self.vx = new_vx;
        self.vy = new_vy;

        Some(())
    }
}

pub fn part1(input: &str) -> i32 {
    // for this part x does not really matter, we just need to make sure we land on y.
    // If we try to let y land on the lowest point of the target area on y axis that should
    // allow us to maximise the max height. Max height is reached when vy is 0
    // (this is the point after which) the probe starts to fall down.
    let target: TargetArea = input.parse().unwrap();
    let min_y = target.min_y;
    let mut vy = -min_y - 1;
    let mut y = 0;
    while vy > 0 {
        y += vy;
        vy -= 1;
    }
    y
}

pub fn part2(input: &str) -> i32 {
    let target: TargetArea = input.parse().unwrap();
    let min_y = target.min_y;
    let mut hits = 0;
    for vx in 0..=target.max_x {
        for vy in min_y..=-min_y - 1 {
            let mut probe = Probe::new(vx, vy);
            if probe.hits_target(&target) {
                hits += 1;
            }
        }
    }

    hits
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
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 3540);
    }
}
