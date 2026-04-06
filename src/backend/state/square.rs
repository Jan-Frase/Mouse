use crate::backend::state::piece::Side;

// Square is simply an index where 0 is A1 and 63 is H8
pub type Square = u8;

pub const fn square_from_rank_and_file(rank: i8, file: i8) -> Square {
    (rank * 8 + file) as Square
}

pub const fn get_rank(square: Square) -> i8 {
    (square as i8) / 8
}

pub const fn get_file(square: Square) -> i8 {
    (square as i8) % 8
}

pub fn back_by_one(square: Square, color: Side) -> Square {
    match color {
        Side::White => square - 8,
        Side::Black => square + 8,
    }
}
