mod stonecards;
mod advantage;
mod claims;
mod jobs;

use std::fmt;

use advantage::Advantage;
use crate::common::{NUM_OF_STONES, NUM_OF_PLAYERS, NUM_OF_COLORS, NUM_OF_NUMS, STONE_CARDS_LIMIT, SPACE, STONE_STR};
use crate::components::{Player, Stone, Card};
use claims::Claims;
use stonecards::StoneCards;

use itertools::Itertools;

#[cfg(test)]
mod tests {
    use std::{rc::Rc, cell::RefCell};

    use super::*;

    use rand::Rng;

    #[test]
    fn cards_on_board_place_card_good() {
        let mut board = CardsOnBoard::new();

        for player in (0..NUM_OF_PLAYERS).map(|i| Player::build(i)) {
            for stone_idx in 0..NUM_OF_STONES {
                let stone = Stone::build(stone_idx);

                board.place_card(player, stone, Card::build(stone_idx+1, player.get_player()));
            }
        }

        for player in 0..NUM_OF_PLAYERS {
            for stone in 0..NUM_OF_STONES {
                let cards = &board.cards[player as usize][stone as usize];

                assert!(cards.len() == 1);

                let card = cards.iter().last();

                assert_eq!(*card.unwrap(), Card::build(stone+1, player+1));
            }
        }

        for player in (0..NUM_OF_PLAYERS).map(|i| Player::build(i)) {
            for stone_idx in 0..NUM_OF_STONES {
                let stone = Stone::build(stone_idx);

                board.place_card(player, stone, Card::build(stone_idx+1, player.get_player()+1));
            }
        }

        for player in 0..NUM_OF_PLAYERS {
            for stone in 0..NUM_OF_STONES {
                let cards = &board.cards[player as usize][stone as usize];

                assert!(cards.len() == 2);

                let card = cards.iter().last();

                assert_eq!(*card.unwrap(), Card::build(stone+1, player+2));
            }
        }
    }
    
    #[test]
    #[should_panic]
    fn cards_on_board_place_card_bad() {
        let mut board = CardsOnBoard::new();

        for _ in 0..STONE_CARDS_LIMIT {
            board.place_card(
                Player::build(0),
                Stone::build(0), 
                Card::build(1,1)
            );
        }

        board.place_card(
            Player::build(0),
            Stone::build(0), 
            Card::build(1,1)
        );
    }

    fn available_stones_rec(claims: Rc<RefCell<Claims>>, stone: u8) -> Vec<Stone> {
        if stone == NUM_OF_STONES {
            let mut board = CardsOnBoard::new();

            board.claims = claims.borrow().clone();

            board.available_stones()
        } else {
            let res_none = available_stones_rec(Rc::clone(&claims), stone+1);
            assert!(res_none.contains(&Stone::build(stone)));

            claims.borrow_mut().claim(Player::build(0), Stone::build(stone));
            let res_p1 = available_stones_rec(Rc::clone(&claims), stone+1);
            assert!(!res_p1.contains(&Stone::build(stone)));
            claims.borrow_mut().unclaim(Stone::build(stone));

            claims.borrow_mut().claim(Player::build(1), Stone::build(stone));
            let res_p2 = available_stones_rec(Rc::clone(&claims), stone+1);
            assert!(!res_p2.contains(&Stone::build(stone)));
            claims.borrow_mut().unclaim(Stone::build(stone));

            match rand::thread_rng().gen_range(0..=2) {
                0 => res_none,
                1 => res_p1,
                2 => res_p2,
                _ => panic!("Greatly unexpected")
            }
        }
    }

    #[test]
    fn cards_on_board_available_stones() {
        available_stones_rec(Rc::new(RefCell::new(Claims::new())), 0);
    }

    #[test]
    fn present_cards_add_card() {
        let mut present_cards = PresentCards::new();

        let all_cards = || {
            (1..=NUM_OF_NUMS)
            .cartesian_product(1..=NUM_OF_COLORS)
            .map(|(num,color)| Card::build(num, color))
        };

        for card1 in all_cards() {
            present_cards.add_card(&card1);

            assert!(present_cards.cards[card1.num_index()][card1.color_index()]);

            let tmp_iter = all_cards();

            // Assumes present_cards.cards is [[bool; NUM_OF_COLORS]; NUM_OF_NUMS]
            let mut switch = false;

            for card2 in tmp_iter {
                if card2 == card1 {
                    switch = true;
                    continue;
                }

                if switch {
                    assert!(!present_cards.cards[card2.num_index()][card2.color_index()]);
                } else {
                    assert!(present_cards.cards[card2.num_index()][card2.color_index()]);
                }
            }
        }
    }

    #[test]
    fn present_cards_is_present() {
        let mut present_cards = PresentCards::new();

        let all_cards = || {
            (1..=NUM_OF_NUMS)
            .cartesian_product(1..=NUM_OF_COLORS)
            .map(|(num,color)| Card::build(num, color))
        };

        for card1 in all_cards() {
            present_cards.add_card(&card1);

            assert!(present_cards.is_present(&card1));

            let tmp_iter = all_cards();

            // Assumes present_cards.cards is [[bool; NUM_OF_COLORS]; NUM_OF_NUMS]
            let mut switch = false;

            for card2 in tmp_iter {
                if card2 == card1 {
                    switch = true;
                    continue;
                }

                if switch {
                    assert!(!present_cards.is_present(&card2));
                } else {
                    assert!(present_cards.is_present(&card2));
                }
            }
        }
    }
}

