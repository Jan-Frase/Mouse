use crate::backend::constants::SIDE_LENGTH;
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::square::Square;

pub const fn is_square_valid(square: Square) -> bool {
    square.file() >= 0 && square.file() <= 7 && square.rank() >= 0 && square.rank() <= 7
}

pub const fn square_to_bb(square: Square) -> BitBoard {
    let mut bitboard = BitBoard::new();
    let bb: u64 = 1 << (square.rank * 8 + square.file);
    bitboard.value = bb;
    bitboard
}

// ---------------------------------------
// Only for use during const!
// ---------------------------------------
pub const fn bb_from_squares(squares: &[Square]) -> BitBoard {
    let mut bitboard = BitBoard::new();

    let mut index = 0;
    while index < squares.len() {
        let square = squares[index];
        let bb: u64 = 1 << (square.rank * 8 + square.file);
        bitboard.value |= bb;
        index += 1;
    }

    bitboard
}

pub const fn bb_from_rank(rank: i8) -> BitBoard {
    let mut squares = [Square::new(0, 0); SIDE_LENGTH];
    let mut file = 0;

    while file < SIDE_LENGTH {
        squares[file] = Square::new(file as i8, rank);
        file += 1;
    }

    bb_from_squares(&squares)
}

pub const fn bb_from_file(file: i8) -> BitBoard {
    let mut squares = [Square::new(0, 0); SIDE_LENGTH];
    let mut rank = 0;

    while rank < SIDE_LENGTH {
        squares[rank] = Square::new(file, rank as i8);
        rank += 1;
    }

    bb_from_squares(&squares)
}
