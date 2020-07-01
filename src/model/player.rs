use std::fmt::{Display, Formatter};
use std::fmt;

pub struct Player {
    pub name: &'static str
}

impl Player {
    pub fn new(name: &'static str) -> Player {
        Player { name }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Player with name {}", self.name)
    }
}