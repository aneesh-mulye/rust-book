// Challenge 2.2 - Card Hand Scorer
//
// Define `Card` with variants:
// - Number(u8)
// - Jack
// - Queen
// - King
// - Ace
//
// Implement:
// - `Card::value(&self) -> u8`
// - `hand_value(hand: &[Card]) -> u8`
// - `is_bust(hand: &[Card]) -> bool`

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Card {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    pub fn value(&self) -> u8 {
        match self {
            Card::Number(v) => *v,
            Card::Ace => 11,
            _ => 10,
        }
    }
}

pub fn hand_value(hand: &[Card]) -> u8 {
    hand.iter().map(Card::value).sum()
}

pub fn is_bust(hand: &[Card]) -> bool {
    hand_value(hand) > 21
}

// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .
// .

#[cfg(test)]
mod tests {
    use super::{Card, hand_value, is_bust};

    #[test]
    fn per_card_values_match_rules() {
        assert_eq!(
            Card::Number(7).value(),
            7,
            "Number card should return its inner value."
        );
        assert_eq!(Card::Jack.value(), 10, "Jack should be worth 10.");
        assert_eq!(Card::Queen.value(), 10, "Queen should be worth 10.");
        assert_eq!(Card::King.value(), 10, "King should be worth 10.");
        assert_eq!(Card::Ace.value(), 11, "Ace should be worth 11.");
    }

    #[test]
    fn prompt_hands_score_correctly() {
        let hand_a = [Card::Number(10), Card::Ace];
        let hand_b = [Card::King, Card::Number(7), Card::Number(8)];

        assert_eq!(hand_value(&hand_a), 21, "Hand A should total 21.");
        assert!(!is_bust(&hand_a), "Hand A should not be bust.");
        assert_eq!(hand_value(&hand_b), 25, "Hand B should total 25.");
        assert!(is_bust(&hand_b), "Hand B should be bust.");
    }
}