#[cfg(test)]
mod test_proto_legal_claim {
    use super::*;

    use rand::Rng;


    #[test]
    fn test_less_than_three() {
        let mut board = CardsOnBoard::new();

        // second stone
        board.place_card(Player::build(1), Stone::build(1), Card::build(1,1));

        // third stone
        board.place_card(Player::build(1), Stone::build(2), Card::build(1,2));
        board.place_card(Player::build(1), Stone::build(2), Card::build(1,3));

        // forth stone
        board.place_card(Player::build(0), Stone::build(3), Card::build(1,4));
        
        // fifth stone
        board.place_card(Player::build(0), Stone::build(4), Card::build(1,5));
        board.place_card(Player::build(1), Stone::build(4), Card::build(1, 6));

        // sixth stone
        board.place_card(Player::build(0), Stone::build(5), Card::build(2,1));
        board.place_card(Player::build(1), Stone::build(5), Card::build(2, 2));
        board.place_card(Player::build(1), Stone::build(5), Card::build(2, 3));

        // seventh stone
        board.place_card(Player::build(0), Stone::build(6), Card::build(2,4));
        board.place_card(Player::build(0), Stone::build(6), Card::build(2,5));

        // eighth stone
        board.place_card(Player::build(0), Stone::build(7), Card::build(2,6));
        board.place_card(Player::build(0), Stone::build(7), Card::build(3,1));
        board.place_card(Player::build(1), Stone::build(7), Card::build(3,2));

        // ninth stone
        board.place_card(Player::build(0), Stone::build(8), Card::build(3,3));
        board.place_card(Player::build(0), Stone::build(8), Card::build(3,4));
        board.place_card(Player::build(1), Stone::build(8), Card::build(3,5));
        board.place_card(Player::build(1), Stone::build(8), Card::build(3,6));

        for player in (0..NUM_OF_PLAYERS).map(|p| Player::build(p)) {
            for stone in (0..NUM_OF_STONES).map(|s| Stone::build(s)) {
                assert!(!board.proto_is_legal_claim(player, stone));
            }
        }
    }

    fn compare(player: Player, vec1: &Vec<Card>, vec2: &Vec<Card>, first_has_advantage: bool) -> bool {
        assert!(vec1.len() == vec2.len());

        let mut board = CardsOnBoard::new();

        let stone = Stone::build(rand::thread_rng().gen_range(0..NUM_OF_STONES));

        for i in 0..vec1.len() {
            if first_has_advantage {
                board.place_card(player, stone, vec1[i].clone());
                board.place_card(player.get_other(), stone, vec2[i].clone());
            } else {
                board.place_card(player.get_other(), stone, vec2[i].clone());
                board.place_card(player, stone, vec1[i].clone());
            }
        }

        board.proto_is_legal_claim(player, stone)
    }

    // Assumes vec1 is stronger than vec2
    fn all_options_winning_hands(vec1: &Vec<Card>, vec2: &Vec<Card>) -> bool {
        assert!(vec1.len() == vec2.len());

        let player1 = Player::build(0);

        compare(player1, vec1, vec2, true) 
        && compare(player1, vec1, vec2, false)
        && compare(player1.get_other(), vec1, vec2, true)
        && compare(player1.get_other(), vec1, vec2, false)
    }

