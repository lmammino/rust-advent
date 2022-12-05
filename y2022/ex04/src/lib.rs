use nom::sequence::separated_pair;
use nom::{character::complete::digit1, combinator::map_res, IResult};
use std::ops::RangeInclusive;
use std::str::FromStr;

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    let mut parser = separated_pair(
        map_res(digit1, u32::from_str),
        nom::character::complete::char('-'),
        map_res(digit1, u32::from_str),
    );
    let (input, (min, max)) = parser(input)?;
    Ok((input, min..=max))
}

fn parse_line(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    let (input, first) = parse_range(input)?;
    let (input, _) = nom::character::complete::char(',')(input)?;
    let (input, second) = parse_range(input)?;
    Ok((input, (first, second)))
}

fn parse_input(
    input: &str,
) -> impl Iterator<Item = (RangeInclusive<u32>, RangeInclusive<u32>)> + '_ {
    input.lines().map(|line| parse_line(line).unwrap().1)
}

fn range_contains(range1: &RangeInclusive<u32>, range2: &RangeInclusive<u32>) -> bool {
    range1.contains(range2.start()) && range1.contains(range2.end())
}

fn range_overlap(range1: &RangeInclusive<u32>, range2: &RangeInclusive<u32>) -> bool {
    range1.contains(range2.start()) || range2.contains(range1.start())
}

pub fn part1(input: &str) -> usize {
    parse_input(input)
        .filter(|(range1, range2)| range_contains(range1, range2) || range_contains(range2, range1))
        .count()
}

pub fn part2(input: &str) -> usize {
    parse_input(input)
        .filter(|(range1, range2)| range_overlap(range1, range2) || range_contains(range2, range1))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 547);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 843);
    }
}
