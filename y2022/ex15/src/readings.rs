use nom::{bytes::complete::tag, character::complete::digit1, combinator::opt, IResult};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Pos {
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
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, s_x) = parse_i64(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, s_y) = parse_i64(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, b_x) = parse_i64(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, b_y) = parse_i64(input)?;
    let p1 = Pos { x: s_x, y: s_y };
    let p2 = Pos { x: b_x, y: b_y };
    let dist = p1.dist(&p2);
    Ok((input, (p1, p2, dist)))
}

pub(crate) fn parse(input: &str) -> impl Iterator<Item = (Pos, Pos, i64)> + '_ {
    input.lines().map(|l| parse_line(l).unwrap().1)
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
            (9323_i64 - 245599_i64).abs() + (979154_i64 - 778791_i64).abs(),
        );
        assert_eq!(parse_line(input), Ok(("", expected)));
    }
}
