pub mod random_player;
pub mod analog_player;

use crate::board::cards_on_board::CardsOnBoard;
use crate::board::hand::Hand;
use crate::components::{Stone, Player as PlayerTag};
use crate::common::NUM_OF_STONES;

pub trait Player {
    fn choose_action(&self, hand: &Hand, board: &CardsOnBoard, player: PlayerTag) -> (usize, Stone);

    fn claim(&self, _hand: &Hand, _board: &CardsOnBoard, _player: PlayerTag) -> Vec<Stone> {
        (0..NUM_OF_STONES).map(|s| Stone::build(s)).collect()
    }
}