    #[test]
    fn test_winning_hands() {
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

        let mut all_stuff: Vec<Vec<Card>> = Vec::new();

        all_stuff.push(sum_low);
        all_stuff.push(sum_high);
        all_stuff.push(run_low);
        all_stuff.push(run_high);
        all_stuff.push(color_low);
        all_stuff.push(color_high);
        all_stuff.push(three_of_low_v1);
        all_stuff.push(three_of_low_v2);
        all_stuff.push(three_of_high_v1);
        all_stuff.push(three_of_high_v2);
        all_stuff.push(color_run_low_v1);
        all_stuff.push(color_run_low_v2);
        all_stuff.push(color_run_high_v1);
        all_stuff.push(color_run_high_v2);

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
            assert!(all_options_winning_hands(&all_stuff[i1], &all_stuff[i2]), "{:?}\n{:?}", &all_stuff[i1], &all_stuff[i2]);
        }
    }

    #[test]
    fn test_ties() {
        let mut all_cards_vec: Vec<Vec<Card>> = Vec::new();

        let all_cards = || {
            (1..=NUM_OF_NUMS)
            .cartesian_product(1..=NUM_OF_COLORS)
            .map(|(num,color)| Card::build(num, color))
        };

        for card1 in all_cards() {
            for card2 in all_cards().filter(|card| *card != card1) {
                for card3 in all_cards().filter(|card| *card != card2 && *card != card1) {
                    all_cards_vec.push(vec![card1.clone(), card2.clone(), card3]);
                }
            }
        }

        for cards in &all_cards_vec {
            assert!(compare(Player::build(0), cards, cards, true));
            assert!(!compare(Player::build(0), cards, cards, false));
            assert!(compare(Player::build(1), cards, cards, true));
            assert!(!compare(Player::build(1), cards, cards, false));
        }
    }

    #[test]
    fn test_789_color_run() {
        let mut board = CardsOnBoard::new();

        board.place_card(Player::build(0), Stone::build(0), Card::build(7,1));
        board.place_card(Player::build(0), Stone::build(0), Card::build(8,1));
        board.place_card(Player::build(0), Stone::build(0), Card::build(9,1));

        assert!(board.proto_is_legal_claim(Player::build(0), Stone::build(0)));
    }

    #[test]
    fn test_123_color_run() {
        let player = Player::build(1);
        let stone = Stone::build(1);

        let tmp_fn = || {
            let mut board = CardsOnBoard::new();

            board.place_card(player, stone, Card::build(1,2));
            board.place_card(player, stone, Card::build(2,2));
            board.place_card(player, stone, Card::build(3,2));

            board
        };

        let mut board = tmp_fn();

        board.place_card(player.get_other(), stone, Card::build(1,6));

        assert!(board.proto_is_legal_claim(player, stone), "Mini Test 1");

        board = tmp_fn();

        board.place_card(player.get_other(), stone, Card::build(2,6));

        assert!(!board.proto_is_legal_claim(player, stone), "Mini Test 2");

        board = tmp_fn();

        board.place_card(player.get_other(), stone, Card::build(7,5));
        board.place_card(player.get_other(), stone, Card::build(8,5));

        let stone2 = Stone::build(5);

        board.place_card(player, stone2, Card::build(9,1));
        board.place_card(player, stone2, Card::build(9,2));
        board.place_card(player, stone2, Card::build(9,3));
        board.place_card(player.get_other(), stone2, Card::build(9,4));
        board.place_card(player.get_other(), stone2, Card::build(9,5));
        board.place_card(player.get_other(), stone2, Card::build(9,6));


        let stone2 = Stone::build(6);

        board.place_card(player, stone2, Card::build(6,1));
        board.place_card(player, stone2, Card::build(6,2));
        board.place_card(player, stone2, Card::build(6,3));
        board.place_card(player.get_other(), stone2, Card::build(6,4));
        board.place_card(player.get_other(), stone2, Card::build(6,5));
        board.place_card(player.get_other(), stone2, Card::build(6,6));

        assert!(board.proto_is_legal_claim(player, stone), "Mini Test 3");
    }

    #[test]
    fn test_999() {
        let player = Player::build(0);
        let stone = Stone::build(2);

        let mut board = CardsOnBoard::new();

        board.place_card(player.get_other(), stone, Card::build(1,6));
        board.place_card(player, stone, Card::build(9,1));
        board.place_card(player, stone, Card::build(9,4));
        board.place_card(player, stone, Card::build(9,5));

        assert!(!board.proto_is_legal_claim(player, stone));

        board.place_card(player, Stone::build(6), Card::build(2,1));
        board.place_card(player, Stone::build(6), Card::build(2,2));
        board.place_card(player, Stone::build(6), Card::build(2,3));
        board.place_card(player.get_other(), Stone::build(6), Card::build(2,4));
        board.place_card(player.get_other(), Stone::build(6), Card::build(2,5));
        board.place_card(player.get_other(), Stone::build(6), Card::build(2,6));

        assert!(board.proto_is_legal_claim(player, stone));
    }

    #[test]
    fn test_111() {
        let player = Player::build(1);
        let stone = Stone::build(3);

        let mut board = CardsOnBoard::new();

        board.place_card(player.get_other(), stone, Card::build(1,6));
        board.place_card(player, stone, Card::build(1,2));
        board.place_card(player, stone, Card::build(1,4));
        board.place_card(player, stone, Card::build(1,5));

        assert!(!board.proto_is_legal_claim(player, stone));

        board.place_card(player, Stone::build(6), Card::build(2,1));
        board.place_card(player, Stone::build(6), Card::build(2,2));
        board.place_card(player, Stone::build(6), Card::build(2,3));
        board.place_card(player.get_other(), Stone::build(6), Card::build(2,4));
        board.place_card(player.get_other(), Stone::build(6), Card::build(2,5));
        board.place_card(player.get_other(), Stone::build(6), Card::build(2,6));

        assert!(board.proto_is_legal_claim(player, stone));

        board = CardsOnBoard::new();

        let stone = Stone::build(1);

        board.place_card(player, stone, Card::build(1,2));
        board.place_card(player, stone, Card::build(1,4));
        board.place_card(player, stone, Card::build(1,5));
        board.place_card(player.get_other(), stone, Card::build(7,5));
        board.place_card(player.get_other(), stone, Card::build(4,5));

        assert!(board.proto_is_legal_claim(player, stone));
    }

    #[test]
    fn test_689_color() {
        let player = Player::build(0);
        let stone = Stone::build(4);

        let tmp_fn = || {

        let mut board = CardsOnBoard::new();

        board.place_card(player, stone, Card::build(6,4));
        board.place_card(player, stone, Card::build(8,4));
        board.place_card(player, stone, Card::build(9,4));
        board.place_card(player.get_other(), stone, Card::build(2,1));

        board

        };

        let mut board = tmp_fn();

        assert!(!board.proto_is_legal_claim(player, stone));

        board.place_card(player.get_other(), stone, Card::build(2,4));

        assert!(!board.proto_is_legal_claim(player, stone));

        board = tmp_fn();

        board.place_card(player.get_other(), stone , Card::build(3,1));

        assert!(!board.proto_is_legal_claim(player, stone));

        board = tmp_fn();

        board.place_card(player.get_other(), stone , Card::build(3,6));

        assert!(board.proto_is_legal_claim(player, stone));

        board = CardsOnBoard::new();

        board.place_card(player, stone, Card::build(6,4));
        board.place_card(player, stone, Card::build(8,4));
        board.place_card(player, stone, Card::build(9,4));

        board.place_card(player.get_other(), stone, Card::build(6,5));
        board.place_card(player.get_other(), stone, Card::build(9,5));

        assert!(board.proto_is_legal_claim(player, stone));
    }

    #[test]
    fn test_124_color() {
        let player = Player::build(1);
        let stone = Stone::build(5);
        let other = player.get_other();

        let tmp_fn = || {
            let mut board = CardsOnBoard::new();

            board.place_card(player, stone, Card::build(4,6));
            board.place_card(player, stone, Card::build(1,6));
            board.place_card(player, stone, Card::build(2,6));

            board
        };

        let mut board = tmp_fn();

        board.place_card(other, stone, Card::build(4,2));
        board.place_card(other, stone, Card::build(1,2));

        assert!(!board.proto_is_legal_claim(player, stone));

        board = tmp_fn();

        board.place_card(other, stone, Card::build(9,2));
        board.place_card(other, stone, Card::build(6,3));

        assert!(board.proto_is_legal_claim(player, stone));

        board = tmp_fn();

        board.place_card(other, stone, Card::build(2,1));
        board.place_card(other, stone, Card::build(3,2));

        assert!(board.proto_is_legal_claim(player, stone));
    }

    #[test]
    fn test_123() {
        let player = Player::build(0);
        let stone = Stone::build(6);
        let other = player.get_other();

        let tmp_fn = || {
            let mut board = CardsOnBoard::new();

            board.place_card(player, stone, Card::build(3,3));
            board.place_card(player, stone, Card::build(1,3));
            board.place_card(player, stone, Card::build(2,2));

            board
        };

        let mut board = tmp_fn();

        board.place_card(other, stone, Card::build(1,6));
        board.place_card(other, stone, Card::build(2,6));

        assert!(!board.proto_is_legal_claim(player, stone));

        board = tmp_fn();

        board.place_card(other, stone, Card::build(1,6));
        board.place_card(other, stone, Card::build(2,5));

        assert!(board.proto_is_legal_claim(player, stone));

        board = tmp_fn();

        board.place_card(other, stone, Card::build(6,3));
        board.place_card(other, stone, Card::build(9,3));

        let first_stone = Stone::build(0);

        board.place_card(player, first_stone, Card::build(2,3));
        board.place_card(player, first_stone, Card::build(4,3));
        board.place_card(player, first_stone, Card::build(5,3));
        board.place_card(other, first_stone, Card::build(7,3));
        board.place_card(other, first_stone, Card::build(8,3));

        assert!(board.proto_is_legal_claim(player, stone));

        board = tmp_fn();

        board.place_card(other, stone, Card::build(6,3));
        board.place_card(other, stone, Card::build(9,4));

        assert!(board.proto_is_legal_claim(player, stone));
    }

    #[test]
    fn test_789() {
        let player = Player::build(1);
        let stone = Stone::build(0);
        let other = player.get_other();

        let mut board = CardsOnBoard::new();

        board.place_card(other, stone, Card::build(6,2));
        board.place_card(other, stone, Card::build(7,1));
        board.place_card(player, stone, Card::build(8,2));
        board.place_card(player, stone, Card::build(7,4));
        board.place_card(player, stone, Card::build(9,3));

        assert!(board.proto_is_legal_claim(player, stone));
    }

    #[test]
    fn test_689() {
        let player = Player::build(0);
        let stone = Stone::build(1);
        let other = player.get_other();

        let tmp_fn = || {
            let mut board = CardsOnBoard::new();

            board.place_card(player, stone, Card::build(6,1));
            board.place_card(player, stone, Card::build(9,6));
            board.place_card(player, stone, Card::build(8,1));

            board
        };

        let mut board = tmp_fn();

        board.place_card(other, stone, Card::build(7,1));
        board.place_card(other, stone, Card::build(8,5));

        assert!(!board.proto_is_legal_claim(player, stone));

        board = tmp_fn();

        board.place_card(other, stone, Card::build(7,5));
        board.place_card(other, stone, Card::build(8,5));

        let last_stone = Stone::build(NUM_OF_STONES-1);
        
        board.place_card(player, last_stone, Card::build(9,1));
        board.place_card(player, last_stone, Card::build(9,2));

        board.place_card(other, last_stone, Card::build(9,3));
        board.place_card(other, last_stone, Card::build(9,4));
        board.place_card(other, last_stone, Card::build(9,5));

        assert!(!board.proto_is_legal_claim(player, stone));

        assert!(NUM_OF_STONES >= 2);
        let second_to_last_stone = Stone::build(NUM_OF_STONES-2);

        board.place_card(player, second_to_last_stone, Card::build(6,2));
        board.place_card(player, second_to_last_stone, Card::build(6,3));
        board.place_card(player, second_to_last_stone, Card::build(6,4));

        board.place_card(other, second_to_last_stone, Card::build(6,5));
        board.place_card(other, second_to_last_stone, Card::build(6,6));

        assert!(!board.proto_is_legal_claim(player, stone));

        let stone3 = Stone::build(2);

        board.place_card(player, stone3, Card::build(1,5));
        board.place_card(player, stone3, Card::build(2,5));
        board.place_card(player, stone3, Card::build(3,5));

        board.place_card(other, stone3, Card::build(4,5));
        board.place_card(other, stone3, Card::build(5,5));

        assert!(board.proto_is_legal_claim(player, stone));
    }

    #[test]
    fn test_899() {
        let player = Player::build(1);
        let stone = Stone::build(2);
        let other = player.get_other();

        let tmp_fn = || {
            let mut board = CardsOnBoard::new();

            board.place_card(player, stone, Card::build(9,1));
            board.place_card(player, stone, Card::build(9,6));
            board.place_card(player, stone, Card::build(8,1));

            board
        };

        let mut board = tmp_fn();

        board.place_card(other, stone, Card::build(9,2));

        let last_stone = Stone::build(NUM_OF_STONES-1);

        board.place_card(player, last_stone, Card::build(9,3));
        board.place_card(player, last_stone, Card::build(9,4));
        board.place_card(player, last_stone, Card::build(9,5));
        
        assert!(!board.proto_is_legal_claim(player, stone));

        board = tmp_fn();

        board.place_card(other, stone, Card::build(9,2));
        board.place_card(other, stone, Card::build(9,3));

        board.place_card(player, last_stone, Card::build(9,4));
        board.place_card(player, last_stone, Card::build(9,5));

        assert!(board.proto_is_legal_claim(player, stone));
    }
}

