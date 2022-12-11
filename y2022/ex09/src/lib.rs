use std::{
    collections::HashSet,
    ops::{AddAssign, Deref, DerefMut},
};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, one_of},
    combinator::map_res,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Knot {
    x: isize,
    y: isize,
}

impl Knot {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn is_adjacent(&self, other: &Self) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
}

impl AddAssign for Knot {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Debug, Clone)]
struct Rope<const N: usize>([Knot; N]);

impl<const N: usize> Deref for Rope<N> {
    type Target = [Knot; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for Rope<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        Self([Knot::default(); N])
    }

    fn head(&self) -> &Knot {
        &(self[0])
    }

    fn tail(&self) -> &Knot {
        &(self[self.len() - 1])
    }

    fn do_move(&mut self, m: &Move) {
        // move the head
        self[0] += m.delta();

        // follow movements of all the other knots
        for i in 1..self.len() {
            if !self[i].is_adjacent(&self[i - 1]) {
                if self[i].x != self[i - 1].x {
                    if self[i].x > self[i - 1].x {
                        self[i].x -= 1;
                    } else {
                        self[i].x += 1;
                    }
                }
                if self[i].y != self[i - 1].y {
                    if self[i].y > self[i - 1].y {
                        self[i].y -= 1;
                    } else {
                        self[i].y += 1;
                    }
                }
            }
        }
    }
}

enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn delta(&self) -> Knot {
        match self {
            Move::Up => Knot::new(0, -1),
            Move::Down => Knot::new(0, 1),
            Move::Left => Knot::new(-1, 0),
            Move::Right => Knot::new(1, 0),
        }
    }
}

fn parse_line(line: &str) -> IResult<&str, (Move, usize)> {
    let (input, (c, d)) = separated_pair(
        one_of("UDLR"),
        tag(" "),
        map_res(digit1, |s: &str| s.parse::<usize>()),
    )(line)?;

    match c {
        'U' => Ok((input, (Move::Up, d))),
        'D' => Ok((input, (Move::Down, d))),
        'L' => Ok((input, (Move::Left, d))),
        'R' => Ok((input, (Move::Right, d))),
        _ => unreachable!(),
    }
}

pub fn part1(input: &str) -> usize {
    let cmds = input.lines().map(|l| parse_line(l).unwrap().1);

    let mut rope = Rope::<2>::new();
    let mut tail_pos: HashSet<Knot> = HashSet::new();
    for (dir, units) in cmds {
        for _ in 0..units {
            rope.do_move(&dir);
            tail_pos.insert(*(rope.tail()));
        }
    }

    tail_pos.len()
}

pub fn part2(input: &str) -> usize {
    let cmds = input.lines().map(|l| parse_line(l).unwrap().1);

    let mut rope = Rope::<10>::new();
    let mut tail_pos: HashSet<Knot> = HashSet::new();
    for (dir, units) in cmds {
        for _ in 0..units {
            rope.do_move(&dir);
            tail_pos.insert(*(rope.tail()));
        }
    }

    tail_pos.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_move_part() {
        let mut rope = Rope::<2>::new();
        // R 4
        rope.do_move(&Move::Right);
        assert_eq!(rope.head(), &Knot::new(1, 0));
        assert_eq!(rope.tail(), &Knot::new(0, 0));
        rope.do_move(&Move::Right);
        assert_eq!(rope.head(), &Knot::new(2, 0));
        assert_eq!(rope.tail(), &Knot::new(1, 0));
        rope.do_move(&Move::Right);
        assert_eq!(rope.head(), &Knot::new(3, 0));
        assert_eq!(rope.tail(), &Knot::new(2, 0));
        rope.do_move(&Move::Right);
        assert_eq!(rope.head(), &Knot::new(4, 0));
        assert_eq!(rope.tail(), &Knot::new(3, 0));
        // U 4
        rope.do_move(&Move::Up);
        assert_eq!(rope.head(), &Knot::new(4, -1));
        assert_eq!(rope.tail(), &Knot::new(3, 0));
        rope.do_move(&Move::Up);
        assert_eq!(rope.head(), &Knot::new(4, -2));
        assert_eq!(rope.tail(), &Knot::new(4, -1));
        rope.do_move(&Move::Up);
        assert_eq!(rope.head(), &Knot::new(4, -3));
        assert_eq!(rope.tail(), &Knot::new(4, -2));
        rope.do_move(&Move::Up);
        assert_eq!(rope.head(), &Knot::new(4, -4));
        assert_eq!(rope.tail(), &Knot::new(4, -3));
        // L 3
        rope.do_move(&Move::Left);
        assert_eq!(rope.head(), &Knot::new(3, -4));
        assert_eq!(rope.tail(), &Knot::new(4, -3));
        rope.do_move(&Move::Left);
        assert_eq!(rope.head(), &Knot::new(2, -4));
        assert_eq!(rope.tail(), &Knot::new(3, -4));
        rope.do_move(&Move::Left);
        assert_eq!(rope.head(), &Knot::new(1, -4));
        assert_eq!(rope.tail(), &Knot::new(2, -4));
        // D 1
        rope.do_move(&Move::Down);
        assert_eq!(rope.head(), &Knot::new(1, -3));
        assert_eq!(rope.tail(), &Knot::new(2, -4));
        // R 4
        rope.do_move(&Move::Right);
        assert_eq!(rope.head(), &Knot::new(2, -3));
        assert_eq!(rope.tail(), &Knot::new(2, -4));
        rope.do_move(&Move::Right);
        assert_eq!(rope.head(), &Knot::new(3, -3));
        assert_eq!(rope.tail(), &Knot::new(2, -4));
        rope.do_move(&Move::Right);
        assert_eq!(rope.head(), &Knot::new(4, -3));
        assert_eq!(rope.tail(), &Knot::new(3, -3));
        rope.do_move(&Move::Right);
        assert_eq!(rope.head(), &Knot::new(5, -3));
        assert_eq!(rope.tail(), &Knot::new(4, -3));
    }

    #[test]
    fn test_part1_example() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 5710);
    }

    #[test]
    fn test_part2_example() {
        let input: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(part2(input), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2259);
    }
}
