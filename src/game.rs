use std::io;
use std::io::Write;
use crate::card::Card;
use crate::game::GameAction::{Hit, Stand};
use crate::game_settings::GameSettings;
use crate::hand::Hand;
use crate::player::Player;
use crate::shoe::Shoe;

pub struct Game {
    pub settings: GameSettings,
    pub shoe: Shoe,
    pub player: Player,
    pub dealer: Player,
    pub state: GameState,
}

impl Game {
    pub fn new(settings: GameSettings) -> Game {
        Game {
            player: Player::new(),
            dealer: Player::new(),
            shoe: Shoe::new(settings.deck_count as usize),
            settings,
            state: GameState::WaitingToDeal
        }
    }

    pub fn get_state(&self) -> &GameState {
        &self.state
    }

    // shuffle function
    pub fn shuffle_shoe(&mut self) {
        self.shoe.shuffle();
    }

    pub fn deal_initial_cards(&mut self) {
        // Deal two cards to player and dealer
        for _ in 0..2 {
            if let Some(card) = self.shoe.draw_card() {
                self.player.add_card_to_hand(card, 0);
            }
            if let Some(card) = self.shoe.draw_card() {
                self.dealer.add_card_to_hand(card, 0);
            }
        }

        if self.player.hands[0].is_natural_blackjack() {
            if self.dealer.hands[0].is_natural_blackjack() {
                self.state = GameState::RoundComplete {
                    dealer_hand: self.dealer.hands[0].clone(),
                    player_hand: self.player.hands[0].clone(),
                    outcome: RoundOutcome::Push,
                };
                return;
            } else {
                self.state = GameState::RoundComplete {
                    dealer_hand: self.dealer.hands[0].clone(),
                    player_hand: self.player.hands[0].clone(),
                    outcome: RoundOutcome::PlayerWin
                };
                return;
            }
        }

        if self.dealer.hands[0].is_natural_blackjack() {
            self.state = GameState::RoundComplete {
                dealer_hand: self.dealer.hands[0].clone(),
                player_hand: self.player.hands[0].clone(),
                outcome: RoundOutcome::DealerWin
            };
            return;
        }

        self.state = GameState::WaitingForPlayer {
            dealer_up_card: self.dealer.hands[0].cards[0].clone(),
            player_hand: self.player.hands[0].clone()
        }
    }

    pub fn process_player_action(&mut self, action: GameAction) {
        match action {
            Hit => {
                if let Some(card) = self.shoe.draw_card() {
                    self.player.add_card_to_hand(card, 0);
                    if self.player.hands[0].is_busted() {
                        self.state = GameState::RoundComplete {
                            dealer_hand: self.dealer.hands[0].clone(),
                            player_hand: self.player.hands[0].clone(),
                            outcome: RoundOutcome::DealerWin,
                        };
                        return;
                    }

                    if self.player.hands[0].is_blackjack() {
                        self.state = GameState::DealerTurn{
                            dealer_hand: self.dealer.hands[0].clone(),
                            player_hand: self.player.hands[0].clone(),
                        };
                        return;
                    }

                    self.state = GameState::WaitingForPlayer {
                        dealer_up_card: self.dealer.hands[0].cards[0].clone(),
                        player_hand: self.player.hands[0].clone()
                    }
                }
            },
            Stand => {
                self.state = GameState::DealerTurn {
                    dealer_hand: self.dealer.hands[0].clone(),
                    player_hand: self.player.hands[0].clone(),
                }
            }
        }
    }

    pub fn next_dealer_turn(&mut self) {
        match self.state {
            GameState::DealerTurn { dealer_hand: _, player_hand: _ } => {
                let dealer_value = self.dealer.hands[0].best_value();

                // Dealer must hit on 16 or below
                if dealer_value <= 16 {
                    if let Some(card) = self.shoe.draw_card() {
                        self.dealer.add_card_to_hand(card, 0);

                        // Check if dealer busted
                        if self.dealer.hands[0].is_busted() {
                            self.state = GameState::RoundComplete {
                                dealer_hand: self.dealer.hands[0].clone(),
                                player_hand: self.player.hands[0].clone(),
                                outcome: RoundOutcome::PlayerWin,
                            };
                            return;
                        }

                        // Continue dealer's turn
                        self.state = GameState::DealerTurn {
                            dealer_hand: self.dealer.hands[0].clone(),
                            player_hand: self.player.hands[0].clone(),
                        };
                    }
                } else {
                    // Dealer stands, determine winner
                    let player_value = self.player.hands[0].best_value();
                    let dealer_value = self.dealer.hands[0].best_value();

                    let outcome = if self.player.hands[0].is_busted() {
                        RoundOutcome::DealerWin
                    } else if dealer_value > player_value {
                        RoundOutcome::DealerWin
                    } else if player_value > dealer_value {
                        RoundOutcome::PlayerWin
                    } else {
                        RoundOutcome::Push
                    };

                    self.state = GameState::RoundComplete {
                        dealer_hand: self.dealer.hands[0].clone(),
                        player_hand: self.player.hands[0].clone(),
                        outcome,
                    };
                }
            },
            _ => {
                // Do nothing if it's not dealer's turn
            }
        }
    }

    pub fn next_round(&mut self) {
        self.player.reset_hands();
        self.dealer.reset_hands();
        self.state = GameState::WaitingToDeal
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

pub enum GameState {
    WaitingToDeal,
    WaitingForPlayer {
        dealer_up_card: Card,
        player_hand: Hand,
    },
    DealerTurn {
        dealer_hand: Hand,
        player_hand: Hand,
    },
    RoundComplete {
        dealer_hand: Hand,
        player_hand: Hand,
        outcome: RoundOutcome,
    }
}

pub enum RoundOutcome {
    PlayerWin,
    DealerWin,
    Push,
}