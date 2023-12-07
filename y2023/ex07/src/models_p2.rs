use crate::models::{Card, Hand, HandRank, HandWithBet};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardP2(Card);

impl Deref for CardP2 {
    type Target = Card;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialOrd for CardP2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CardP2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_value = match &self.0 {
            Card::Jack => 1,
            _ => self.0.clone() as u8,
        };
        let other_value = match &other.0 {
            Card::Jack => 1,
            _ => other.0.clone() as u8,
        };

        self_value.cmp(&other_value)
    }
}

impl From<Card> for CardP2 {
    fn from(card: Card) -> Self {
        Self(card)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HandP2 {
    cards: [CardP2; 5],
    original: Hand,
}

impl From<Hand> for HandP2 {
    fn from(hand: Hand) -> Self {
        let cards: [CardP2; 5] = hand
            .cards
            .iter()
            .map(|c| c.clone().into())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self {
            cards,
            original: hand,
        }
    }
}

impl HandP2 {
    pub fn rank(&self) -> u8 {
        let num_jokers = self.original.counts.get(&Card::Jack).unwrap_or(&0);
        let mut counts_without_jokers = self.original.counts.clone();
        counts_without_jokers.remove(&Card::Jack);

        match num_jokers {
            0 => self.original.rank(),
            1 => {
                // J ? ? ? ?

                // 1 joker can be turned into a five of a kind if the hand has a four of a kind
                // (X) X X X X
                if counts_without_jokers.values().any(|&v| v == 4) {
                    return HandRank::FiveOfAKind as u8;
                }
                // 1 joker can be turned into a four of a kind if the hand has a three of a kind
                // (X) X X X Y
                if counts_without_jokers.values().any(|&v| v == 3) {
                    return HandRank::FourOfAKind as u8;
                }
                // 1 joker can be turned into a full house if the hand has 2 couples
                // (X) X X Y Y
                if counts_without_jokers.values().all(|&v| v == 2) {
                    return HandRank::FullHouse as u8;
                }
                // 1 joker can be turned into a three of a kind if the hand has 1 couple
                // (X) X X Y Z
                if counts_without_jokers.values().any(|&v| v == 2) {
                    return HandRank::ThreeOfAKind as u8;
                }

                // otherwise the best we can do is a couple
                // (X) X Y Z W
                HandRank::OnePair as u8
            }
            2 => {
                // J J ? ? ?

                // 2 jokers can be turned into a five of a kind if the hand has a three of a kind
                // (X X) X X X
                if counts_without_jokers.values().any(|&v| v == 3) {
                    return HandRank::FiveOfAKind as u8;
                }
                // otherwise you can turn it into a four of a kind if the hand has a couple
                // (X X) X X Y
                if counts_without_jokers.values().any(|&v| v == 2) {
                    return HandRank::FourOfAKind as u8;
                }
                // otherwise you can turn it into a three of a kind
                // (X X) X Y Z
                HandRank::ThreeOfAKind as u8
            }
            3 => {
                // J J J ? ?

                // 3 jokers can be turned into a five of a kind if the hand has a couple
                // (X X X) X X
                if counts_without_jokers.values().any(|&v| v == 2) {
                    return HandRank::FiveOfAKind as u8;
                }
                // otherwise you can turn it into a four of a kind
                // (X X X) X Y
                HandRank::FourOfAKind as u8
            }
            4 => {
                // J J J J ?

                // 4 jokers can be turned into a five of a kind
                // (X X X X) X
                HandRank::FiveOfAKind as u8
            }
            5 => {
                // 5 jokers is a five of a kind already
                HandRank::FiveOfAKind as u8
            }
            _ => unreachable!("cannot have more than 5 jokers"),
        }
    }
}

impl PartialOrd for HandP2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandP2 {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandWithBetP2 {
    hand: HandP2,
    pub bet: u64,
}

impl From<HandWithBet> for HandWithBetP2 {
    fn from(hand_with_bet: HandWithBet) -> Self {
        Self {
            hand: hand_with_bet.hand.into(),
            bet: hand_with_bet.bet,
        }
    }
}

impl PartialOrd for HandWithBetP2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandWithBetP2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}
