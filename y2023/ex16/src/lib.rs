use crate::models::Direction;
use models::Map;

mod models;

pub fn part1(input: &str) -> usize {
    let map = Map::<110, 110>::new(input);
    let energized_pos = map.simulate((0, 0), Direction::Right);
    energized_pos.len()
}

pub fn part2(input: &str) -> usize {
    let map = Map::<110, 110>::new(input);
    let top_down = (0..110).map(|x| ((x, 0), Direction::Down));
    let left_right = (0..110).map(|y| ((0, y), Direction::Right));
    let bottom_up = (0..110).map(|x| ((x, 109), Direction::Up));
    let right_left = (0..110).map(|y| ((109, y), Direction::Left));
    top_down
        .chain(left_right)
        .chain(bottom_up)
        .chain(right_left)
        .map(|(position, direction)| map.simulate(position, direction).len())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 8034);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 8225);
    }
}
