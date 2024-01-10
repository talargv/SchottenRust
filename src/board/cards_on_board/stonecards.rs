use std::ops::Index;

use crate::common::STONE_CARDS_LIMIT;
use crate::components::Card;

#[cfg(test)]
mod tests {
    use super::*;

    use itertools::Itertools;

    #[test]
    fn test_is_full() {
        let mut cards = StoneCards::new();

        for _ in 0..STONE_CARDS_LIMIT {
            cards.push(Card::build(1,1));
        }

        assert!(cards.is_full());
    }

    #[test]
    fn test_strength() {
        let sum_low = vec![Card::build(1,1), Card::build(1,2), Card::build(2,1)]; // 0
        let sum_high = vec![Card::build(9,1), Card::build(9,2), Card::build(8,2)]; // 1

        let run_low = vec![Card::build(1,3),Card::build(2,2),Card::build(3,2)]; // 2
        let run_high = vec![Card::build(7,1),Card::build(8,1),Card::build(9,3)]; // 3

        let color_low = vec![Card::build(1,4), Card::build(2,4), Card::build(4,4)]; // 4
        let color_high = vec![Card::build(6,4), Card::build(8,4), Card::build(9,4)]; // 5

        // not valid against sum_low
        let three_of_low_v1 = vec![Card::build(1,5), Card::build(1,6), Card::build(1,1)]; // 6 
        // not valid against run_low
        let three_of_low_v2 = vec![Card::build(1,5), Card::build(1,6), Card::build(1,3)]; // 7
        // not valid against sum_high
        let three_of_high_v1 = vec![Card::build(9,5), Card::build(9,6), Card::build(9,1)]; // 8
        // not valid against run_high
        let three_of_high_v2 = vec![Card::build(9,5), Card::build(9,6), Card::build(9,3)]; // 9

        // not valid against three_of_low_v1
        let color_run_low_v1 = vec![Card::build(1,5), Card::build(2,5), Card::build(3,5)]; // 10
        // not valid against run_low, three_of_low_v2
        let color_run_low_v2 = vec![Card::build(1,3), Card::build(2,3), Card::build(3,3)]; // 11
        // not valid against three_of_high_v1
        let color_run_high_v1 = vec![Card::build(9,5), Card::build(8,5), Card::build(7,5)]; // 12
        // not valid against color_high
        let color_run_high_v2 = vec![Card::build(9,4), Card::build(8,4), Card::build(7,4)]; // 13

        let mut all_stuff: Vec<StoneCards> = Vec::new();

        all_stuff.push(StoneCards { cards:sum_low });
        all_stuff.push(StoneCards { cards:sum_high });
        all_stuff.push(StoneCards { cards:run_low });
        all_stuff.push(StoneCards { cards:run_high });
        all_stuff.push(StoneCards { cards:color_low });
        all_stuff.push(StoneCards { cards:color_high });
        all_stuff.push(StoneCards { cards:three_of_low_v1 });
        all_stuff.push(StoneCards { cards:three_of_low_v2 });
        all_stuff.push(StoneCards { cards:three_of_high_v1 });
        all_stuff.push(StoneCards { cards:three_of_high_v2 });
        all_stuff.push(StoneCards { cards:color_run_low_v1 });
        all_stuff.push(StoneCards { cards:color_run_low_v2 });
        all_stuff.push(StoneCards { cards:color_run_high_v1 });
        all_stuff.push(StoneCards { cards:color_run_high_v2 });

        let not_forbidden_games = |(i1,i2): (usize,usize)| {
            let tmp = (i1, i2);
            !(i1 <= i2 
            || tmp == (6,1) 
            || tmp == (7,6) || tmp == (7,2)
            || tmp == (8,1) 
            || tmp == (9,8) || tmp == (9,3)
            || tmp == (10,6) 
            || tmp == (11,10) || tmp == (11,2) || tmp == (11,7)
            || tmp == (12,9)
            || tmp == (13,12) || tmp == (13,5))
        };

        for (i1,i2) in (0..all_stuff.len()).cartesian_product(0..all_stuff.len()).filter(|item| not_forbidden_games(item.clone())) {
            let str1 = all_stuff[i1].strength();
            let str2 = all_stuff[i2].strength();
            assert!(str1 >  str2, "{:?} - {str1} \n {:?} - {str2}", all_stuff[i1], all_stuff[i2]);
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct StoneCards {
    // Using tinyvec\smallvec might be faster.
    cards: Vec<Card>,
}
impl StoneCards {
    pub fn new() -> Self {
        StoneCards { cards: Vec::with_capacity(3) }
    }

    pub fn is_full(&self) -> bool { self.cards.len() == STONE_CARDS_LIMIT as usize }

    pub fn push(&mut self, card: Card) {
        if self.is_full() { panic!("Cannot push new card to a full struct"); }

        self.cards.push(card);
    }

    pub fn strength(&self) -> u8 {
        /*
        Returns strength of card sequence, according to the following table:
        
        Type            | Value
        ----------------|---------
        sum             | 4-26
        run             | 27-33
        color           | 40-56
        three of a kind | 57-65
        color run       | 66-72

         */

        if !self.is_full() { panic!("Cannot determine strength of a non-full struct");}

        // assert!(STONE_CARDS_LIMIT == 3)

        // Check three of a kind 
        if self.cards[0].num() == self.cards[1].num() && self.cards[1].num() == self.cards[2].num() {
            return 56 + self.cards[0].num();
        }

        let mut is_run = false;
        let mut is_color = false;

        let mut cards_sorted = self.cards.clone();
        cards_sorted.sort_unstable_by_key(|card| card.num());

        // Check run
        if cards_sorted[0].num() + 1 == cards_sorted[1].num() && cards_sorted[1].num() + 1 == cards_sorted[2].num() {
            is_run = true;
        }

        // Check color
        if self.cards[0].color() == self.cards[1].color() && self.cards[1].color() == self.cards[2].color() {
            is_color = true;
        }

        if is_run && is_color { 
            65+cards_sorted[0].num() 
        } else if is_run {
            26+cards_sorted[0].num()
        } else if is_color {
            cards_sorted[0].num() + cards_sorted[1].num() + cards_sorted[2].num() + 33
        } else {
            cards_sorted[0].num() + cards_sorted[1].num() + cards_sorted[2].num()
        }
    }

    pub fn iter(&self) -> core::slice::Iter<'_, Card>{
        self.cards.iter()
    }

    pub fn len(&self) -> usize { self.cards.len() }
}

impl Index<usize> for StoneCards {
    type Output = Card; 

    fn index(&self, idx: usize) -> &Self::Output { &self.cards[idx] }
}
