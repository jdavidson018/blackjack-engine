use std::io;
use std::io::Write;
use crate::game::GameAction::{Hit, Stand};
use crate::game_settings::GameSettings;
use crate::hand::Hand;
use crate::player::Player;
use crate::shoe::Shoe;

pub struct Game {
    pub settings: GameSettings,
    pub shoe: Shoe,
    pub player: Player,
    pub dealer: Player
}

impl Game {
    pub fn new(settings: GameSettings) -> Game {
        Game {
            player: Player::new(),
            dealer: Player::new(),
            shoe: Shoe::new(settings.deck_count as usize),
            settings
        }
    }

    // shuffle function
    pub fn shuffle_shoe(&mut self) {
        self.shoe.shuffle();
    }

    fn deal_initial_cards(&mut self) {
        // Deal two cards to player and dealer
        for _ in 0..2 {
            if let Some(card) = self.shoe.draw_card() {
                self.player.add_card_to_hand(card, 0);
            }
            if let Some(card) = self.shoe.draw_card() {
                self.dealer.add_card_to_hand(card, 0);
            }
        }
    }

    fn get_player_action(&self) -> GameAction {
        loop {
            print!("Would you like to (H)it or (S)tand? ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            if let Some(action) = GameAction::from_string(&input) {
                return action;
            }
            println!("Invalid input. Please enter 'H' for Hit or 'S' for Stand.");
        }
    }

    pub fn play_round(&mut self) {
        // Shuffle and clean out hands from the last round
        self.shuffle_shoe();
        self.player.reset_hands();
        self.dealer.reset_hands();

        self.deal_initial_cards();

        // Show initial hands
        self.show_hands();

        // Player's turn
        loop {
            let player_hand = &self.player.hands[0];
            if player_hand.is_busted() {
                println!("Bust! You lose!");
                return;
            }

            match self.get_player_action() {
                Hit => {
                    if let Some(card) = self.shoe.draw_card() {
                        println!("You drew: {}", card.to_string());
                        self.player.add_card_to_hand(card, 0);
                    }
                }
                Stand => break,
            }
        }

        // Dealer's turn
        println!("\nDealer's turn:");
        self.show_hands();

        let player_value = self.player.hands[0].best_value();
        if !self.player.hands[0].is_busted() {
            // Dealer hits until they beat the player or bust
            while self.dealer.hands[0].best_value() <= player_value {
                if let Some(card) = self.shoe.draw_card() {
                    println!("Dealer drew: {}", card.to_string());
                    self.dealer.add_card_to_hand(card, 0);
                    self.show_hands();

                    if self.dealer.hands[0].is_busted() {
                        println!("Dealer busts! You win!");
                        return;
                    }
                }
            }
        }

        // Determine winner
        self.determine_winner();
    }

    fn test(player: &Hand) {

    }

    fn determine_winner(&self) {
        let player_hand = &self.player.hands[0];
        let dealer_hand = &self.dealer.hands[0];

        let player_value = player_hand.best_value();
        let dealer_value = dealer_hand.best_value();

        if player_hand.is_busted() {
            println!("You bust! Dealer wins!");
        } else if dealer_hand.is_busted() {
            println!("Dealer busts! You win!");
        } else if dealer_value > player_value {
            println!("Dealer wins with {} vs your {}", dealer_value, player_value);
        } else if player_value > dealer_value {
            println!("You win with {} vs dealer's {}", player_value, dealer_value);
        } else {
            println!("Push! Both have {}", player_value);
        }
    }

    pub fn show_hands(&self) {
        println!("\nDealer's hand:");
        self.dealer.print_active_hand();
        println!("Dealer's total: {}", self.dealer.hands[0].best_value());

        println!("\nYour hand:");
        self.player.print_active_hand();
        println!("Your total: {}", self.player.hands[0].best_value());
        println!();
    }
}

pub enum PlayerType {
    MainPlayer,
    Dealer,
    Other(u32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameAction {
    Hit, Stand
}

impl GameAction {
    pub fn from_string(value: &str) -> Option<GameAction> {
        match value.to_lowercase().trim() {
            "h" | "hit" => Some(Hit),
            "s" | "stand" => Some(Stand),
            _ => None
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Hit => "HIT".to_string(),
            Stand => "STAND".to_string(),
        }
    }
}

pub enum HandResult {
    Blackjack, Bust, Other(Vec<i32>)
}