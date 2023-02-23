#![allow(dead_code)]
#![allow(unused_variables)]

use std::io::stdin;

mod engine;
use engine::game::Game;

fn main() {
    println!("Welcome to Battleship. Creating game...");

    println!("Please enter the name of the first player: ");

    let mut u1_name = String::new();
    stdin().read_line(&mut u1_name).unwrap();

    println!("Please enter the name of the second player: ");

    let mut u2_name = String::new();
    stdin().read_line(&mut u2_name).unwrap();

    let mut game = Game::new(u1_name, u2_name);

    println!("{}", game.u1.field.to_string());

    println!("{:?}", game.u1.field.get_unplaced_ships());
}

fn accept_move() {
    println!("Please enter a move: ");
}
