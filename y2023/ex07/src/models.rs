use std::{collections::HashMap, fmt::Display};

use nom::{
    bytes::complete::tag,
    character::complete::{one_of, u64},
    combinator::complete,
    multi::count,
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandRank {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPairs = 3,
    OnePair = 2,
    HighCard = 1,
}

// A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("Invalid card: {}", c),
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Ace => 'A',
            Self::King => 'K',
            Self::Queen => 'Q',
            Self::Jack => 'J',
            Self::Ten => 'T',
            Self::Nine => '9',
            Self::Eight => '8',
            Self::Seven => '7',
            Self::Six => '6',
            Self::Five => '5',
            Self::Four => '4',
            Self::Three => '3',
            Self::Two => '2',
        };
        write!(f, "{}", c)
    }
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, c) = one_of("AKQJT98765432")(input)?;
    Ok((input, c.into()))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand {
    pub cards: [Card; 5],
    pub counts: HashMap<Card, usize>,
}

impl Hand {
    pub fn new(cards: [Card; 5]) -> Self {
        let mut counts = HashMap::new();
        for card in &cards {
            *counts.entry(card.clone()).or_insert(0) += 1;
        }

        Self { cards, counts }
    }

    pub fn is_five_of_a_kind(&self) -> bool {
        for i in 0..4 {
            if self.cards[i] != self.cards[i + 1] {
                return false;
            }
        }
        true
    }

    pub fn is_four_of_a_kind(&self) -> bool {
        for count in self.counts.values() {
            if *count == 4 {
                return true;
            }
        }
        false
    }

    pub fn is_full_house(&self) -> bool {
        let mut three_count = 0;
        let mut two_count = 0;
        for count in self.counts.values() {
            if *count == 3 {
                three_count += 1;
            } else if *count == 2 {
                two_count += 1;
            }
        }
        three_count == 1 && two_count == 1
    }

    pub fn is_three_of_a_kind(&self) -> bool {
        let mut three_count = 0;
        let mut single_count = 0;
        for count in self.counts.values() {
            if *count == 3 {
                three_count += 1;
            } else if *count == 1 {
                single_count += 1;
            }
        }
        three_count == 1 && single_count == 2
    }

    pub fn is_two_pairs(&self) -> bool {
        let mut pair_count = 0;
        let mut single_count = 0;
        for count in self.counts.values() {
            if *count == 2 {
                pair_count += 1;
            } else if *count == 1 {
                single_count += 1;
            }
        }
        pair_count == 2 && single_count == 1
    }

    pub fn is_one_pair(&self) -> bool {
        let mut pair_count = 0;
        let mut single_count = 0;
        for count in self.counts.values() {
            if *count == 2 {
                pair_count += 1;
            } else if *count == 1 {
                single_count += 1;
            }
        }
        pair_count == 1 && single_count == 3
    }

    pub fn is_high_card(&self) -> bool {
        for count in self.counts.values() {
            if *count != 1 {
                return false;
            }
        }
        true
    }

    pub fn rank(&self) -> u8 {
        if self.is_five_of_a_kind() {
            HandRank::FiveOfAKind as u8
        } else if self.is_four_of_a_kind() {
            HandRank::FourOfAKind as u8
        } else if self.is_full_house() {
            HandRank::FullHouse as u8
        } else if self.is_three_of_a_kind() {
            HandRank::ThreeOfAKind as u8
        } else if self.is_two_pairs() {
            HandRank::TwoPairs as u8
        } else if self.is_one_pair() {
            HandRank::OnePair as u8
        } else if self.is_high_card() {
            HandRank::HighCard as u8
        } else {
            // should never happen
            unreachable!("Invalid hand: {:?}", self);
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in &self.cards {
            write!(f, "{}", card)?;
        }
        Ok(())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_rank = self.rank();
        let other_rank = other.rank();

        if self_rank != other_rank {
            return self_rank.cmp(&other_rank);
        }

        // if they have the same rank then we need to compare the cards
        for i in 0..4 {
            let self_card = &self.cards[i];
            let other_card = &other.cards[i];
            if self_card != other_card {
                return self_card.cmp(other_card);
            }
        }

        // if all is still equal so far return the comparison of the last card in both hands
        self.cards[4].cmp(&other.cards[4])
    }
}

pub fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, cards) = count(parse_card, 5)(input)?;
    Ok((input, Hand::new(cards.try_into().unwrap())))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandWithBet {
    pub hand: Hand,
    pub bet: u64,
}

