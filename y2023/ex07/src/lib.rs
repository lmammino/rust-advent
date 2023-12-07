use std::collections::BinaryHeap;

use models_p2::HandWithBetP2;
mod models;
mod models_p2;

pub fn part1(input: &str) -> u64 {
    let mut hands_heap = BinaryHeap::new();
    for line in input.lines() {
        let (_, hand_with_bet) = models::parse_hand_with_bet(line).unwrap();
        hands_heap.push(hand_with_bet); // reverse makes it a min heap
    }

    hands_heap
        .into_sorted_vec()
        .iter()
        .enumerate()
        .map(|(i, hand_with_bet)| (i + 1) as u64 * hand_with_bet.bet)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let mut hands_heap: BinaryHeap<HandWithBetP2> = BinaryHeap::new();
    for line in input.lines() {
        let (_, hand_with_bet) = models::parse_hand_with_bet(line).unwrap();
        hands_heap.push(hand_with_bet.into()); // reverse makes it a min heap
    }

    hands_heap
        .into_sorted_vec()
        .iter()
        .enumerate()
        .map(|(i, hand_with_bet)| (i + 1) as u64 * hand_with_bet.bet)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");
    const EXAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 6440);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 248217452);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 5905);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 245576185);
    }
}
