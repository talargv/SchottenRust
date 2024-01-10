use super::{Player, Hand, CardsOnBoard, PlayerTag, Stone};

use rand::{prelude::thread_rng, Rng};

pub struct RandomPlayer;
impl Player for RandomPlayer {
    fn choose_action(&self, hand: &Hand, board: &CardsOnBoard, player: PlayerTag) -> (usize, Stone) {
        let mut rng = thread_rng();

        let available_stones = board.available_stones_for(player);

        (rng.gen_range(0..hand.len()), available_stones[rng.gen_range(0..available_stones.len())])
    }
}