use std::{collections::{HashSet, VecDeque}, str::FromStr};

type Card = u8;
// type Deck = VecDeque<Card>;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
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
// TODO: this might be slower than just strings for instance
type GameHistory = HashSet<(Deck, Deck)>;

fn game(mut deck1:Deck, mut deck2:Deck) -> (usize, Deck) {
    let mut history = GameHistory::new();
    
    loop {
        // if decks had the same state in other rounds, P1 instantly wins
        // TODO: just insert instead of contains & insert on each loop
        // TODO: get rid of clone
        if history.contains(&(deck1.clone(), deck2.clone())) {
            return (1, deck1);
        }
        // TODO: get rid of clone
        history.insert((deck1.clone(), deck2.clone()));

        if deck1.cards.is_empty() {
            return (2, deck2);
        }
        if deck2.cards.is_empty() {
            return (1, deck1);
        }

        let p1_card = deck1.cards.pop_front().unwrap();
        let p2_card = deck2.cards.pop_front().unwrap();
        if p1_card as usize <= deck1.cards.len() && p2_card as usize <= deck2.cards.len() {
            // recursive combat game initiated here
            let new_deck1: VecDeque<Card> = deck1.cards.iter().take(p1_card as usize).cloned().collect();
            let new_deck2: VecDeque<Card> = deck2.cards.iter().take(p2_card as usize).cloned().collect();

            assert_eq!(new_deck1.len(), p1_card as usize);
            assert_eq!(new_deck2.len(), p2_card as usize);
            
            let (winner, _) = game(Deck {cards: new_deck1}, Deck {cards: new_deck2});
            if winner == 1 {
                deck1.cards.push_back(p1_card);
                deck1.cards.push_back(p2_card);
            } else {
                deck2.cards.push_back(p2_card);
                deck2.cards.push_back(p1_card);
            }
        } else {
            if p1_card > p2_card {
                deck1.cards.push_back(p1_card);
                deck1.cards.push_back(p2_card);
            } else {
                deck2.cards.push_back(p2_card);
                deck2.cards.push_back(p1_card);
            }
        }
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

pub fn part2(input: &str) -> u32 {
    let (p1, p2) = input.split_once("\n\n").unwrap();
    let p1q: Deck = p1.parse().unwrap();
    let p2q: Deck = p2.parse().unwrap();

    let (_, winning_deck) = game(p1q, p2q);

    winning_deck.cards
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, n)| (i as u32 + 1) * n as u32)
        .sum()
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
    #[test]
    fn test_hash_set_tuple_eq () {
        let deck1 = Deck::from_str("Player1:\n1").unwrap();
        let deck2 = Deck::from_str("Player2:\n5").unwrap();

        let mut history = GameHistory::new();
        history.insert((deck1.clone(), deck2.clone()));
        assert!(history.contains(&(deck1.clone(), deck2.clone())));
    }

    #[test]
    fn test_simpler_input() {
        let input = include_str!("../example.txt");
        assert_eq!(part2(input), 291);
    }
}
