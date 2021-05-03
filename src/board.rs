use std::fmt;

use crate::config::{GRID_COUNT, SHIPS, SHIP_COUNT};
use crate::structures::{Direction, FieldStatus, PlacedShip};

#[derive(Debug)]
pub struct Board<'a> {
    pub placed_ships: Vec<PlacedShip<'a>>,
    /// field of the user
    /// 0 => nothing set
    /// 1 =>
    pub fields: [[&'static FieldStatus; GRID_COUNT]; GRID_COUNT],
}

impl Board<'_> {
    pub fn new<'a>() -> Board<'a> {
        Board {
            placed_ships: Vec::new(),
            fields: [[&FieldStatus::EMPTY; GRID_COUNT]; GRID_COUNT],
        }
    }

    pub fn place_ship(&mut self, x: usize, y: usize, ship_length: usize, direction: Direction) {
        for i in 0..ship_length {
            match direction {
                Direction::HORIZONTAL => {
                    self.fields[x + i][y] = &FieldStatus::SHIP;
                }
                Direction::VERTICAL => {
                    self.fields[x][y + i] = &FieldStatus::SHIP;
                }
            }
        }
    }
}

impl fmt::Display for Board<'_> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        for x in 1..self.fields.len() {
            println!("{:?}", self.fields[x]);
        }

        Ok(())
    }
}
