use std::fmt;
use std::error::Error;

use crate::config::{GRID_COUNT};
use crate::structures::{Direction, FieldStatus};

#[derive(Debug)]
pub struct Board {
    /// field of the user
    /// 0 => nothing set
    /// 1 =>
    pub fields: [[FieldStatus; GRID_COUNT]; GRID_COUNT],
}

impl Board {
    pub fn new() -> Board {
        Board {
            fields: [[FieldStatus::EMPTY; GRID_COUNT]; GRID_COUNT],
        }
    }

    pub fn place_ship(
        &mut self,
        y: usize,
        x: usize,
        ship_length: usize,
        direction: Direction,
    ) -> Result<(), Box<dyn Error>> {
        if y >= self.fields.len() || x >= self.fields[y].len() {
            return Err(Box::from("out of field bounds"));
        }

        for i in 0..ship_length {
            match direction {
                Direction::VERTICAL => {
                    if y + i >= self.fields.len() {
                        return Err(Box::from("out of field bounds"));
                    }
                    if self.fields[y + i][x] != FieldStatus::EMPTY {
                        return Err(Box::from("field already placed with another ship"));
                    }
                }
                Direction::HORIZONTAL => {
                    if x + i >= self.fields[y].len() {
                        return Err(Box::from("out of field bounds"));
                    }
                    if self.fields[y][x + i] != FieldStatus::EMPTY {
                        return Err(Box::from("field already placed with another ship"));
                    }
                }
            }
        }

        for i in 0..ship_length {
            match direction {
                Direction::VERTICAL => {
                    self.fields[y + i][x] = FieldStatus::SHIP;
                }
                Direction::HORIZONTAL => {
                    self.fields[y][x + i] = FieldStatus::SHIP;
                }
            }
        }

        Ok(())
    }
}

impl fmt::Display for Board {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.fields.len() {
            println!("{:?}", self.fields[y]);
        }

        Ok(())
    }
}
