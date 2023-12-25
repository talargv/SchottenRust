use crate::common::NUM_OF_STONES;
use crate::components::{Player, Stone};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claim() {
        let mut claims = Claims::new();

        claims.claim(Player::build(0), Stone::build(1));

        assert!(claims.claims[1].is_some());
        assert_eq!(claims.claims[1].unwrap(), Player::build(0));

        for i in 0..NUM_OF_STONES {
            if i == 1 { continue; }

            assert!(claims.claims[i as usize].is_none());
        }
    }

    #[test]
    fn test_claims() {
        let mut claims = Claims::new();

        let mut stones: Vec<Stone> = Vec::new();

        claims.claims(Player::build(1), &stones[..]);

        for i in 0..NUM_OF_STONES {
            assert!(claims.claims[i as usize].is_none());
        }

        stones.push(Stone::build(1));
        stones.push(Stone::build(5));

        claims.claims(Player::build(1), &stones[..]);

        assert!(claims.claims[1].is_some());
        assert_eq!(claims.claims[1].unwrap(), Player::build(1));

        assert!(claims.claims[5].is_some());
        assert_eq!(claims.claims[5].unwrap(), Player::build(1));
    }
}

#[derive(Debug, Clone)]
pub struct Claims {
    claims: [Option<Player>; NUM_OF_STONES as usize],
}
impl Claims {
    pub fn new() -> Self {
        Claims {claims: [None; NUM_OF_STONES as usize]} 
    }

    pub fn claim(&mut self, player: Player, stone: Stone) {
        self.claims[stone.get_index() as usize] = Some(player);
    }

    pub fn claims(&mut self, player: Player, stones: &[Stone]) {
        for stone in stones {
            self.claims[stone.get_index() as usize] = Some(player);
        }
    }

    pub fn who_claimed(&self, stone: Stone) -> Option<Player>{
        self.claims[stone.get_index()]
    }

    // ONLY FOR TESTING!!!
    pub fn unclaim(&mut self, stone: Stone) {
        self.claims[stone.get_index()] = None;
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Option<Player>>{
        self.claims.iter()
    }
}