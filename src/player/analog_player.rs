use std::{fmt, io};

use super::{Player, Hand, CardsOnBoard, PlayerTag, Stone};

pub struct AnalogPlayer;
impl Player for AnalogPlayer {
    fn choose_action(&self, hand: &Hand, board: &CardsOnBoard, player: PlayerTag) -> (usize, Stone) {
        println!("{}", *board);
        println!("{}", *hand);

        loop {
            println!("Choose a card and stone to play (zero-indexed, comma separated):");

            let mut buffer = String::new();

            io::stdin().read_line(&mut buffer).unwrap();
            buffer = String::from(buffer.trim_end());

            //println!("{buffer}");

            let chosen_action: Vec<&str> = buffer.split(',').collect();

            //println!("{:?}", chosen_action);

            match chosen_action.len() {
                2 => {
                    let actions_parsed = (chosen_action[0].parse::<usize>(), chosen_action[1].parse::<usize>());

                    match actions_parsed {
                        (Ok(card_index), Ok(stone_index)) => {
                            if card_index >= hand.len() {
                                println!("Invalid card index: {card_index}");
                            } else if let Ok(stone) = Stone::try_build(stone_index as u8) {
                                let available_stones = board.available_stones_for(player);

                                if available_stones.contains(&stone) {
                                    return (card_index, stone);
                                } else {
                                    println!("Stone is not available");
                                }
                            } else {
                                println!("Invalid stone index.")
                            }
                        }

                        _ => println!("Invalid input."),
                    }
                }

                _ => println!("Invalid input."),
            }
        }
    }
}