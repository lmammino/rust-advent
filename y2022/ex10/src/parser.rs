use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt},
    IResult,
};

use crate::program::Cmd;

pub(crate) fn parse_line(input: &str) -> IResult<&str, Cmd> {
    let (input, cmd) = alt((parse_noop, parse_addx))(input)?;
    Ok((input, cmd))
}

fn parse_noop(input: &str) -> IResult<&str, Cmd> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Cmd::Noop))
}

fn parse_addx(input: &str) -> IResult<&str, Cmd> {
    let (input, _) = tag("addx ")(input)?;
    let (input, sign) = opt(tag("-"))(input)?;
    let sign = sign.map(|_| -1).unwrap_or(1);
    let (input, unsigned_value) = map_res(digit1, |s: &str| s.parse::<i32>())(input)?;
    Ok((input, Cmd::AddX(unsigned_value * sign)))
}
