mod cave;
use cave::*;

pub fn part1(input: &str) -> usize {
    let mut cave: Cave = input.parse().unwrap();
    let abyss_line = cave.tiles.keys().map(|pos| pos.y).max().unwrap() + 1;
    let mut num_grains = 0;

    'main_loop: loop {
        let mut grain_moves = cave.next_grain();
        for pos in grain_moves.by_ref() {
            if pos.y > abyss_line {
                break 'main_loop;
            }
        }
        cave.tiles.insert(grain_moves.pos, Block::Sand);
        num_grains += 1;
    }

    num_grains
}

pub fn part2(input: &str) -> u64 {
    let mut cave: Cave = input.parse().unwrap();
    let abyss_line = cave.tiles.keys().map(|pos| pos.y).max().unwrap() + 1;
    cave.floor_level = Some(abyss_line + 1);
    let mut num_grains = 0;

    loop {
        let mut grain_moves = cave.next_grain();
        for _ in grain_moves.by_ref() {
            // exhausts the iterator
        }

        num_grains += 1;

        if grain_moves.pos == cave.sand_emitter {
            break;
        }

        cave.tiles.insert(grain_moves.pos, Block::Sand);
    }

    num_grains
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_example() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(part1(input), 24);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 913);
    }

    #[test]
    fn test_part2_example() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(part2(input), 93);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 30762);
    }
}
