use std::fmt;
use std::fmt::{Formatter, Display};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Color {
    White,
    Black
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Value {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Piece {
    pub color: Color,
    pub value: Value
}

impl Piece {
    pub fn new(color: Color, value: Value) -> Piece {
        Piece { color, value }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.color {
            Color::White => {
                match &self.value {
                    Value::King    => write!(f, "♔"),
                    Value::Queen   => write!(f, "♕"),
                    Value::Rook    => write!(f, "♖"),
                    Value::Bishop  => write!(f, "♗"),
                    Value::Knight  => write!(f, "♘"),
                    Value::Pawn    => write!(f, "♙")
                }
            },
            Color::Black => {
                match &self.value {
                    Value::King    => write!(f, "♚"),
                    Value::Queen   => write!(f, "♛"),
                    Value::Rook    => write!(f, "♜"),
                    Value::Bishop  => write!(f, "♝"),
                    Value::Knight  => write!(f, "♞"),
                    Value::Pawn    => write!(f, "♟︎")
                }
            }
        }
    }
}