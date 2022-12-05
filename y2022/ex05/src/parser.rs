use crate::{Move, Stacks};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, one_of},
    combinator::{map, map_res},
    sequence::delimited,
    IResult,
};

pub fn parse_crate(input: &str) -> IResult<&str, Option<char>> {
    let mut parser = map(
        delimited(tag("["), one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), tag("]")),
        Some,
    );
    parser(input)
}

pub fn parse_empty(input: &str) -> IResult<&str, Option<char>> {
    let mut parser = map(tag("   "), |_| None);
    parser(input)
}

pub fn parse_crate_or_empty(input: &str) -> IResult<&str, Option<char>> {
    alt((parse_crate, parse_empty))(input)
}

pub fn parse_crates_line(input: &str) -> IResult<&str, Vec<Option<char>>> {
    let mut parser = nom::multi::separated_list0(tag(" "), parse_crate_or_empty);
    parser(input)
}

pub fn parse_crates_stack(input: &str) -> IResult<&str, Vec<Vec<Option<char>>>> {
    nom::multi::separated_list0(tag("\n"), parse_crates_line)(input)
}

pub fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, what) = map_res(digit1, |s: &str| s.parse::<usize>())(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = map_res(digit1, |s: &str| s.parse::<usize>())(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = map_res(digit1, |s: &str| s.parse::<usize>())(input)?;
    Ok((
        input,
        Move {
            quantity: what,
            from,
            to,
        },
    ))
}

pub fn parse_input(input: &str) -> IResult<&str, (Stacks, impl Iterator<Item = Move> + '_)> {
    // splits at " 1"
    let (stacks_part, rest) = input.split_once(" 1").unwrap();
    let (_, moves_part) = rest.split_once("\n\n").unwrap();

    let (_, stacks) = parse_crates_stack(stacks_part)?;
    let moves = moves_part.lines().map(|line| parse_move(line).unwrap().1);

    Ok((input, (stacks.into(), moves)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_crate() {
        let result = parse_crate("[X]");
        assert_eq!(result, Ok(("", Some('X'))));
    }

    #[test]
    fn test_parse_empty() {
        let result = parse_empty("   ");
        assert_eq!(result, Ok(("", None)));
    }

    #[test]
    fn test_parse_crate_or_empty() {
        let result = parse_crate_or_empty("[X]");
        assert_eq!(result, Ok(("", Some('X'))));

        let result = parse_crate_or_empty("   ");
        assert_eq!(result, Ok(("", None)));
    }

    #[test]
    fn test_parse_crates_line() {
        let result = parse_crates_line("[P]     [L]         [T]            ");
        assert_eq!(
            result,
            Ok((
                "",
                Vec::from([
                    Some('P'),
                    None,
                    Some('L'),
                    None,
                    None,
                    Some('T'),
                    None,
                    None,
                    None,
                ])
            ))
        );
    }

    #[test]
    fn test_parse_crates_stack() {
        let result = parse_crates_stack(
            "[P]     [L]         [T]            \n[L]     [M] [G]     [G]     [S]    ",
        );
        assert_eq!(
            result,
            Ok((
                "",
                Vec::from([
                    Vec::from([
                        Some('P'),
                        None,
                        Some('L'),
                        None,
                        None,
                        Some('T'),
                        None,
                        None,
                        None,
                    ]),
                    Vec::from([
                        Some('L'),
                        None,
                        Some('M'),
                        Some('G'),
                        None,
                        Some('G'),
                        None,
                        Some('S'),
                        None
                    ])
                ])
            ))
        );
    }

    #[test]
    fn test_parse_move() {
        let result = parse_move("move 11 from 8 to 3");
        assert_eq!(
            result,
            Ok((
                "",
                Move {
                    quantity: 11,
                    from: 8,
                    to: 3
                }
            ))
        );
    }
}
