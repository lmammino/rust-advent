use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, opt},
    sequence::{preceded, tuple},
    IResult,
};
use regex::Regex;

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(r"Sensor at x=(?P<x1>[-]?\d+), y=(?P<y1>[-]?\d+): closest beacon is at x=(?P<x2>[-]?\d+), y=(?P<y2>[-]?\d+)").unwrap();
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pos {
    pub(crate) x: i64,
    pub(crate) y: i64,
}

impl Pos {
    pub(crate) fn dist(&self, other: &Pos) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub(crate) fn inside_square(&self, start: i64, end: i64) -> bool {
        self.x >= start && self.x <= end && self.y >= start && self.y <= end
    }
}

fn parse_i64(input: &str) -> IResult<&str, i64> {
    let (input, sign) = opt(tag("-"))(input)?;
    let (input, value) = digit1(input)?;
    let mut value = value.parse::<i64>().unwrap();
    if sign.is_some() {
        value *= -1;
    }
    Ok((input, value))
}

fn parse_line(input: &str) -> IResult<&str, (Pos, Pos, i64)> {
    // Sensor at x=9323, y=979154: closest beacon is at x=-245599, y=778791
    let (input, (s_x, s_y, b_x, b_y)) = all_consuming(tuple((
        preceded(tag("Sensor at x="), parse_i64),
        preceded(tag(", y="), parse_i64),
        preceded(tag(": closest beacon is at x="), parse_i64),
        preceded(tag(", y="), parse_i64),
    )))(input)?;
    let p1 = Pos { x: s_x, y: s_y };
    let p2 = Pos { x: b_x, y: b_y };
    let dist = p1.dist(&p2);
    Ok((input, (p1, p2, dist)))
}

pub fn parse(input: &str) -> impl Iterator<Item = (Pos, Pos, i64)> + '_ {
    input.lines().map(|l| parse_line(l).unwrap().1)
}

pub(crate) fn parse_line_regex(line: &str) -> (Pos, Pos, i64) {
    let re = Regex::new(
        r"Sensor at x=(?P<x1>[-]?\d+), y=(?P<y1>[-]?\d+): closest beacon is at x=(?P<x2>[-]?\d+), y=(?P<y2>[-]?\d+)",
    )
    .unwrap();

    let captures = re.captures(line).unwrap();
    let sensor = Pos {
        x: captures["x1"].parse().unwrap(),
        y: captures["y1"].parse().unwrap(),
    };
    let beacon = Pos {
        x: captures["x2"].parse().unwrap(),
        y: captures["y2"].parse().unwrap(),
    };
    let dist = sensor.dist(&beacon);
    (sensor, beacon, dist)
}

pub fn parse_regex(input: &str) -> impl Iterator<Item = (Pos, Pos, i64)> + '_ {
    input.lines().map(parse_line_regex)
}

pub(crate) fn parse_line_regex_lazy(line: &str) -> (Pos, Pos, i64) {
    let captures = LINE_REGEX.captures(line).unwrap();
    let sensor = Pos {
        x: captures["x1"].parse().unwrap(),
        y: captures["y1"].parse().unwrap(),
    };
    let beacon = Pos {
        x: captures["x2"].parse().unwrap(),
        y: captures["y2"].parse().unwrap(),
    };
    let dist = sensor.dist(&beacon);
    (sensor, beacon, dist)
}

pub fn parse_regex_lazy(input: &str) -> impl Iterator<Item = (Pos, Pos, i64)> + '_ {
    input.lines().map(parse_line_regex_lazy)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input = "Sensor at x=9323, y=979154: closest beacon is at x=-245599, y=778791";
        let expected = (
            Pos { x: 9323, y: 979154 },
            Pos {
                x: -245599,
                y: 778791,
            },
            455285,
        );
        assert_eq!(parse_line(input), Ok(("", expected)));
    }

    #[test]
    fn test_parse_line_regex() {
        let input = "Sensor at x=9323, y=979154: closest beacon is at x=-245599, y=778791";
        let expected = (
            Pos { x: 9323, y: 979154 },
            Pos {
                x: -245599,
                y: 778791,
            },
            455285,
        );
        assert_eq!(parse_line_regex(input), expected);
    }
}
