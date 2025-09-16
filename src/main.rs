use crate::bitboard::BitBoard;
use crate::square::Square;

mod bitboard;
mod bitboard_manager;
mod piece;
mod square;

fn main() {
    let mut bitboard = BitBoard::new();

    let square = Square { file: 0, rank: 0 };

    bitboard.fill_square(square);

    let filled = bitboard.get_square(square);

    println!("Is filled {filled}")
}
