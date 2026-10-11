use card_deck::*;

#[test]
fn test_suit_translate() {
    assert_eq!(Suit::translate(1), Suit::Heart);
    assert_eq!(Suit::translate(2), Suit::Diamonds);
    assert_eq!(Suit::translate(3), Suit::Spade);
    assert_eq!(Suit::translate(4), Suit::Club);
}

#[test]
fn test_rank_translate() {
    assert_eq!(Rank::translate(1), Rank::Ace);

    for i in 2..=10 {
        assert_eq!(Rank::translate(i), Rank::Number(i));
    }

    assert_eq!(Rank::translate(11), Rank::Jack);
    assert_eq!(Rank::translate(12), Rank::Queen);
    assert_eq!(Rank::translate(13), Rank::King);
}

#[test]
fn test_winner_card() {
    let winner = Card {
        suit: Suit::Spade,
        rank: Rank::Ace,
    };

    assert!(winner_card(&winner));

    let loser = Card {
        suit: Suit::Heart,
        rank: Rank::Ace,
    };

    assert!(!winner_card(&loser));
}

#[test]
fn test_random() {
    for _ in 0..100 {
        let suit = Suit::random();
        let rank = Rank::random();

        assert!(matches!(
            suit,
            Suit::Heart | Suit::Diamonds | Suit::Spade | Suit::Club
        ));

        assert!(matches!(
            rank,
            Rank::Ace | Rank::Jack | Rank::Queen | Rank::King | Rank::Number(2..=10)
        ));
    }
}
