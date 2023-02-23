#![allow(dead_code)]
#![allow(unused_variables)]

use std::io::stdin;

mod engine;
use engine::game::Game;

use crate::engine::types::Point;

fn main() {
    println!("Welcome to Battleship. Creating game...");

    println!("Please enter the name of the first player: ");

    let mut u1_name = String::new();
    stdin().read_line(&mut u1_name).unwrap();

    println!("Please enter the name of the second player: ");

    let mut u2_name = String::new();
    stdin().read_line(&mut u2_name).unwrap();

    let mut game = Game::new(u1_name.trim().into(), u2_name.trim().into());

    loop {
        if game.are_all_ships_placed() {
            break;
        }

        let active_user = game.get_active_user();

        println!("          {}'s turn to place ships", game.active_user_name);
        println!("{}", active_user.field.player_view());

        let ship = active_user.field.get_next_ship_to_place();
        if let Some(ship) = ship {
            let ship_at_point = active_user.field.get_ship_at_point(Point { x: 0, y: 0 });
            let ship_name = ship.name.clone();
            println!(
                "Placing ship: {} which takes up {} spaces",
                ship_name, ship.size
            );
            let position = accept_ship_placement_move();
            println!("You entered: {:?}", position);

            let result = game
                .get_active_user_mut()
                .field
                .place_ship(ship_name, position);

            if let Err(e) = result {
                println!("Error: Specified ship does not fit at that point or there is already a ship at that point");
                continue;
            }
            game.start_next_turn();
        }
    }

    loop {
        let active_user = game.get_active_user();

        if active_user.field.are_all_ships_sunk() {
            println!("{} has won!", game.non_active_user_name);
            break;
        }

        println!("          {}'s turn to attack", game.active_user_name);
        println!("          Your Field");
        println!("{}", active_user.field.player_view());
        println!("          Opponent's Field");
        println!("{}", active_user.field.enemy_view());

        let position = accept_attack_move();
        println!("You entered: {:?}", position);

        let result = game.get_active_user_mut().field.shoot(&position);

        if let Err(e) = result {
            println!("Error: You have already attacked that point");
            continue;
        } else if let Ok(i) = result {
            println!("You hit a you enemies {}!", i.name);
        } else {
            println!("You missed!");
        }

        game.start_next_turn();
    }
}

fn accept_ship_placement_move() -> engine::types::Position {
    println!("Please enter a position in the form of \"[A-J][1-10]\": ");

    let point: engine::types::Point;
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let potential_point = engine::game::convert_input_to_point(&input);
        if let Ok(i) = potential_point {
            point = i;
            break;
        } else {
            println!("Invalid input. Please try again: ");
        }
    }

    println!(
        "Please enter a direction in the form of \"h\" for horizontal or \"v\" for vertical: "
    );

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

    engine::types::Position { point, direction }
}

fn accept_attack_move() -> engine::types::Point {
    println!("Please enter a position in the form of \"[A-J][1-10]\": ");

    let point: engine::types::Point;
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let potential_point = engine::game::convert_input_to_point(&input);
        if let Ok(i) = potential_point {
            point = i;
            break;
        } else {
            println!("Invalid input. Please try again: ");
        }
    }

    point
}
