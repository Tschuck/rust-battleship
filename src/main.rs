mod board;
mod structures;
mod config;

use structures::{Direction};
use board::Board;

fn main() {
    let mut boards = [Board::new(), Board::new()];

    boards[0].place_ship(0, 0, 4, Direction::HORIZONTAL);
    boards[0].place_ship(1, 3, 4, Direction::VERTICAL);

    println!("{}", boards[0]);
    println!("{}", boards[1]);
}
