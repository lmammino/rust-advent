use models::{find_loop, get_enclosed, Map};
mod models;

pub fn part1(input: &str) -> usize {
    let map: Map<140, 140> = input.parse().unwrap();
    let path = find_loop(&map).unwrap();
    path.len() / 2
}

pub fn part2(input: &str) -> usize {
    let map: Map<140, 140> = input.parse().unwrap();
    let enclosed = get_enclosed(&map);
    enclosed.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 6697);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 423);
    }
}
