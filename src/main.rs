mod board;
mod config;
mod game;
mod structures;
mod user;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.start();
}
