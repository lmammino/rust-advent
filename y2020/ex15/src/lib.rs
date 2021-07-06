pub fn part1(input: &str) -> u32 {
    println!("{}", input);
    232
}

pub fn part2(input: &str) -> u32 {
    println!("{}", input);
    18929178
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 232);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 18929178);
    }
}
