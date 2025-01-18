use crate::card::Card;
use crate::game::GameAction::{Double, Hit, Split, Stand};
use crate::game::GameState::WaitingToDeal;
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
            state: GameState::WaitingForBet
        }
    }

    pub fn get_state(&self) -> &GameState {
        &self.state
    }

    // shuffle function
    pub fn shuffle_shoe(&mut self) {
        self.shoe.shuffle();
    }

    pub fn accept_user_bet(&mut self, bet: f64) {
        if self.player.bank_roll < bet {
            println!("You cannot bet more than you have");
            return;
        }
        self.player.bank_roll -= bet;
        self.player.hands[0].bet = bet;
        self.state = WaitingToDeal { player_bet: bet, player_bankroll: self.player.bank_roll }
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
                // push, add the bet back to player bankroll
                self.player.bank_roll += self.player.hands[0].bet;
                self.state = GameState::RoundComplete {
                    dealer_hand: self.dealer.hands[0].clone(),
                    player_hand: self.player.hands[0].clone(),
                    outcome: RoundOutcome::Push,
                    player_bankroll: self.player.bank_roll
                };
                return;
            } else {
                self.player.bank_roll += self.player.hands[0].bet * 2.5;
                self.state = GameState::RoundComplete {
                    dealer_hand: self.dealer.hands[0].clone(),
                    player_hand: self.player.hands[0].clone(),
                    outcome: RoundOutcome::PlayerWin,
                    player_bankroll: self.player.bank_roll
                };
                return;
            }
        }

        if self.dealer.hands[0].is_natural_blackjack() {
            self.state = GameState::RoundComplete {
                dealer_hand: self.dealer.hands[0].clone(),
                player_hand: self.player.hands[0].clone(),
                outcome: RoundOutcome::DealerWin,
                player_bankroll: self.player.bank_roll
            };
            return;
        }

        self.state = GameState::WaitingForPlayer {
            dealer_hand: self.dealer.hands[0].clone(),
            player_hand: self.player.hands[0].clone(),
            player_bankroll: self.player.bank_roll,
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
                            player_bankroll: self.player.bank_roll
                        };
                        return;
                    }

                    if self.player.hands[0].is_blackjack() {
                        self.state = GameState::DealerTurn{
                            dealer_hand: self.dealer.hands[0].clone(),
                            player_hand: self.player.hands[0].clone(),
                            player_bankroll: self.player.bank_roll
                        };
                        return;
                    }

                    self.state = GameState::WaitingForPlayer {
                        dealer_hand: self.dealer.hands[0].clone(),
                        player_hand: self.player.hands[0].clone(),
                        player_bankroll: self.player.bank_roll,
                    }
                }
            },
            Stand => {
                self.state = GameState::DealerTurn {
                    dealer_hand: self.dealer.hands[0].clone(),
                    player_hand: self.player.hands[0].clone(),
                    player_bankroll: self.player.bank_roll
                }
            }
            Double => {
                if let Some(card) = self.shoe.draw_card() {
                    self.player.add_card_to_hand(card, 0);
                    self.player.bank_roll -= self.player.hands[0].bet;
                    self.player.hands[0].bet = self.player.hands[0].bet * 2f64;
                    self.state = GameState::DealerTurn {
                        dealer_hand: self.dealer.hands[0].clone(),
                        player_hand: self.player.hands[0].clone(),
                        player_bankroll: self.player.bank_roll
                    }
                }
            },
            Split => {}
        }
    }

    pub fn next_dealer_turn(&mut self) {
        match self.state {
            GameState::DealerTurn { dealer_hand: _, player_hand: _, player_bankroll: _ } => {
                let dealer_value = self.dealer.hands[0].best_value();

                // Dealer must hit on 16 or below
                if dealer_value <= 16 {
                    if let Some(card) = self.shoe.draw_card() {
                        self.dealer.add_card_to_hand(card, 0);

                        // Check if dealer busted
                        if self.dealer.hands[0].is_busted() {
                            self.player.bank_roll += self.player.hands[0].bet * 2f64;
                            self.state = GameState::RoundComplete {
                                dealer_hand: self.dealer.hands[0].clone(),
                                player_hand: self.player.hands[0].clone(),
                                outcome: RoundOutcome::PlayerWin,
                                player_bankroll: self.player.bank_roll
                            };
                            return;
                        }

                        // Continue dealer's turn
                        self.state = GameState::DealerTurn {
                            dealer_hand: self.dealer.hands[0].clone(),
                            player_hand: self.player.hands[0].clone(),
                            player_bankroll: self.player.bank_roll
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
                        self.player.bank_roll += self.player.hands[0].bet * 2f64;
                        RoundOutcome::PlayerWin
                    } else {
                        self.player.bank_roll += self.player.hands[0].bet;
                        RoundOutcome::Push
                    };

                    self.state = GameState::RoundComplete {
                        dealer_hand: self.dealer.hands[0].clone(),
                        player_hand: self.player.hands[0].clone(),
                        outcome,
                        player_bankroll: self.player.bank_roll
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
        self.state = GameState::WaitingForBet
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
    Hit, Stand, Double, Split
}

impl GameAction {
    pub fn from_string(value: &str) -> Option<GameAction> {
        match value.to_lowercase().trim() {
            "h" | "hit" => Some(Hit),
            "s" | "stand" => Some(Stand),
            "d" | "double" => Some(Double),
            "p" | "split" => Some(Split),
            _ => None
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Hit => "HIT".to_string(),
            Stand => "STAND".to_string(),
            Double => "DOUBLE".to_string(),
            Split => "SPLIT".to_string(),
        }
    }
}

pub enum HandResult {
    Blackjack, Bust, Other(Vec<i32>)
}

pub enum GameState {
    WaitingForBet,
    WaitingToDeal {
        player_bet: f64,
        player_bankroll: f64,
    },
    WaitingForPlayer {
        dealer_hand: Hand,
        player_hand: Hand,
        player_bankroll: f64,
    },
    DealerTurn {
        dealer_hand: Hand,
        player_hand: Hand,
        player_bankroll: f64,
    },
    RoundComplete {
        dealer_hand: Hand,
        player_hand: Hand,
        outcome: RoundOutcome,
        player_bankroll: f64,
    }
}

pub enum RoundOutcome {
    PlayerWin,
    DealerWin,
    Push,
}