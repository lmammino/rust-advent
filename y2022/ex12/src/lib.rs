mod map;
use map::*;

pub fn part1(input: &str) -> usize {
    let map: Map<144, 41> = input.parse().unwrap();
    let paths_from_start = map.find_paths(&map.start, false);
    let end_edge = paths_from_start[&map.end];
    end_edge.0
}

pub fn part2(input: &str) -> usize {
    let map: Map<144, 41> = input.parse().unwrap();
    let walkable_paths = map.find_paths(&map.end, true);

    map.scenic_points
        .iter()
        .filter_map(|&p| walkable_paths.get(&p).map(|(dist, _)| *dist))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 423);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 416);
    }
}
