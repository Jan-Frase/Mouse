use crate::backend::constants::{A1, SIDE_LENGTH};
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::square::Square;
use crate::backend::state::square::square_from_rank_and_file;

// ---------------------------------------
// Nothing in this file should be performance-critical!
// ---------------------------------------

pub const fn bb_from_squares(squares: &[Square]) -> BitBoard {
    let mut bitboard = BitBoard::new();

    let mut index = 0;
    while index < squares.len() {
        let square = squares[index];
        let bb: u64 = 1 << square;
        bitboard.value |= bb;
        index += 1;
    }

    bitboard
}

pub const fn bb_from_rank(rank: i8) -> BitBoard {
    let mut squares = [A1; SIDE_LENGTH];
    let mut file = 0;

    while file < SIDE_LENGTH {
        squares[file] = square_from_rank_and_file(rank, file as i8);
        file += 1;
    }

    bb_from_squares(&squares)
}

pub const fn bb_from_file(file: i8) -> BitBoard {
    let mut squares = [A1; SIDE_LENGTH];
    let mut rank = 0;

    while rank < SIDE_LENGTH {
        squares[rank] = square_from_rank_and_file(rank as i8, file);
        rank += 1;
    }

    bb_from_squares(&squares)
}
