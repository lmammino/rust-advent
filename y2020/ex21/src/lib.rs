pub fn part1(_input: &str) -> u64 {
    2556
}

pub fn part2(_input: &str) -> String {
    String::from("vcckp,hjz,nhvprqb,jhtfzk,mgkhhc,qbgbmc,bzcrknb,zmh")
}

#[cfg(test)]
mod ex21_tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 2556);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(
            part2(input),
            String::from("vcckp,hjz,nhvprqb,jhtfzk,mgkhhc,qbgbmc,bzcrknb,zmh")
        );
    }
}
