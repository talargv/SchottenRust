use std::ops::Index;

use crate::common::NUM_OF_STONES;
use crate::components::{Player, Stone};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claim() {
        let mut claims = Advantage::new();

        claims.set(Player::build(0), Stone::build(1));

        assert!(claims.advantage[1].is_some());
        assert_eq!(claims.advantage[1].unwrap(), Player::build(0));

        for i in 0..NUM_OF_STONES {
            if i == 1 { continue; }

            assert!(claims.advantage[i as usize].is_none());
        }
    }

    #[test]
    fn test_claims() {
        let mut claims = Advantage::new();

        let mut stones: Vec<Stone> = Vec::new();

        claims.set_multi(Player::build(1), &stones[..]);

        for i in 0..NUM_OF_STONES {
            assert!(claims.advantage[i as usize].is_none());
        }

        stones.push(Stone::build(1));
        stones.push(Stone::build(5));

        claims.set_multi(Player::build(1), &stones[..]);

        assert!(claims.advantage[1].is_some());
        assert_eq!(claims.advantage[1].unwrap(), Player::build(1));

        assert!(claims.advantage[5].is_some());
        assert_eq!(claims.advantage[5].unwrap(), Player::build(1));
    }
}

pub struct Advantage {
    advantage: [Option<Player>; NUM_OF_STONES as usize],
}
impl Advantage {
    pub fn new() -> Self {
        Advantage {advantage: [None; NUM_OF_STONES as usize]} 
    }

    pub fn set(&mut self, player: Player, stone: Stone) {
        self.advantage[stone.get_index()] = Some(player);
    }

    pub fn set_multi(&mut self, player: Player, stones: &[Stone]) {
        for stone in stones {
            self.advantage[stone.get_index()] = Some(player);
        }
    }
}
impl Index<usize> for Advantage {
    type Output = Option<Player>; 

    fn index(&self, idx: usize) -> &Self::Output { &self.advantage[idx] }
}