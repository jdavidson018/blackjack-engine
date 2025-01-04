use crate::card::Card;
use crate::hand::Hand;

pub struct Player {
    pub hands: Vec<Hand>,
    pub bank_roll: i32
    // I'll eventually want to track previous_hands, but not necessary yet
    // pub previous_hands: Vec<Hand>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            hands: vec![Hand::new()],
            bank_roll: 10_000
        }
    }

    pub fn add_card_to_hand(&mut self, card: Card, hand_index: usize) {
        if let Some(hand) = self.hands.get_mut(hand_index) {
            hand.add_card(card);
        }
    }

    // Will be implemented later on
    // pub fn double_down(&mut self, hand_index: usize) -> Option<()> {
    //     if let Some(hand) = self.hands.get_mut(hand_index) {
    //         // Deduct money from bankroll first
    //         let additional_bet = hand.bet;
    //         if self.bank_roll >= additional_bet as i32 {
    //             self.bank_roll -= additional_bet as i32;
    //             hand.double_down().ok()?;
    //             Some(())
    //         } else {
    //             None
    //         }
    //     } else {
    //         None
    //     }
    // }

    // Will complete implementation later on
    // pub fn split_hand(&mut self, hand_index: usize) -> Option<()> {
    //     if let Some(hand) = self.hands.get_mut(hand_index) {
    //         if hand.cards.len() != 2 {
    //             return None;
    //         }
    //
    //         // Check if cards have same rank
    //         if hand.cards[0].rank != hand.cards[1].rank {
    //             return None;
    //         }
    //
    //         let split_card = hand.cards.pop()?;
    //         let mut new_hand = Hand::new();
    //         new_hand.bet = hand.bet;
    //         new_hand.add_card_to_hand(split_card);
    //
    //         // Deduct additional bet from bankroll
    //         self.bank_roll -= hand.bet as i32;
    //         self.hands.push(new_hand);
    //         Some(())
    //     } else {
    //         None
    //     }
    // }

    pub fn print_active_hand(&self) {
        for (i, hand) in self.hands.iter().enumerate() {
            print!("Hand {}: ", i + 1);
            for card in hand.cards.iter() {
                print!("{} ", card.to_string());
            }
            println!("\n");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Rank, Suit};

    #[test]
    fn test_new_player() {
        let player = Player::new();
        assert_eq!(player.hands.len(), 1);
        assert_eq!(player.bank_roll, 10_000);
        assert_eq!(player.hands[0].cards.len(), 0);
    }

    #[test]
    fn test_add_card_to_hand() {
        let mut player = Player::new();
        let card = Card::new(Rank::Ace, Suit::Spades);
        player.add_card_to_hand(card, 0);

        assert_eq!(player.hands[0].cards.len(), 1);
        assert_eq!(player.hands[0].cards[0].rank, Rank::Ace);
        assert_eq!(player.hands[0].cards[0].suit, Suit::Spades);
    }

    #[test]
    fn test_add_card_to_invalid_hand() {
        let mut player = Player::new();
        let card = Card::new(Rank::Ace, Suit::Spades);
        player.add_card_to_hand(card, 999); // Invalid index

        assert_eq!(player.hands[0].cards.len(), 0); // Should not add card
    }

    #[test]
    fn test_add_multiple_cards() {
        let mut player = Player::new();
        let card1 = Card::new(Rank::Ace, Suit::Spades);
        let card2 = Card::new(Rank::King, Suit::Hearts);

        player.add_card_to_hand(card1, 0);
        player.add_card_to_hand(card2, 0);

        assert_eq!(player.hands[0].cards.len(), 2);
    }
}