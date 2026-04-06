use crate::backend::constants::{A1, A2, A3, A4, A5, A6, A7, A8, H1, H2, H3, H4, H5, H6, H7, H8};
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::square::Square;

pub const fn is_square_at_right_edge(square: Square) -> bool {
    square == H1
        || square == H2
        || square == H3
        || square == H4
        || square == H5
        || square == H6
        || square == H7
        || square == H8
}

pub const fn is_square_at_left_edge(square: Square) -> bool {
    square == A1
        || square == A2
        || square == A3
        || square == A4
        || square == A5
        || square == A6
        || square == A7
        || square == A8
}

pub const fn is_square_valid(rank: i8, file: i8) -> bool {
    rank > 0 && rank < 8 && file > 0 && file < 8
}

pub const fn square_to_bb(square: Square) -> BitBoard {
    let mut bitboard = BitBoard::new();
    let bb: u64 = 1 << square;
    bitboard.value = bb;
    bitboard
}
