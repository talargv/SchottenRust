use schotten::Game;
use schotten::player::{random_player::RandomPlayer, analog_player::AnalogPlayer};

fn main() {
    let mut game = Game::new();

    println!("{:?}", game.play(AnalogPlayer, RandomPlayer));
    println!("{}", game);
}