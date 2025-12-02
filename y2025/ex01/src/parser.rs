use winnow::ascii::digit1;
use winnow::combinator::alt;
use winnow::prelude::*;
use winnow::Result;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum DialOp {
    TurnLeft(u16),
    TurnRight(u16),
}

pub(crate) fn parse_dial_op(input: &mut &str) -> Result<DialOp> {
    let dir = alt(('L', 'R')).parse_next(input)?;
    let steps = digit1.try_map(str::parse::<u16>).parse_next(input)?;
    let op = match dir {
        'L' => DialOp::TurnLeft(steps),
        'R' => DialOp::TurnRight(steps),
        _ => unreachable!(),
    };
    Ok(op)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dial_op() {
        let mut input = "L30";
        let op = parse_dial_op(&mut input).unwrap();
        assert_eq!(op, DialOp::TurnLeft(30));
        let mut input = "R70";
        let op = parse_dial_op(&mut input).unwrap();
        assert_eq!(op, DialOp::TurnRight(70));
    }
}