#[cfg(test)]
mod test_terminal_state {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;

    // Tests for terminal_state assumes that implementation depends only on the claimed stones.
    // The tests currently access self.claims (self is an instance of CardsOnBoard).

    fn five_claimed_rec(claims: Rc<RefCell<Claims>>, player: Player, stone: usize, count: u8, others_streak: u8) {
        if stone == NUM_OF_STONES as usize {
            let mut board = CardsOnBoard::new();
            
            board.claims = claims.borrow().clone();

            let res = board.terminal_state();

            assert!(res.is_some() && res.unwrap() == player);
        } else if NUM_OF_STONES - stone as u8 == count {
            for s in stone as u8..NUM_OF_STONES {
                claims.borrow_mut().claim(player, Stone::build(s as u8));
            }

            five_claimed_rec(Rc::clone(&claims), player, NUM_OF_STONES as usize, count, others_streak);

            for s in stone as u8..NUM_OF_STONES {
                claims.borrow_mut().unclaim(Stone::build(s as u8));
            }
        } else {
            if others_streak < 2 {
                claims.borrow_mut().claim(player.get_other(), Stone::build(stone as u8));

                five_claimed_rec(Rc::clone(&claims), player, stone+1, count, others_streak+1);

                claims.borrow_mut().unclaim(Stone::build(stone as u8));
            }

            five_claimed_rec(Rc::clone(&claims), player, stone+1, count, 0);

            if count > 0 {
                claims.borrow_mut().claim(player, Stone::build(stone as u8));

                five_claimed_rec(Rc::clone(&claims), player, stone+1, count-1, 0);

                claims.borrow_mut().unclaim(Stone::build(stone as u8));
            }
        }
    }

