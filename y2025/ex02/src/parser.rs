use std::ops::RangeInclusive;
use winnow::combinator::separated;
use winnow::Result;
use winnow::{ascii::digit1, Parser};

fn parse_range(input: &mut &str) -> Result<RangeInclusive<u64>> {
    let start = digit1.try_map(str::parse).parse_next(input)?;
    let _ = '-'.parse_next(input)?;
    let end = digit1.try_map(str::parse).parse_next(input)?;
    Ok(RangeInclusive::new(start, end))
}

pub(crate) fn parse_input(input: &mut &str) -> Result<Vec<RangeInclusive<u64>>> {
    separated(1.., parse_range, ",").parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_range() {
        let mut input = "5-10";
        let range = parse_range(&mut input).unwrap();
        assert_eq!(range, 5..=10);
    }

    #[test]
    fn test_parse_input() {
        let mut input = "5-10,15-20,25-30";
        let ranges = parse_input(&mut input).unwrap();
        assert_eq!(ranges, vec![5..=10, 15..=20, 25..=30]);
    }
}
