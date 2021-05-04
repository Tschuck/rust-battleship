use crate::{structures::UserType, user::User};
use std::{io};

pub struct Game {
    users: Vec<User>,
}

impl Game {
    pub fn new() -> Game {
        let mut game: Game = Game {
            users: Vec::new(),
        };

        for i in 0..2 {
            game.print_header();
            println!("Welcome to the RUST-BATTLESHIP game. Before we start, please introduce yourself:\n");
            println!("----------------------- User {} -----------------------", i + 1);
            let mut user = User::new();
            if user.user_type == UserType::USER {
                user.place_ships();
            } else {
                user.place_ships_random();
            }
            game.users.push(user);
        }

        return game;
    }

    fn print_header(&self) {
        print!("\x1B[2J\x1B[1;1H");
        println!("|-------------------|");
        println!("|  RUST-BATTLESHIP  |");
        println!("|-------------------|\n");
    }

    pub fn start(&mut self) {
        // ui => user_index
        // use indexes only to prevent multiple borrowing of the users array
        let mut active_ui = 0;
        let mut target_ui = 1;

        loop {
            self.print_header();
            println!("Active User: {}", self.users[active_ui].name);
            self.users[active_ui].board.log_board(true);
            println!("\n\n");
            println!("Target User: {}", self.users[target_ui].name);
            self.users[target_ui].board.log_board(false);
            println!("\n\n----------------------\n\n");

            println!("Shoot!");

            if self.users[active_ui].user_type == UserType::USER {
                loop {
                    let position = self.users[active_ui].get_x_y_position();
                    let shoot_result = self.users[target_ui].shoot(position[0], position[1]);
                    match shoot_result {
                        Ok(_) => {
                            break;
                        }
                        Err(e) => {
                            println!("    ->  {} - Try again!\n\n", e);
                        }
                    }
                }
            } else {
                self.users[target_ui].shoot_random();
            }

            if self.users[target_ui].lost {
                println!("{} WON! JEEEHAAA 🥳", self.users[active_ui].name);
                break;
            }

            println!("Press any key to continue!");
            let mut pressed_key = String::new();
            io::stdin()
                .read_line(&mut pressed_key)
                .expect("Failed to readline");

            if self.users[target_ui].lost {
                break;
            }

            active_ui = if active_ui == 0 { 1 } else { 0 };
            target_ui = if active_ui == 0 { 1 } else { 0 };
        }
    }
}

