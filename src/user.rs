use crate::board::Board;
use crate::config::SHIPS;
use crate::structures::{Ship, PlacedShip, UserType, Direction};
use std::io;

pub struct User {
    pub board: Board,
    pub name: String,
    pub placed_ships: Vec<PlacedShip>,
    pub user_type: UserType,
}

impl User {
    pub fn new() -> User {
        User {
            board: Board::new(),
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

        return user_name;
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

    pub fn place_ship(&mut self, ship: &'static Ship) {
        let mut direction: Direction;
        let mut position: [usize; 2] = [0, 0];

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

            let result = self.board.place_ship(position[0], position[1], ship.length, direction);
            match result {
                Ok(_) => {
                    println!("your current field: ");
                    println!("{}", self.board);
                    self.placed_ships.push(PlacedShip {
                        ship,
                        y: position[0],
                        x: position[1],
                    });
                    break;
                }
                Err(e) => {
                    println!("    ->  {}", e);
                    println!("your current field: ");
                    println!("{}", self.board);
                    continue;
                }
            }
        }
    }
}
