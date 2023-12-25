pub mod board;
pub mod common;
pub mod components;
pub mod player;

use board::cards_on_board::CardsOnBoard;
use board::deck::Deck;
use board::hand::Hand;
use common::CARDS_IN_HAND;
use components::Player;
use player::Player as PlayerTrait;

struct Game {
    board: CardsOnBoard,
    deck: Deck,
    hand1: Hand,
    hand2: Hand,
}
impl Game {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        let mut hand1 = Hand::new();
        let mut hand2 = Hand::new();

        for _ in 0..CARDS_IN_HAND {
            hand1.add(deck.draw().expect("A deck should have enough cards to initialize the game."));
            hand2.add(deck.draw().unwrap());
        }

        Game {
            board: CardsOnBoard::new(),
            deck,
            hand1,
            hand2,
        }
    }

    fn make_move<T: PlayerTrait>(&mut self, player: Player, p_type: &T) {
        let hand = if player.get_player() == 1 {&mut self.hand1} else {&mut self.hand2};

        for stone in p_type.claim(hand, &self.board, player) {
            self.board.claim(player, stone);
        }

        if self.board.any_available_stones_for(player) {
            let (hand_index, chosen_stone) = p_type.choose_action(hand, &self.board, player);

            self.board.place_card(player, chosen_stone, hand.remove(hand_index));

            if let Some(card) = self.deck.draw() {
                hand.add(card);
            }
        }
    }

    pub fn play<T: PlayerTrait, S: PlayerTrait>(&mut self, player1: T, player2: S) -> Player {
        loop {
            if let Some(player) = self.board.terminal_state() {
                return player;
            } else {
                self.make_move(Player::build(0), &player1);
            }

            if let Some(player) = self.board.terminal_state() {
                return player;
            } else {
                self.make_move(Player::build(1), &player2);
            }
        }
    }
}
