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

pub fn square_to_string(square: Square) -> String {
    let mut result = String::new();

    let file = get_file(square);
    let rank = get_rank(square);

    result.push_str(match file {
        0 => "a",
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        7 => "h",
        _ => panic!("Invalid file value"),
    });

    result.push_str(match rank {
        0 => "1",
        1 => "2",
        2 => "3",
        3 => "4",
        4 => "5",
        5 => "6",
        6 => "7",
        7 => "8",
        _ => panic!("Invalid rank value"),
    });

    result
}
