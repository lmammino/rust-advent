use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    ops::Deref,
};

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{line_ending, one_of},
    combinator::{complete, eof},
    multi::many1,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Left = 0,
    Right = 1,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Left => write!(f, "L"),
            Direction::Right => write!(f, "R"),
        }
    }
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (input, c) = one_of("LR")(input)?;
    let direction = match c {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => unreachable!("Invalid direction: {}", c),
    };

    Ok((input, direction))
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, directions) = many1(parse_direction)(input)?;
    Ok((input, directions))
}

#[derive(Debug, Clone)]
pub struct Paths<'a>(HashMap<&'a str, [&'a str; 2]>);

impl<'a> Paths<'a> {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, from: &'a str, left: &'a str, right: &'a str) {
        self.0.insert(from, [left, right]);
    }

    fn get(&self, from: &'a str) -> Option<[&'a str; 2]> {
        self.0.get(from).copied()
    }

    pub fn next(&self, from: &'a str, direction: Direction) -> Option<&'a str> {
        self.get(from).map(|next| next[direction as usize])
    }
}

impl<'a> Deref for Paths<'a> {
    type Target = HashMap<&'a str, [&'a str; 2]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn parse_path_record(input: &str) -> IResult<&str, (&str, &str, &str)> {
    let label = is_a("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
    let (input, from) = label(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, left) = label(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, right) = label(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = alt((line_ending, eof))(input)?;

    Ok((input, (from, left, right)))
}

fn parse_paths(input: &str) -> IResult<&str, Paths> {
    let (input, records) = complete(many1(parse_path_record))(input)?;
    let mut paths = Paths::new();
    for (from, left, right) in records {
        paths.insert(from, left, right);
    }

    Ok((input, paths))
}

pub fn parse_input(input: &str) -> IResult<&str, (Vec<Direction>, Paths)> {
    let (input, directions) = parse_directions(input)?;
    let (input, _) = tuple((line_ending, line_ending))(input)?;
    let (input, paths) = parse_paths(input)?;

    Ok((input, (directions, paths)))
}
