mod point3d;
mod scanner;
use point3d::*;
use scanner::*;

pub fn part1(input: &str) -> u32 {
    let scanners = scanners_from_input(input);

    dbg!(scanners);
    414
}

pub fn part2(_input: &str) -> u32 {
    13000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 414);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 13000);
    }
}
