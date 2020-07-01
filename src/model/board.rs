use crate::model::piece;
use crate::model::piece::{Value, Piece};
use std::ops::{Add, Sub};
use std::fmt::{Formatter, Display};
use std::fmt;

#[derive(Debug, Clone)]
struct InvalidMoveError;

const DIM: usize = 8;

pub struct Board {
    fields: [[Option<piece::Piece>; DIM]; DIM]
}

impl Board {
    pub fn new() -> Board {
        Board {
            fields: [[None; DIM]; DIM]
        }
    }

    pub fn is_valid_move(&self, row_from: usize, column_from: usize, row_to: usize, column_to: usize) -> bool {
        if (row_from, column_from) == (row_to, column_to) {
            return false;
        }

        if row_to > DIM || column_to > DIM {
            return false;
        }

        let piece = self.get_field(row_from, column_from);
        if piece.is_none() {
            return false;
        }

        let piece = piece.unwrap();
        let target = self.get_field(row_to, column_to);
        if let Some(target) = target {
            // Cannot move to a field with a piece of the same color
            if target.color == piece.color {
                return false;
            }
        }

        let delta_row = abs_diff(row_from, row_to);
        let delta_column = abs_diff(column_from, column_to);

        match piece.value {
            Value::King => {
                delta_row <= 1 && delta_column <= 1
            },
            Value::Queen => {
                if delta_row == 0 {
                    let (min, max) = if column_to > column_from { (column_from, column_to) } else { (column_to, column_from) };
                    for i in min + 1..max {
                        if self.is_occupied(row_from, i) {
                            return false;
                        }
                    }
                } else if delta_column == 0 {
                    let (min, max) = if row_to > row_from { (row_from, row_to) } else { (row_to, row_from) };
                    for i in min + 1..max {
                        if self.is_occupied(i, column_from) {
                            return false;
                        }
                    }
                } else if delta_row == delta_column {
                    let (mut x, mut y) = (row_from, column_from);

                    while (x, y) != (row_to, column_to) {
                        x = if row_to > row_from { x.add(1) } else { x.sub(1) };
                        y = if column_to > column_from { y.add(1) } else { y.sub(1) };
                        if self.is_occupied(x, y) {
                            return false;
                        }
                    }
                } else {
                    return false;
                }
                true
            },
            Value::Rook => {
                if delta_row == 0 {
                    let (min, max) = if column_to > column_from { (column_from, column_to) } else { (column_to, column_from) };
                    for i in min + 1..max {
                        if self.is_occupied(row_from, i) {
                            return false;
                        }
                    }
                } else if delta_column == 0 {
                    let (min, max) = if row_to > row_from { (row_from, row_to) } else { (row_to, row_from) };
                    for i in min + 1..max {
                        if self.is_occupied(i, column_from) {
                            return false;
                        }
                    }
                }  else {
                    return false;
                }
                true
            },
            Value::Bishop => {
                if delta_row == 0 || delta_column == 0 {
                    return false;
                }

                if delta_row != delta_column {
                    return false;
                }

                let (mut x, mut y) = (row_from, column_from);

                while (x, y) != (row_to, column_to) {
                    x = if row_to > row_from { x.add(1) } else { x.sub(1) };
                    y = if column_to > column_from { y.add(1) } else { y.sub(1) };
                    if self.is_occupied(x, y) {
                        return false;
                    }
                }
                true
            },
            Value::Knight => {
                (delta_row == 1 && delta_column == 2) ||
                    (delta_row == 2 && delta_column == 1)
            },
            Value::Pawn => {
                // Check if the pawn moves diagonally
                if (delta_row, delta_column) == (1, 1) {
                    return self.is_occupied(row_to, column_to);
                }

                // Check if the pawn moves legally forward
                let allowed_forward = if row_from == 0 { 2 } else { 1 };
                if column_from != column_to || delta_row > allowed_forward {
                    return false;
                }

                for i in row_from + 1..row_to + 1 {
                    if self.is_occupied(i, column_from) {
                        return false;
                    }
                }
                return true;
            }
        }
    }

    pub fn move_piece(&mut self, row_from: usize, column_from: usize, row_to: usize, column_to: usize) -> Result<Option<Piece>, InvalidMoveError> {
        if !self.is_valid_move(row_from, column_from, row_to, column_to) {
            return Err(InvalidMoveError);
        }

        let piece = self.get_field(row_from, column_from);
        let removed_piece = self.get_field(row_to, column_to);

        self.set_field(row_to, column_to, piece);
        self.set_field(row_from, column_from, None);
        Ok(removed_piece)
    }

    pub fn get_field(&self, row: usize, column: usize) -> Option<Piece> {
        self.fields[row][column]
    }

    pub fn set_field(&mut self, row: usize, column: usize, value: Option<Piece>) {
        self.fields[row][column] = value;
    }

    pub fn is_empty(&self, row: usize, column: usize) -> bool {
        self.get_field(row, column).is_none()
    }

