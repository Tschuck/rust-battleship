use std::error::Error;

use crate::config::{GRID_COUNT};
use crate::structures::{Direction, FieldStatus};

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

    pub fn log_board(&self, show_ships: bool) {
        let mut header = String::new();
        header.push_str(" |      | ");

        for i in 0..GRID_COUNT {
            if i < 10 {
                header.push_str(&format!("  {} | ", i));
            } else {
                header.push_str(&format!(" {} | ", i));
            }
        }

        println!("{}", header);

        for y in 0..self.fields.len() {
            let mut line_log = String::new();
            if y < 10 {
                line_log.push_str(&format!(" |   {}  |", y));
            } else {
                line_log.push_str(&format!(" |  {}  |", y));
            }

            for x in 0..self.fields[y].len() {
                line_log.push_str("  ");

                if self.fields[y][x] == FieldStatus::EMPTY {
                    line_log.push_str(&format!("❓"));
                } else if self.fields[y][x] == FieldStatus::FAIL {
                    line_log.push_str(&format!("❌"));
                } else if self.fields[y][x] == FieldStatus::HIT {
                    line_log.push_str(&format!("💥"));
                } else if self.fields[y][x] == FieldStatus::SHIP {
                    if show_ships {
                        line_log.push_str(&format!("🚢"));
                    } else {
                        line_log.push_str(&format!("❓"));
                    }
                }

                line_log.push_str(&format!(" |"));
            }

            println!("{}", line_log);
        }
    }
}
