use crate::engine::field;

pub struct User {
    pub name: String,
    pub field: field::Field,
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            name,
            field: field::Field::new(),
        }
    }
}

pub struct Game {
    pub u1: User,
    pub u2: User,
}

pub fn create_game(u1_name: String, u2_name: String) -> Game {
    Game {
        u1: User::new(u1_name),
        u2: User::new(u2_name),
    }
}