    #[test]
    fn test_five_claimed() {
        five_claimed_rec(
            Rc::new(RefCell::new(Claims::new())),
            Player::build(0),
            0,
            5,
            0
        );

        five_claimed_rec(
            Rc::new(RefCell::new(Claims::new())),
            Player::build(1),
            0,
            5,
            0
        );
    }

    fn three_streak_rec(
        claims: Rc<RefCell<Claims>>,
        player: Player,
        stone: usize,
        player_count: u8,
        player_streak: u8,
        streak_done: bool,
        other_count: u8,
        other_streak: u8,)
    {
        let mut tmp_streak_done = streak_done;

        if player_streak == 3 {
            tmp_streak_done = true;
        }

        if stone == NUM_OF_STONES as usize {
            let mut board = CardsOnBoard::new();
            
            board.claims = claims.borrow().clone();

            let res = board.terminal_state();

            assert!(res.is_some() && res.unwrap() == player, "{:?}\nplayer_count:{player_count} player_streak:{player_streak} {streak_done}", board.claims);
        } else {
            if (!tmp_streak_done) && ((NUM_OF_STONES - stone as u8 == 3 - player_streak) || stone == 6) {
                for s in stone..(stone + 3 - player_streak as usize) {
                    claims.borrow_mut().claim(player, Stone::build(s as u8));
                }

                three_streak_rec(
                    Rc::clone(&claims),
                    player,
                    NUM_OF_STONES as usize,
                    player_count,
                    player_streak,
                    tmp_streak_done,
                    other_count,
                    other_streak
                );

                for s in stone..(stone + 3 - player_streak as usize) {
                    claims.borrow_mut().unclaim(Stone::build(s as u8));
                }
            } else {
                three_streak_rec(
                    Rc::clone(&claims),
                    player,
                    stone+1,
                    player_count,
                    0,
                    tmp_streak_done,
                    other_count,
                    0
                );

                if other_count > 0 && other_streak < 2 {
                    claims.borrow_mut().claim(player.get_other(), Stone::build(stone as u8));

                    three_streak_rec(
                        Rc::clone(&claims),
                        player,
                        stone+1,
                        player_count,
                        0,
                        tmp_streak_done,
                        other_count-1,
                        other_streak+1
                    );  

                    claims.borrow_mut().unclaim(Stone::build(stone as u8));
                }

                if (!tmp_streak_done) && player_count == 3 - player_streak {
                    for i in 0..(3-player_streak) as usize {
                    claims.borrow_mut().claim(player, Stone::build((stone+i) as u8));
                    }

                    three_streak_rec(
                        Rc::clone(&claims),
                        player,
                        stone+3,
                        0,
                        3,
                        true,
                        other_count,
                        0
                    );  

                    for i in 0..(3-player_streak) as usize {
                        claims.borrow_mut().unclaim(Stone::build((stone+i) as u8));
                    }
                } else if player_count > 0 {
                    claims.borrow_mut().claim(player, Stone::build(stone as u8));

                    three_streak_rec(
                        Rc::clone(&claims),
                        player,
                        stone+1,
                        player_count-1,
                        player_streak+1,
                        tmp_streak_done,
                        other_count,
                        0
                    );

                    claims.borrow_mut().unclaim(Stone::build(stone as u8));
                }
            }
        }
    }

