mod model;

fn main() {
    let black_king = model::piece::Piece::new(model::piece::Color::Black, model::piece::Value::King);
    let mut board = model::board::Board::new();
    board.set_field(0, 0, Some(black_king));
    println!("{}", board);
}
