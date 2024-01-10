use std::ops::Index;
use std::fmt;

use crate::components::Card;
use crate::common::{CARDS_IN_HAND, SPACE};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let mut hand = Hand::new();

        assert_eq!(hand.to_string(), "");

        hand.add(Card::build(1,3));
        hand.add(Card::build(5,1));
        hand.add(Card::build(8,6));

        assert_eq!(hand.to_string(), format!("Re1{SPACE}Pu5{SPACE}Bl8{SPACE}"));
    }
}

pub struct Hand {
    hand: Vec<Card>,
}
impl Hand {
    pub fn new() -> Self {
        Hand { hand: Vec::with_capacity(CARDS_IN_HAND as usize) }
    }

    pub fn remove(&mut self, index: usize) -> Card {
        self.hand.swap_remove(index)
    }

    pub fn add(&mut self, card: Card) {
        if self.hand.len() == CARDS_IN_HAND as usize {
            panic!("Hand is full!");
        }

        self.hand.push(card);
    }

    pub fn len(&self) -> usize {
        self.hand.len()
    }
}
impl Index<usize> for Hand {
    type Output = Card; 

    fn index(&self, idx: usize) -> &Self::Output { &self.hand[idx] }
}
impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut hand_str = String::new();

        for card in self.hand.iter() {
            let tmp = card.to_string();

            hand_str.push_str(&tmp);
            hand_str.push_str(SPACE);
        }

        write!(f, "{}", hand_str)
    }
}