    #[test]
    fn test_three_streak() {
        three_streak_rec(
            Rc::new(RefCell::new(Claims::new())),
            Player::build(0),
            0,
            5,
            0,
            false,
            4,
            0
        );

        three_streak_rec(
            Rc::new(RefCell::new(Claims::new())),
            Player::build(1),
            0,
            5,
            0,
            false,
            4,
            0
        );
    }

    fn not_terminal_rec(
        claims: Rc<RefCell<Claims>>,
        stone: usize,
        p1_streak: u8,
        p1_count: u8,
        p2_streak: u8,
        p2_count: u8) 
    {
        if stone == NUM_OF_STONES as usize {
            let mut board = CardsOnBoard::new();
            
            board.claims = claims.borrow().clone();

            let res = board.terminal_state();

            assert!(res.is_none());  
        } else {
            not_terminal_rec(
                Rc::clone(&claims),
                stone+1,
                0,
                p1_count,
                0,
                p2_count
            );

            if p1_streak < 2 && p1_streak > 0 {
                claims.borrow_mut().claim(Player::build(0), Stone::build(stone as u8));

                not_terminal_rec(
                    Rc::clone(&claims),
                    stone+1,
                    p1_streak+1,
                    p1_count-1,
                    0,
                    p2_count
                );

                claims.borrow_mut().unclaim(Stone::build(stone as u8));
            }

            if p2_streak < 2 && p2_streak > 0 {
                claims.borrow_mut().claim(Player::build(1), Stone::build(stone as u8));

                not_terminal_rec(
                    Rc::clone(&claims),
                    stone+1,
                    0,
                    p1_count,
                    p2_streak+1,
                    p2_count-1
                );

                claims.borrow_mut().unclaim(Stone::build(stone as u8));
            }
        }
    }

    #[test]
    fn test_not_terminal() {
        not_terminal_rec(
            Rc::new(RefCell::new(Claims::new())),
            0,
            0,
            4,
            0,
            4);
    }
}

#[cfg(test)]
mod test_display {
    use super::*;

    use std::{cell::RefCell, rc::Rc};

    fn string_stones_for_rec(claims: Rc<RefCell<Claims>>, compare_to: Rc<RefCell<String>>, index: usize, player: Option<Player>) {
        if index == NUM_OF_STONES as usize {
            let mut board = CardsOnBoard::new();

            board.claims = claims.borrow().clone();

            assert_eq!(board.string_stones_for(player), compare_to.borrow().clone(), "{:#?}", board.claims);
        } else {

            assert_eq!(SPACE.len(), 3);

            let all_players = vec![None, Some(Player::build(0)), Some(Player::build(1))];

            for p in all_players {
                compare_to.borrow_mut().push_str(if player == p { STONE_STR } else { SPACE });
                compare_to.borrow_mut().push_str(SPACE);

                if let Some(p) = p {
                    claims.borrow_mut().claim(p, Stone::build(index as u8));    
                }

                string_stones_for_rec(Rc::clone(&claims), Rc::clone(&compare_to), index+1, player);


                for _ in 0..6 {
                    compare_to.borrow_mut().pop();
                }
                claims.borrow_mut().unclaim(Stone::build(index as u8));
            }
        }
    }