impl PartialOrd for HandWithBet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandWithBet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}

pub fn parse_hand_with_bet(input: &str) -> IResult<&str, HandWithBet> {
    let (input, hand) = parse_hand(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, bet) = complete(u64)(input)?;
    Ok((input, HandWithBet { hand, bet }))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compare_cards() {
        assert!(Card::Ace > Card::King);
        assert!(Card::King > Card::Queen);
        assert!(Card::Queen > Card::Jack);
        assert!(Card::Jack > Card::Ten);
        assert!(Card::Ten > Card::Nine);
        assert!(Card::Nine > Card::Eight);
        assert!(Card::Eight > Card::Seven);
        assert!(Card::Seven > Card::Six);
        assert!(Card::Six > Card::Five);
        assert!(Card::Five > Card::Four);
        assert!(Card::Four > Card::Three);
        assert!(Card::Three > Card::Two);
    }

    #[test]
    fn test_compare_hands() {
        let mut hands = [
            "JJJJJ", "K2444", "5AAAA", "78Q77", "42447", "33337", "JJTTT", "88388", "78888",
        ]
        .map(|s| parse_hand(s).unwrap().1);

        let expected_sort = [
            "42447", "78Q77", "K2444", "JJTTT", "33337", "5AAAA", "78888", "88388", "JJJJJ",
        ]
        .map(|s| parse_hand(s).unwrap().1);

        hands.sort();
        assert_eq!(hands, expected_sort)
    }

    #[test]
    fn test_parse_hand() {
        let input = "94J8A";
        let expected_counts = [
            (Card::Nine, 1),
            (Card::Four, 1),
            (Card::Jack, 1),
            (Card::Eight, 1),
            (Card::Ace, 1),
        ];
        let expected = Hand {
            cards: [Card::Nine, Card::Four, Card::Jack, Card::Eight, Card::Ace],
            counts: expected_counts.into(),
        };
        let (_, hand) = parse_hand(input).unwrap();
        assert_eq!(hand, expected);
    }

    #[test]
    fn test_parse_hand_with_bet() {
        let input = "JK5KA 722";
        let expected_counts = [
            (Card::Jack, 1),
            (Card::King, 2),
            (Card::Five, 1),
            (Card::Ace, 1),
        ];
        let expected = HandWithBet {
            hand: Hand {
                cards: [Card::Jack, Card::King, Card::Five, Card::King, Card::Ace],
                counts: expected_counts.into(),
            },
            bet: 722,
        };
        let (_, hand_with_bet) = parse_hand_with_bet(input).unwrap();
        assert_eq!(hand_with_bet, expected);
    }

    #[test]
    fn test_hand_rankings() {
        let (_, five_of_a_kind) = parse_hand("AAAAA").unwrap();
        let (_, four_of_a_kind) = parse_hand("AA8AA").unwrap();
        let (_, full_house) = parse_hand("23332").unwrap();
        let (_, three_of_a_kind) = parse_hand("TTT98").unwrap();
        let (_, two_pairs) = parse_hand("23432").unwrap();
        let (_, one_pair) = parse_hand("A23A4").unwrap();
        let (_, high_card) = parse_hand("23456").unwrap();
        assert!(five_of_a_kind.is_five_of_a_kind());
        assert!(four_of_a_kind.is_four_of_a_kind());
        assert!(full_house.is_full_house());
        assert!(three_of_a_kind.is_three_of_a_kind());
        assert!(two_pairs.is_two_pairs());
        assert!(one_pair.is_one_pair());
        assert!(high_card.is_high_card());
    }
}
