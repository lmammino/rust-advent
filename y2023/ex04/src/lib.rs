use nom::{
    bytes::complete::tag,
    character::complete::{space0, space1, u32},
    combinator::complete,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, PartialEq, Eq)]
struct Card {
    id: u32,
    winning: HashSet<u32>,
    own: HashSet<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        let count_matching = self.count_matching();
        if count_matching > 0 {
            2u32.pow(count_matching as u32 - 1)
        } else {
            0
        }
    }

    fn count_matching(&self) -> usize {
        self.winning.intersection(&self.own).count()
    }
}

fn parse_card_id(input: &str) -> IResult<&str, u32> {
    let (input, _) = tuple((tag("Card"), space1))(input)?;
    let (input, id) = u32(input)?;

    Ok((input, id))
}

fn parse_numbers(input: &str) -> IResult<&str, HashSet<u32>> {
    let (input, a) = separated_list1(space1, u32)(input)?;
    Ok((input, a.into_iter().collect::<HashSet<u32>>()))
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, id) = parse_card_id(input)?;
    let (input, _) = tuple((tag(": "), space0))(input)?;
    let (input, winning) = parse_numbers(input)?;
    let (input, _) = tuple((tag(" | "), space0))(input)?;
    let (input, own) = complete(parse_numbers)(input)?;

    Ok((input, Card { id, winning, own }))
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, card) = parse_card(line).unwrap();
            card.score()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let cards = input
        .lines()
        .map(|line| {
            let (_, card) = parse_card(line).unwrap();
            card
        })
        .collect::<Vec<_>>();

    let mut num_processed_cards = cards.len();
    let mut copies_count = HashMap::new();

    for card in cards.iter() {
        let curr_card_idx = card.id as usize - 1;
        for i in 1..=card.count_matching() {
            let copied_card_idx = curr_card_idx + i;
            let entry: &mut usize = copies_count.entry(copied_card_idx).or_default();
            *entry += 1;
        }
    }

    while !copies_count.is_empty() {
        let mut new_copies_count = HashMap::new();

        for (card_idx, num_copies) in copies_count.drain() {
            num_processed_cards += num_copies;

            let card = &cards[card_idx];
            let curr_card_idx = card.id as usize - 1;
            for i in 1..=card.count_matching() {
                let copied_card_idx = curr_card_idx + i;
                let entry: &mut usize = new_copies_count.entry(copied_card_idx).or_default();
                *entry += num_copies;
            }
        }

        copies_count = new_copies_count;
    }

    num_processed_cards
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_parse_card() {
        let input = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        let (_, card) = parse_card(input).unwrap();
        assert_eq!(
            card,
            Card {
                id: 4,
                winning: vec![41, 92, 73, 84, 69].into_iter().collect(),
                own: vec![59, 84, 76, 51, 58, 5, 54, 83].into_iter().collect(),
            }
        );
    }

    #[test]
    fn test_card_scores() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let scores = input
            .lines()
            .map(|line| {
                let (_, card) = parse_card(line).unwrap();
                card.score()
            })
            .collect::<Vec<_>>();
        assert_eq!(scores, vec![8, 2, 2, 1, 0, 0]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 23673);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 12263631);
    }
}