    #[test]
    fn test_string_stones_for() {
        string_stones_for_rec(
            Rc::new(RefCell::new(Claims::new())),
            Rc::new(RefCell::new(String::new())),
            0,
            None,
        );

        string_stones_for_rec(
            Rc::new(RefCell::new(Claims::new())),
            Rc::new(RefCell::new(String::new())),
            0,
            Some(Player::build(0)),
        );

        string_stones_for_rec(
            Rc::new(RefCell::new(Claims::new())),
            Rc::new(RefCell::new(String::new())),
            0,
            Some(Player::build(1)),
        );
    }


    #[test]
    fn test_string_nth_card_for() {
        let mut board = CardsOnBoard::new();

        for num in (1..NUM_OF_NUMS+1).filter(|n| *n != 4) {
            board.place_card(Player::build(0), Stone::build(num-1), Card::build(num, 1));
        }

        let expected_string = format!("Pu1{SPACE}Pu2{SPACE}Pu3{SPACE}{SPACE}{SPACE}Pu5{SPACE}Pu6{SPACE}Pu7{SPACE}Pu8{SPACE}Pu9{SPACE}");
        assert_eq!(board.string_nth_card_for(Player::build(0), 0), expected_string);
        assert_eq!(board.string_nth_card_for(Player::build(1), 0), SPACE.repeat(2*NUM_OF_STONES as usize));
        assert_eq!(board.string_nth_card_for(Player::build(0), 1), SPACE.repeat(2*NUM_OF_STONES as usize));
        assert_eq!(board.string_nth_card_for(Player::build(0), 2), SPACE.repeat(2*NUM_OF_STONES as usize));

        for num in (1..NUM_OF_NUMS+1).filter(|n| *n != 4) {
            board.place_card(Player::build(1), Stone::build(num-1), Card::build(num, 1));
        }

        assert_eq!(board.string_nth_card_for(Player::build(1), 0), expected_string);
    }
}

pub struct CardsOnBoard {
    advantage: Advantage,
    cards: Vec<Vec<StoneCards>>, // Maybe use array\slices somehow.
    present_cards: PresentCards,
    claims: Claims,
}

impl CardsOnBoard {
    pub fn new() -> Self {
        let mut cards: Vec<Vec<StoneCards>> = Vec::with_capacity(NUM_OF_PLAYERS as usize);

        for player in 0..NUM_OF_PLAYERS {
            cards.push(Vec::with_capacity(NUM_OF_STONES as usize));

            for _ in 0..NUM_OF_STONES {
                cards[player as usize].push(StoneCards::new());
            }
        }

        CardsOnBoard {
            advantage: Advantage::new(),
            cards: cards,
            present_cards: PresentCards::new(),
            claims: Claims::new(),
        }
    }

    pub fn place_card(&mut self, player: Player, stone: Stone, card: Card) {
        let stone_cards = &mut self.cards[player.get_index()][stone.get_index()];

        self.present_cards.add_card(&card);

        stone_cards.push(card);

        if stone_cards.is_full() {
            if let None = self.advantage[stone.get_index()] {
                self.advantage.set(player, stone);
            }
        }
    }

    fn proto_is_legal_claim(&self, player: Player, stone: Stone) -> bool {
        let cards_of_player = &self.cards[player.get_index()][stone.get_index()];

        if !cards_of_player.is_full() { return false; }

        let cards_of_other = &self.cards[player.get_other().get_index()][stone.get_index()];

        let cards_of_player_strength = cards_of_player.strength();

        // Rest of the code assumes the following:
        assert!(STONE_CARDS_LIMIT == 3);

        let all_nonpresent_cards = || {
            (1..=NUM_OF_NUMS)
            .cartesian_product(1..=NUM_OF_COLORS)
            .map(|(num,color)| Card::build(num, color))
            .filter(|card| !self.present_cards.is_present(card))
        };

        match cards_of_other.len() {
            3 => {
                if self.advantage[stone.get_index()].is_some() && self.advantage[stone.get_index()].unwrap() == player {
                    cards_of_player_strength >= cards_of_other.strength()
                } else {
                    cards_of_player_strength > cards_of_other.strength()
                }
            }
            2 => {
                for card in all_nonpresent_cards() {
                    let mut tmp = cards_of_other.clone();
                    tmp.push(card);

                    if tmp.strength() > cards_of_player_strength {
                        return false;
                    }
                }

                true
            }
            1 => {
                for card1 in all_nonpresent_cards() {
                    for card2 in all_nonpresent_cards().filter(|card| *card != card1) {
                        let mut tmp = cards_of_other.clone();
                        tmp.push(card1.clone());
                        tmp.push(card2);

                        if tmp.strength() > cards_of_player_strength {
                            return false;
                        }   
                    }
                }

                true
            }
            0 => {
                for card1 in all_nonpresent_cards() {
                    for card2 in all_nonpresent_cards().filter(|card| *card != card1) {
                        for card3 in all_nonpresent_cards().filter(|card| *card != card2 && *card != card1) {
                            let mut tmp = StoneCards::new();
                            tmp.push(card1.clone());
                            tmp.push(card2.clone());
                            tmp.push(card3);

                            if tmp.strength() > cards_of_player_strength {
                                return false;
                            }   
                        }
                    }
                }

                true
            }

            _ => panic!("Unexcpected num of cards.")
        }
    }

