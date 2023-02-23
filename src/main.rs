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

    let position = accept_ship_placement_move();
    println!("You entered: {:?}", position);

    let result = game.u1.field.place_ship("destroyer".into(), position);
    println!("Result: {:?}", result);

    println!("{}", game.u1.field.to_string());
}

fn accept_ship_placement_move() -> engine::types::Position {
    println!("Please enter a position in the form of \"[A-J][1-10]\": ");

    let point: engine::types::Point;
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let potential_point = engine::game::convert_input_to_point(&input);
        if potential_point.is_ok() {
            point = potential_point.unwrap();
            break;
        } else {
            println!("Invalid input. Please try again: ");
        }
    }

    println!("Please enter a direction in the form of \"h\" for horizontal or \"v\" for vertical: ");

    let direction: engine::types::Direction;
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let potential_direction = engine::game::convert_input_to_direction(&input);

        if potential_direction.is_ok() {
            direction = potential_direction.unwrap();
            break;
        } else {
            println!("Invalid direction. Please try again: ");
        }
    }

    engine::types::Position {
        point,
        direction
    }
}