    pub fn is_occupied(&self, row: usize, column: usize) -> bool {
        !self.is_empty(row, column)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in self.fields.iter() {
            for piece in row.iter() {
                if let Some(piece) = piece {
                    write!(f, "{}", piece)?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn abs_diff(x: usize, y: usize) -> usize {
    if x > y {
        x - y
    } else {
        y - x
    }
}

#[cfg(test)]
mod tests {
    use crate::model::board::Board;
    use crate::model::piece::{Piece, Color, Value};

    #[test]
    fn valid_move_king() {
        let mut board = Board::new();
        let (x, y) = (4, 4);
        board.set_field(x, y, Some(Piece::new(Color::Black, Value::King)));

        assert!(board.is_valid_move(x, y, x + 1, y + 1));
        assert!(board.is_valid_move(x, y, x, y + 1));
        assert!(board.is_valid_move(x, y, x + 1, y));
        assert!(!board.is_valid_move(x, y, x, y));
        assert!(!board.is_valid_move(x, y, x + 2, y));
        assert!(!board.is_valid_move(x, y, x, y + 2));
    }

    #[test]
    fn valid_move_queen() {
        let mut board = Board::new();
        let (x, y) = (4, 4);
        board.set_field(x, y, Some(Piece::new(Color::Black, Value::Queen)));

        // King test
        assert!(board.is_valid_move(x, y, x + 1, y + 1));
        assert!(board.is_valid_move(x, y, x, y + 1));
        assert!(board.is_valid_move(x, y, x + 1, y));

        // Rook test
        assert!(board.is_valid_move(x, y, x + 2, y));
        assert!(board.is_valid_move(x, y, x, y + 2));

        // Bishop test
        assert!(board.is_valid_move(x, y, x + 2, y + 2));
        assert!(board.is_valid_move(x, y, x - 2, y - 2));
        assert!(board.is_valid_move(x, y, x + 2, y - 2));
    }

    #[test]
    fn valid_move_rook() {
        let mut board = Board::new();
        let (x1, y1) = (3, 3);
        board.set_field(x1, y1, Some(Piece::new(Color::Black, Value::Rook)));

        let (x2, y2) = (6, 3);
        board.set_field(x2, y2, Some(Piece::new(Color::White, Value::Pawn)));

        assert!(board.is_valid_move(x1, y1, x1 + 2, y1));
        assert!(board.is_valid_move(x1, y1, x1, y1 + 2));
        assert!(board.is_valid_move(x1, y2, x2, y2));

        assert!(!board.is_valid_move(x1, y1, x1, y1));
        assert!(!board.is_valid_move(x1, y1, x1 + 2, y1 + 2));
        assert!(!board.is_valid_move(x1, y1, x2 + 1, y2));
    }

    #[test]
    fn valid_move_bishop() {
        let mut board = Board::new();
        let (x1, y1) = (2, 4);
        board.set_field(x1, y1, Some(Piece::new(Color::Black, Value::Bishop)));

        assert!(board.is_valid_move(x1, y1, x1 + 1, y1 + 1));
        assert!(board.is_valid_move(x1, y1, x1 + 2, y1 + 2));
        assert!(board.is_valid_move(x1, y1, x1 - 1, y1 - 1));
        assert!(board.is_valid_move(x1, y1, x1 + 2, y1 - 2));

        assert!(!board.is_valid_move(x1, y1, x1, y1));
        assert!(!board.is_valid_move(x1, y1, x1 + 1, y1));
    }

    #[test]
    fn valid_move_knight() {
        let mut board = Board::new();
        let (x, y) = (4, 4);
        board.set_field(x, y, Some(Piece::new(Color::Black, Value::Knight)));

        assert!(board.is_valid_move(x, y, x + 1, y + 2));
        assert!(board.is_valid_move(x, y, x - 2, y + 1));
        assert!(!board.is_valid_move(x, y, x, y));
        assert!(!board.is_valid_move(x, y, x + 1, y + 1));
        assert!(!board.is_valid_move(x, y, x + 2, y + 2));
    }

    #[test]
    fn valid_move_pawn() {
        let mut board = Board::new();
        let (x1, y1) = (0, 4);
        board.set_field(x1, y1, Some(Piece::new(Color::Black, Value::Pawn)));

        let (x2, y2) = (1, 5);
        board.set_field(x2, y2, Some(Piece::new(Color::White, Value::Pawn)));

        assert!(board.is_valid_move(x1, y1, x1 + 1, y1));
        assert!(board.is_valid_move(x1, y1, x1 + 2, y1));
        assert!(board.is_valid_move(x1, y1, x2, y2));
        assert!(board.is_valid_move(x2, y2, x2 + 1, y2));
        assert!(!board.is_valid_move(x1, y1, x1, y1));
        assert!(!board.is_valid_move(x2, y2, x2 + 2, y2));
        assert!(!board.is_valid_move(x1, y1, x1, y1 + 1));
    }
}
