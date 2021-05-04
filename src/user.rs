use std::error::Error;
use crate::board::Board;
use crate::config::{GRID_COUNT,SHIPS};
use crate::structures::{Ship, PlacedShip, UserType, Direction, FieldStatus};
use std::io;
use rand::Rng;

pub struct User {
    pub board: Board,
    pub name: String,
    pub placed_ships: Vec<PlacedShip>,
    pub user_type: UserType,
    pub lost: bool,
}

impl User {
    pub fn new() -> User {
        User {
            board: Board::new(),
            lost: false,
            name: User::read_name(),
            placed_ships: Vec::new(),
            user_type: User::read_user_type(),
        }
    }

    pub fn read_name() -> String {
        let mut user_name = String::new();

        println!(" - Whats your name?");
        loop {
            io::stdin()
                .read_line(&mut user_name)
                .expect("    -> Failed to read name for new user");

            if user_name.trim().len() == 0 {
                println!("    -> Please enter a user name!");
            } else {
                break;
            }
        }

        return user_name.trim().to_string();
    }

    pub fn read_user_type() -> UserType {
        let mut user_type: UserType = UserType::USER;

        println!("\n - How should the user act? (0 = manual, 1 = bot)");
        loop {
            let mut type_input = String::new();
            io::stdin()
                .read_line(&mut type_input)
                .expect("    -> Failed to readline");
            let type_input: u32 = match type_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("    -> please insert a number.");
                    continue;
                }
            };

            if type_input == 0 {
                user_type = UserType::USER;
                break;
            } else if type_input == 1{
                user_type = UserType::BOT;
                break;
            } else {
                println!("    -> please insert 0 = manual user / 1 = bot user.");
            }
        }

        return user_type;
    }

    pub fn place_ships(&mut self) {
        println!("\n - Please place your ships");
        for i in 0..SHIPS.len() {
            self.place_ship(&SHIPS[i]);
        }
    }

    pub fn place_ships_random(&mut self) {
        println!("\n - Placed ships randomly");

        for i in 0..SHIPS.len() {
            self.place_ship_random(&SHIPS[i]);
        }

        println!("your current field: ");
        self.board.log_board(true);
    }

    pub fn get_x_y_position(&self) -> [usize; 2] {
        let mut position: [usize; 2] = [0, 0];

        for i in 0..2 {
            loop {
                if i == 0 {
                    println!("    ->  Please insert the y position");
                } else {
                    println!("    ->  Please insert the x position");
                }

                let mut position_input = String::new();
                io::stdin()
                    .read_line(&mut position_input)
                    .expect("Failed to readline");
                position[i] = match position_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("please insert a number: {}", position_input);
                        continue;
                    }
                };

                break;
            }
        }

        return position;
    }

    pub fn place_ship_random(&mut self, ship: &'static Ship) {
        loop {
            let y = rand::thread_rng().gen_range(0, GRID_COUNT);
            let x = rand::thread_rng().gen_range(0, GRID_COUNT);
            let direction = if rand::thread_rng().gen_range(0, 2) == 0 {
                Direction::HORIZONTAL
            } else {
                Direction::VERTICAL
            };

            let result = self.board.place_ship(y, x, ship.length, direction);
            match result {
                Ok(_) => {
                    self.placed_ships.push(PlacedShip {
                        ship: &ship,
                        y,
                        x,
                        hits: 0,
                    });
                    break;
                }
                Err(e) => {
                    continue;
                }
            }
        }
    }

    pub fn place_ship(&mut self, ship: &'static Ship) {
        let mut direction: Direction;

        println!("\n - {} ({} length)", ship.name, ship.length);

        loop {
            loop {
                println!("    ->  Please insert v or h the direction (v = vertical (|), h = horizontal (-))");

                let mut direction_input = String::new();
                io::stdin()
                    .read_line(&mut direction_input)
                    .expect("     -> Failed to readline");

                if direction_input.trim() == "v" {
                    direction = Direction::VERTICAL;
                    break;
                } else if direction_input.trim() == "h" {
                    direction = Direction::HORIZONTAL;
                    break;
                } else {
                    println!("     -> please insert v = vertical / h = horizontal");
                }
            }

            let position: [usize; 2] = self.get_x_y_position();

            let result = self.board.place_ship(position[0], position[1], ship.length, direction);
            match result {
                Ok(_) => {
                    println!("your current field: ");
                    self.board.log_board(true);
                    self.placed_ships.push(PlacedShip {
                        ship,
                        y: position[0],
                        x: position[1],
                        hits: 0,
                    });
                    break;
                }
                Err(e) => {
                    println!("    ->  {}", e);
                    println!("your current field: ");
                    self.board.log_board(true);
                    continue;
                }
            }
        }
    }

    pub fn shoot(&mut self, y: usize, x: usize) -> Result<(), Box<dyn Error>> {
        if y >= self.board.fields.len() || x >= self.board.fields[y].len() {
            return Err(Box::from("out of field bounds"));
        } else if self.board.fields[y][x] == FieldStatus::HIT || self.board.fields[y][x] == FieldStatus::FAIL {
            println!("You already shot at this position? (y = {}, x = {})", y, x);
            return Err(Box::from("You already shot at this position?"));
        } else if self.board.fields[y][x] == FieldStatus::EMPTY {
            println!("Awwww nothing... (y = {}, x = {})", y, x);
            self.board.fields[y][x] = FieldStatus::FAIL;
        } else if self.board.fields[y][x] == FieldStatus::SHIP {
            println!("You have hit a ship! (y = {}, x = {})", y, x);
            self.board.fields[y][x] = FieldStatus::HIT;
        } else {
            println!("dafuq");
        }

        self.calculateLost();

        return Ok(());
    }

    pub fn shoot_random(&mut self) {
        loop {
            let y = rand::thread_rng().gen_range(0, GRID_COUNT);
            let x = rand::thread_rng().gen_range(0, GRID_COUNT);

            let result = self.shoot(y, x);
            match result {
                Ok(_) => {
                    break;
                }
                Err(e) => {
                    continue;
                }
            }
        }

        self.calculateLost();
    }

    pub fn calculateLost(&mut self) {
        for y in 0..self.board.fields.len() {
            for x in 0..self.board.fields[y].len() {
                if self.board.fields[y][x] == FieldStatus::SHIP {
                    return;
                }
            }
        }

        self.lost = true;
    }
}
