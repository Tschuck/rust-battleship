use crate::user::User;
use std::io;

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
            user.place_ships_random();
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

        while !self.users[active_ui].lost {
            self.print_header();
            println!("Active User: {}", self.users[active_ui].name);
            self.users[active_ui].board.log_board(true);
            println!("\n\n");
            println!("Target User: {}", self.users[target_ui].name);
            self.users[target_ui].board.log_board(false);
            println!("\n\n----------------------\n\n");

            println!("Shoot!");

            loop {
                let position = self.users[active_ui].get_x_y_position();
                let mut shootResult = self.users[target_ui].shoot(position[0], position[1]);
                match shootResult {
                    Ok(_) => {
                        break;
                    }
                    Err(e) => {
                        println!("    ->  {} - Try again!\n\n", e);
                    }
                }
            }

            println!("Press any key to continue!");
            let mut pressedKey = String::new();
            io::stdin()
                .read_line(&mut pressedKey)
                .expect("Failed to readline");

            active_ui = if active_ui == 0 { 1 } else { 0 };
            target_ui = if active_ui == 0 { 1 } else { 0 };
        }
    }

    pub fn has_lost() -> bool {
        return false;
    }
}

