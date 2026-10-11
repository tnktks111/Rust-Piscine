use rand::RngExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Heart,
    Diamonds,
    Spade,
    Club,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Ace,
    Jack,
    Queen,
    King,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::rng();

        let value = rng.random_range(1..=4);

        Self::translate(value)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamonds,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid suit value: {}", value),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::rng();

        let value = rng.random_range(1..=13);

        Self::translate(value)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Invalid rank value: {}", value),
        }
    }
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}
