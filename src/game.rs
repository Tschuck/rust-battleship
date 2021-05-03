use crate::user::User;
use crate::structures::Direction;

fn print_header() {
    print!("\x1B[2J\x1B[1;1H");
    println!("|-------------------|");
    println!("|  RUST-BATTLESHIP  |");
    println!("|-------------------|\n");
    println!("Welcome to the RUST-BATTLESHIP game. Before we start, please introduce yourself:\n");
}

pub fn start_game() {
    let mut users: Vec<User> = Vec::new();
    for i in 0..1 {
        print_header();
        println!("----------------------- User {} -----------------------", i + 1);
        let mut user = User::new();
        user.place_ships();
        users.push(user);
    }

    // users[0].board.place_ship(0, 0, 4, Direction::HORIZONTAL);
    // users[0].board.place_ship(1, 3, 4, Direction::VERTICAL);

    // println!("User {}", users[0].name);
    // println!("{}", users[0].board);
    // println!("User {}", users[1].name);
    // println!("{}", users[1].board);
}

