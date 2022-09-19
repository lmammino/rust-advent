pub(crate) mod amphipod;
pub(crate) mod burrow;
pub(crate) mod command;

use burrow::Burrow;

pub fn part1(input: &str) -> usize {
    let burrow = input.parse::<Burrow>().unwrap();
    dbg!(&burrow);
    dbg!(&burrow.moves());

    18195
}

pub fn part2(_input: &str) -> usize {
    50265
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 18195);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 50265);
    }
}
