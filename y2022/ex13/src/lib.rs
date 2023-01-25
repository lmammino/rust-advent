mod packet;
use std::cmp::Ordering;

use packet::*;

pub fn part1(input: &str) -> usize {
    parse_input_in_pairs(input)
        .enumerate()
        .filter(|(_, (left, right))| {
            let cmp = left.cmp(right);
            cmp == Ordering::Less
        })
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let (_, mut packets) = parse_input_as_list(input).unwrap();
    let divider_packet_1: Packet = "[[2]]".parse().unwrap();
    let divider_packet_2: Packet = "[[6]]".parse().unwrap();
    packets.push(divider_packet_1.clone());
    packets.push(divider_packet_2.clone());

    packets.sort();
    packets
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if *p == divider_packet_1 || *p == divider_packet_2 {
                Some(i + 1)
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 6420);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 22000);
    }
}
