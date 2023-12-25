use std::ops::Index;

use crate::components::Card;
use crate::common::CARDS_IN_HAND;

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