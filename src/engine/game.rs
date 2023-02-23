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

impl Game {
    pub fn new(u1: String, u2: String) -> Self {
        Self {
            u1: User::new(u1),
            u2: User::new(u2),
        }
    }
}
