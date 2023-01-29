use std::{
    collections::{HashSet, VecDeque},
    ops::{Deref, DerefMut},
    str::FromStr,
};

type Card = usize;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Deck(VecDeque<Card>);

impl Deref for Deck {
    type Target = VecDeque<Card>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Deck {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deck {
    fn calculate_score(self) -> usize {
        self.0
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, n)| (i + 1) * n)
            .sum()
    }

    fn hash(&self) -> usize {
        // The hashing function is similar to the calculate_score one
        // this hash is not perfect, it can lead to some collisions, but it is "good enough" for our tests
        // The speedup compared to use the whole cards VecDeque is huge (90% faster), still using the default rust HashSet
        self.iter().enumerate().map(|(i, n)| (i * 29 + 1) * n).sum()
    }
}

impl FromStr for Deck {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: VecDeque<Card> = s
            .lines()
            .skip(1)
            .map(|i| i.parse::<Card>().unwrap())
            .collect();

        Ok(Deck(cards))
    }
}

type GameHistory = HashSet<(usize, usize)>;

enum Player {
    One,
    Two,
}

fn game(mut deck1: Deck, mut deck2: Deck) -> (Player, Deck) {
    let mut history = GameHistory::new();

    loop {
        // if decks had the same state in other rounds, P1 instantly wins
        if !history.insert((deck1.hash(), deck2.hash())) {
            return (Player::One, deck1);
        }

        if deck1.is_empty() {
            return (Player::Two, deck2);
        }
        if deck2.is_empty() {
            return (Player::One, deck1);
        }

        let p1_card = deck1.pop_front().unwrap();
        let p2_card = deck2.pop_front().unwrap();
        if p1_card <= deck1.len() && p2_card <= deck2.len() {
            // recursive combat game initiated here
            let new_deck1: VecDeque<Card> = deck1.iter().take(p1_card).cloned().collect();
            let new_deck2: VecDeque<Card> = deck2.iter().take(p2_card).cloned().collect();

            assert_eq!(new_deck1.len(), p1_card);
            assert_eq!(new_deck2.len(), p2_card);

            match game(Deck(new_deck1), Deck(new_deck2)) {
                (Player::One, _) => {
                    deck1.push_back(p1_card);
                    deck1.push_back(p2_card);
                }
                (Player::Two, _) => {
                    deck2.push_back(p2_card);
                    deck2.push_back(p1_card);
                }
            }
        } else if p1_card > p2_card {
            deck1.push_back(p1_card);
            deck1.push_back(p2_card);
        } else {
            deck2.push_back(p2_card);
            deck2.push_back(p1_card);
        }
    }
}

pub fn part1(input: &str) -> usize {
    let (p1, p2) = input.split_once("\n\n").unwrap();
    let mut p1q: Deck = p1.parse().unwrap();
    let mut p2q: Deck = p2.parse().unwrap();

    let winner: Deck = loop {
        if p1q.is_empty() {
            break p2q;
        }
        if p2q.is_empty() {
            break p1q;
        }
        let p1_card = p1q.pop_front().unwrap();
        let p2_card = p2q.pop_front().unwrap();

        if p1_card > p2_card {
            p1q.push_back(p1_card);
            p1q.push_back(p2_card);
        } else {
            p2q.push_back(p2_card);
            p2q.push_back(p1_card);
        }
    };

    winner.calculate_score()
}

pub fn part2(input: &str) -> usize {
    let (p1, p2) = input.split_once("\n\n").unwrap();
    let p1q: Deck = p1.parse().unwrap();
    let p2q: Deck = p2.parse().unwrap();

    let (_, winning_deck) = game(p1q, p2q);

    winning_deck.calculate_score()
}

#[cfg(test)]
mod ex22_tests {
    use super::*;
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1() {
        assert_eq!(part1(INPUT), 33421);
    }

    #[test]
    fn part_2() {
        assert_eq!(part2(INPUT), 33651);
    }

    #[test]
    fn test_hash_set_tuple_eq() {
        let deck1 = Deck::from_str("Player1:\n1").unwrap();
        let deck2 = Deck::from_str("Player2:\n5").unwrap();

        let mut history = GameHistory::new();
        history.insert((deck1.hash(), deck2.hash()));
        assert!(history.contains(&(deck1.hash(), deck2.hash())));
    }

    #[test]
    fn test_simpler_input() {
        let input = include_str!("../example.txt");
        assert_eq!(part2(input), 291);
    }
}
