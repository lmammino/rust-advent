mod map;
use map::*;

pub fn part1(input: &str) -> usize {
    let map: Map<144, 41> = input.parse().unwrap();
    let paths_from_start = map.dijkstra(&map.start);
    let end_edge = paths_from_start[&map.end];
    end_edge.0
}

pub fn part2(input: &str) -> usize {
    // this could probably be optimised by making a reverse dijkstra from the end and then analysing all the scenic points
    // this way we would run dijkstra only once
    let map: Map<144, 41> = input.parse().unwrap();
    map.scenic_points
        .iter()
        .filter_map(|&p| {
            let paths = map.dijkstra(&p);
            paths.get(&map.end).map(|(dist, _)| *dist)
        })
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
