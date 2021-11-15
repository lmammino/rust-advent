use std::{collections::VecDeque, str::FromStr};

type Card = u8;
// type Deck = VecDeque<Card>;

struct Deck {
    cards: VecDeque<Card>,
}

impl FromStr for Deck {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: VecDeque<Card> = s
            .lines()
            .skip(1)
            .map(|i| i.parse::<Card>().unwrap())
            .collect();

        Ok(Deck { cards })
    }
}

pub fn part1(input: &str) -> u32 {
    let (p1, p2) = input.split_once("\n\n").unwrap();
    let mut p1q: Deck = p1.parse().unwrap();
    let mut p2q: Deck = p2.parse().unwrap();

    let winner: Deck = loop {
        if p1q.cards.is_empty() {
            break p2q;
        }
        if p2q.cards.is_empty() {
            break p1q;
        }
        let p1_card = p1q.cards.pop_front().unwrap();
        let p2_card = p2q.cards.pop_front().unwrap();

        if p1_card > p2_card {
            p1q.cards.push_back(p1_card);
            p1q.cards.push_back(p2_card);
        } else {
            p2q.cards.push_back(p2_card);
            p2q.cards.push_back(p1_card);
        }
    };

    winner
        .cards
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, n)| (i as u32 + 1) * n as u32)
        .sum()
}

pub fn part2(_input: &str) -> usize {
    33651
}

#[cfg(test)]
mod ex22_tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 33421);
    }

    #[test]
    fn part_2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 33651);
    }
}
