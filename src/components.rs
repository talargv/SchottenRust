use crate::common::{NUM_OF_PLAYERS, NUM_OF_COLORS, NUM_OF_NUMS, NUM_OF_STONES};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_build_good() {
        Card::build(1,1);
        Card::build(NUM_OF_NUMS,NUM_OF_COLORS);
    }

    #[test]
    #[should_panic]
    fn card_build_bad_zero_num() {
        Card::build(0, 1);
    }

    #[test]
    #[should_panic]
    fn card_build_bad_zero_color() {
        Card::build(1, 0);
    }

    #[test]
    #[should_panic]
    fn card_build_bad_high_color() {
        Card::build(1, NUM_OF_COLORS+1);
    }

    #[test]
    #[should_panic]
    fn card_build_bad_high_num() {
        Card::build(NUM_OF_NUMS+1, 6);
    }

    #[test]
    fn card_eq() {
        assert_eq!(Card::build(1,5), Card::build(1,5));
        assert_ne!(Card::build(1,5), Card::build(5,1));
    }

    #[test]
    fn player_build_good() {
        for p in 0..NUM_OF_PLAYERS { Player::build(p); }
    }

    #[test]
    #[should_panic]
    fn player_build_bad() {
        Player::build(NUM_OF_PLAYERS);
    }

    #[test]
    fn player_other() {
        assert!(NUM_OF_PLAYERS == 2);
        let p1 = Player::build(0);
        let p2 = Player::build(1);

        assert_eq!(p1.get_other(), p2);
        assert_eq!(p1, p2.get_other());
    }

    #[test]
    fn stone_build_good() {
        for s in 0..NUM_OF_STONES { Stone::build(s); }
    }

    #[test]
    #[should_panic]
    fn stone_build_bad() {
        Stone::build(NUM_OF_STONES);
    }

}

#[derive(PartialEq, PartialOrd, Clone, Hash, Eq, Debug)]
pub struct Card {
    num: u8,
    color: u8,
}
impl Card {
    pub fn build(num: u8, color: u8) -> Self {
        if num < 1 || num > NUM_OF_NUMS || color < 1 || color > NUM_OF_COLORS {
            panic!("Invalid number or color.\n
                Expected num in range [1, NUM_OF_NUMS], got {}.\n
                Expected color in range [1, NUM_OF_COLORS], got {}", num, color);
        }

        Card { num, color }
    }

    pub fn num(&self) -> u8 { self.num }

    pub fn num_index(&self) -> usize { (self.num-1) as usize }

    pub fn color(&self) -> u8 { self.color }

    pub fn color_index(&self) -> usize { (self.color-1) as usize }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Player {
    p: u8,
}
impl Player {
    pub fn build(p: u8) -> Self {
        if p >= NUM_OF_PLAYERS {panic!("Player index must be in range [0, NUM_OF_PLAYERS), got {}", p);}

        Player {p}
    }

    pub fn get_player(&self) -> u8 {
        self.p + 1
    }

    pub fn get_index(&self) -> usize {
        self.p as usize
    }

    pub fn get_other(&self) -> Self {
        // Assumes the following:
        assert!(NUM_OF_PLAYERS == 2);

        Player { p: 1-self.p }
    }

}

#[derive(Clone, Copy, PartialEq)]
pub struct Stone {
    s: u8,
}
impl Stone {
    pub fn build(s: u8) -> Self {
        if s >= NUM_OF_STONES {panic!("Stone index must be in range [0, NUM_OF_STONES), got {}", s);}

        Stone {s}
    }

    pub fn get_stone(&self) -> u8 {
        self.s + 1
    }

    pub fn get_index(&self) -> usize {
        self.s as usize
    }
}