    pub fn claim(&mut self, player: Player, stone: Stone) -> bool{
        if self.claims.who_claimed(stone).is_none() && self.proto_is_legal_claim(player, stone) {
            self.claims.claim(player, stone);

            true
        } else {
            false
        }
    }

    pub fn terminal_state(&self) -> Option<Player> {
        // Current implementation assumes the following
        assert!(NUM_OF_STONES == 9);

        let mut count: [u8; 3] = [0; 3];
        let mut neighboring_stones_count = 0;
        let mut neighboring_stones_player: Option<Player> = None;


        for player in self.claims.iter() {
            if neighboring_stones_count == 3 && neighboring_stones_player.is_some() {break;}

            if *player == neighboring_stones_player {
                neighboring_stones_count += 1;
            } else {
                neighboring_stones_count = 1;
                neighboring_stones_player = *player;
            }

            match *player {
                Some(p) => count[p.get_player() as usize] += 1,
                _ => count[0] += 1,
            }
        }

        if neighboring_stones_count == 3 && neighboring_stones_player.is_some() { return neighboring_stones_player; }

        if count[1] >= 5 { return Some(Player::build(0)); }
        if count[2] >= 5 { return Some(Player::build(1)); }

        None
    }

    pub fn available_stones(&self) -> Vec<Stone> {
        self.claims
            .iter()
            .enumerate()
            .filter(|item| item.1.is_none())
            .map(|item| Stone::build(item.0 as u8))
            .collect()
    }

    pub fn available_stones_for(&self, player: Player) -> Vec<Stone> {
        self.claims
            .iter()
            .enumerate()
            .filter(|item| item.1.is_none() && !self.cards[player.get_index()][item.0].is_full())
            .map(|item| Stone::build(item.0 as u8))
            .collect()
    }

    pub fn any_available_stones_for(&self, player: Player) -> bool {
        self.claims
            .iter()
            .enumerate()
            .filter(|item| item.1.is_none() && !self.cards[player.get_index()][item.0].is_full())
            .next()
            .is_some()
    }
}

impl CardsOnBoard {
    // Helper methods for display trait

    fn string_stones_for(&self, player: Option<Player>) -> String {
        let mut output = String::new();

        for stone in self.claims.iter() {
            if *stone == player {
                output.push_str(STONE_STR);
            } else {
                output.push_str(SPACE);
            }

            output.push_str(SPACE);
        }

        output
    }

    fn string_nth_card_for(&self, player: Player, index: usize) -> String {
        if index >= STONE_CARDS_LIMIT as usize { panic!("Index out of bounds"); }

        self.cards[player.get_index()]
            .iter()
            .map(|cards| {
                let mut s = String::new();

                if cards.len() > index {
                    let tmp = cards[index].to_string();

                    s.push_str(&tmp);
                } else {
                    s.push_str(SPACE);
                }

                s.push_str(SPACE);

                s
            })
            .collect::<String>()

    }
}

impl fmt::Display for CardsOnBoard {
    

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // assert!(Card::build(1,1).to_string().len() == SPACE.len())

        let mut board_string = String::new();

        // Player 2's claimed stones
        board_string.push_str(&self.string_stones_for(Some(Player::build(1))));
        board_string.push_str("\n");

        // Player 2's cards
        for i in (0..STONE_CARDS_LIMIT as usize).rev() {
            board_string.push_str(&self.string_nth_card_for(Player::build(1), i));
            board_string.push_str("\n");
        }

        // Unclaimed stones
        board_string.push_str(&self.string_stones_for(None));
        board_string.push_str("\n");

        // Player 1's cards
        for i in 0..STONE_CARDS_LIMIT as usize {
            board_string.push_str(&self.string_nth_card_for(Player::build(0), i));
            board_string.push_str("\n");
        }

        // Player 1's claimed stones
        board_string.push_str(&self.string_stones_for(Some(Player::build(0))));
        board_string.push_str("\n");

        write!(f, "{}", board_string)
    }
}

struct PresentCards {
    cards: [[bool; NUM_OF_COLORS as usize]; NUM_OF_NUMS as usize],
}
impl PresentCards {
    fn new() -> Self {
        PresentCards { cards: [[false; NUM_OF_COLORS as usize]; NUM_OF_NUMS as usize] }
    }

    fn add_card(&mut self, card: &Card) {
        self.cards[card.num_index() as usize][card.color_index() as usize] = true;
    }

    fn is_present(&self, card: &Card) -> bool {
        self.cards[card.num_index() as usize][card.color_index() as usize] == true
    }
}

