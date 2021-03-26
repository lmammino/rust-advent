pub fn part1(input: &str) -> u32 {
    println!("{}", input);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 0)
    }
}
