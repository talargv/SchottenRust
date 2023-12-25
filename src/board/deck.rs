use std::ops::Index;

use crate::common::{CARDS_IN_DECK, NUM_OF_NUMS, NUM_OF_COLORS};
use crate::components::Card;

use rand::seq::SliceRandom;
use rand::thread_rng;

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn deck_new() {
        let deck = Deck::new();

        let all_cards = (1..=NUM_OF_NUMS)
            .cartesian_product(1..=NUM_OF_COLORS)
            .map(|(num,color)| Card::build(num, color));

        for card in all_cards {
            assert!(deck.deck.contains(&card));
        }

        assert_eq!(deck.len(), 54);
    }

    #[test]
    fn deck_new_shuffled() {
        // Has a 1/(CARDS_IN_DECK!) chance of a false positive

        assert_ne!(Deck::new().deck, Deck::new().deck);
    }

    #[test]
    fn deck_draw() {
        let mut deck = Deck::new();

        for _ in 0..CARDS_IN_DECK {
            assert!(deck.draw().is_some());
        }
        assert!(deck.draw().is_none());
    }
}

pub struct Deck {
    deck: Vec<Card>,
}
impl Deck {
    pub fn new() -> Self {
        let mut deck: Vec<Card> = Vec::with_capacity(CARDS_IN_DECK as usize);

        for num in 1..=NUM_OF_NUMS {
            for color in 1..=NUM_OF_COLORS{
                deck.push(Card::build(num, color));
            }
        }

        deck.shuffle(&mut thread_rng());

        Deck { deck }
    }

    pub fn len(&self) -> usize { self.deck.len() }

    pub fn draw(&mut self) -> Option<Card> { self.deck.pop() }
}
impl Index<usize> for Deck {
    type Output = Card; 

    fn index(&self, idx: usize) -> &Self::Output { &self.deck[idx] }
}