use std::ptr::null;

struct Point {
    x: u8,
    y: u8,
}

enum Direction {
    Vertical,
    Horizontal,
}

struct Ship {
    name: String,
    size: u8,
    hits: Vec<bool>,
    position: Option<Point>,
}

impl Ship {
    fn is_sunk(&self) -> bool {
        self.hits.iter().all(|h| *h)
    }
}

/**
 * 5 ships:
 *
 * Carrier: 5 holes
 * Battleship: 4 holes
 * Cruiser: 3 holes
 * Submarine: 3 holes
 * Destroyer: 2 holes
 */
fn default_ships() -> Vec<Ship> {
    let mut ships = Vec::new();

    ships.push(Ship {
        name: "Carrier".into(),
        size: 5,
        hits: vec![false; 5],
        position: None,
    });

    ships.push(Ship {
        name: "Battleship".into(),
        size: 4,
        hits: vec![false; 4],
        position: None,
    });

    ships.push(Ship {
        name: "Cruiser".into(),
        size: 3,
        hits: vec![false; 3],
        position: None,
    });

    ships.push(Ship {
        name: "Submarine".into(),
        size: 3,
        hits: vec![false; 3],
        position: None,
    });

    ships.push(Ship {
        name: "Destroyer".into(),
        size: 2,
        hits: vec![false; 2],
        position: None,
    });

    ships
}

struct Field {
    spaces: [[bool; 10]; 10],
    ships: Vec<Ship>,
}

impl Field {
    fn new() -> Self {
        let spaces = [[false; 10]; 10];

        // let ships: Vec<Ship> =
        return Field {
            spaces,
            ships: default_ships(),
        };
    }

    fn shoot(&self, x: usize, y: usize) -> bool {
        self.spaces[x][y]
    }

    fn place_ship(name: String, position: Point) {}
}

struct User {
    name: String,
    field: Field,
}

impl User { 
    fn new(name: String) -> Self {
        Self { name, field: Field::new() }
    }
}

fn main() {
    let u1 = User::new("Cam".into());
    let u2 = User::new("Maya".into());
}

// Generate game board
// Create new matrix n*n
// Get vec of pieces
// Choose if piece is vertical or horizontal
//

// Game board: 10x10
// Ships may be horizontal or vertical
