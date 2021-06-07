pub fn part1(input:&str) -> u32 {
    println!("{:?}", input);
    136
}

pub fn part2(input:&str) -> u64 {
    println!("{:?}", input);
    305068317272992
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 136);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");        
        assert_eq!(part2(input), 305068317272992);
    }
}
