use models::Map;

mod models;

pub fn part1(input: &str) -> usize {
    let mut map: Map = input.parse().unwrap();
    map.tilt_north();
    map.total_load()
}

pub fn part2(input: &str) -> usize {
    let mut map: Map = input.parse().unwrap();
    // from observing the data it seems that after a few cycles (depending on the input)
    // the total load stabilises and keeps rotating between a set of fixed values.
    //
    // idea:
    // - we do 100 cycles to let the system stabilise
    // - we take the load value (starting load value)
    // - we collect a repeating sequence of load values until we find a duplicate of the starting load value
    // - we take the difference between the two indices and we have the cycle length
    // - we take the remainder of (1_000_000_000 - 100) / cycle_length
    // - we take the load value from the repeating sequence at that index
    for _ in 0..100 {
        map.cycle();
    }
    let starting_load = map.total_load();
    let mut repeating_loads = vec![starting_load];
    loop {
        map.cycle();
        let new_load = map.total_load();
        if new_load == starting_load {
            break;
        } else {
            repeating_loads.push(new_load);
        }
    }
    let cycle_length = repeating_loads.len();
    let remainder = (1_000_000_000 - 100) % cycle_length;
    repeating_loads[remainder]
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");
    const EXAMPLE_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 109661);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 64);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 90176);
    }
}
