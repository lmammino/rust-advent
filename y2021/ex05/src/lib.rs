use std::{cmp::Ordering, collections::BTreeMap, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone)]
struct Segment {
    p1: Point,
    p2: Point,
}

impl Segment {
    fn is_straight(&self) -> bool {
        self.p1.x == self.p2.x || self.p1.y == self.p2.y
    }

    fn walk(&self) -> SegmentWalker {
        SegmentWalker::new(self)
    }
}

impl FromStr for Segment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p1, p2) = s.split_once(" -> ").unwrap();
        let (x1, y1) = p1.split_once(',').unwrap();
        let (x2, y2) = p2.split_once(',').unwrap();
        Ok(Segment {
            p1: Point {
                x: x1.parse().unwrap(),
                y: y1.parse().unwrap(),
            },
            p2: Point {
                x: x2.parse().unwrap(),
                y: y2.parse().unwrap(),
            },
        })
    }
}

#[derive(Debug)]
struct SegmentWalker {
    x2: u32,
    y2: u32,
    incr_x: i32,
    incr_y: i32,
    curr_x: u32,
    curr_y: u32,
    completed: bool,
}

impl SegmentWalker {
    fn new(segment: &Segment) -> Self {
        let x1 = segment.p1.x;
        let y1 = segment.p1.y;
        let x2 = segment.p2.x;
        let y2 = segment.p2.y;
        let incr_x = match x2.cmp(&x1) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            Ordering::Equal => 0,
        };
        let incr_y = match y2.cmp(&y1) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            Ordering::Equal => 0,
        };
        let curr_x = x1;
        let curr_y = y1;
        SegmentWalker {
            x2,
            y2,
            incr_x,
            incr_y,
            curr_x,
            curr_y,
            completed: false,
        }
    }
}

impl Iterator for SegmentWalker {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.completed {
            return None;
        }

        let p = Point {
            x: self.curr_x,
            y: self.curr_y,
        };

        if self.curr_x == self.x2 && self.curr_y == self.y2 {
            self.completed = true;
        } else {
            self.curr_x = (self.incr_x + self.curr_x as i32) as u32;
            self.curr_y = (self.incr_y + self.curr_y as i32) as u32;
        }

        Some(p)
    }
}

#[derive(Debug, Default)]
struct Space(BTreeMap<Point, usize>);

impl Space {
    fn draw_segment(&mut self, segment: &Segment) {
        for p in segment.walk() {
            let count = self.0.entry(p).or_default();
            *count += 1;
        }
    }
}

pub fn part1(input: &str) -> usize {
    let straight_segments = input
        .lines()
        .map(|s| s.parse::<Segment>().unwrap())
        .filter(Segment::is_straight);
    let mut space = Space::default();
    for segment in straight_segments {
        space.draw_segment(&segment);
    }

    space.0.iter().filter(|(_, x)| **x > 1).count()
}

pub fn part2(input: &str) -> usize {
    let segments = input.lines().map(|s| s.parse::<Segment>().unwrap());
    let mut space = Space::default();
    for segment in segments {
        space.draw_segment(&segment);
    }

    space.0.iter().filter(|(_, x)| **x > 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walk_line() {
        // diagonal
        let segment = Segment {
            p1: Point { x: 0, y: 0 },
            p2: Point { x: 2, y: 2 },
        };
        let points: Vec<Point> = segment.walk().collect();
        assert_eq!(
            points,
            vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 2 }
            ]
        );

        // diagonal reverse
        let segment = Segment {
            p1: Point { x: 2, y: 2 },
            p2: Point { x: 0, y: 0 },
        };
        let points: Vec<Point> = segment.walk().collect();
        assert_eq!(
            points,
            vec![
                Point { x: 2, y: 2 },
                Point { x: 1, y: 1 },
                Point { x: 0, y: 0 },
            ]
        );

        // vertical
        let segment = Segment {
            p1: Point { x: 0, y: 0 },
            p2: Point { x: 0, y: 2 },
        };
        let points: Vec<Point> = segment.walk().collect();
        assert_eq!(
            points,
            vec![
                Point { x: 0, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 0, y: 2 }
            ]
        );
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 6267);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 20196);
    }
}
