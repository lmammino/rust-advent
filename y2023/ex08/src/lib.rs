use models::parse_input;
use num::integer::lcm;

mod models;

pub fn part1(input: &str) -> usize {
    let (_, (directions, paths)) = parse_input(input).unwrap();
    let mut current = "AAA";
    let target = "ZZZ";
    let mut steps = 0;

    while current != target {
        let next = paths
            .next(current, directions[steps % directions.len()])
            .unwrap();
        current = next;
        steps += 1;
    }

    steps
}

pub fn part2(input: &str) -> usize {
    let (_, (directions, paths)) = parse_input(input).unwrap();
    let current_nodes = paths
        .keys()
        .copied()
        .filter(|k| k.ends_with('A'))
        .collect::<Vec<_>>();

    // HACK: looks like for every node there's a loop that can be easily found
    // by getting the index of the first node that ends with 'Z'
    let first_z_index: Vec<usize> = current_nodes
        .iter()
        .map(|start_node| {
            let mut steps = 0;
            let mut current_node = *start_node;
            while !current_node.ends_with('Z') {
                let next = paths
                    .next(current_node, directions[steps % directions.len()])
                    .unwrap();
                current_node = next;
                steps += 1;
            }

            steps
        })
        .collect();

    println!("{:?}", first_z_index);

    // calculates the LCM (lowest common multiplier) of all the first Z indices
    first_z_index.iter().fold(1, |acc, x| lcm(acc, *x))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");
    const EXAMPLE_INPUT_PART1: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    const EXAMPLE_INPUT_PART2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT_PART1), 6);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 19631);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT_PART2), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 21003205388413);
    }
}
