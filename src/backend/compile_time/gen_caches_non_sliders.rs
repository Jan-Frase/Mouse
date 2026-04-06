use crate::backend::compile_time::gen_util::{
    is_square_at_left_edge, is_square_at_right_edge, is_square_valid, square_to_bb,
};
use crate::backend::constants::{A1, A8, H1, SIDES, SQUARES_AMOUNT};
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::piece::{Piece, Side};
use crate::backend::state::square::{Square, get_file, get_rank};

/// Initializes a collection of bitboards representing all possible moves for each square.
///
/// Since this function is const, it can be evaluated at compile time.
/// # Parameters
/// - `generate_moves`: A function that generates a `BitBoard` with all possible moves for the piece from a given `Square`.
///   This function is called for each square on the board.
///
/// # Returns
/// An array of `BitBoard` of size `SQUARES_AMOUNT`, where each entry corresponds to the
/// possible moves for the square at the same index.
pub const fn gen_potential_moves_cache(piece_type: Piece) -> [BitBoard; SQUARES_AMOUNT] {
    let mut potential_moves = [BitBoard::new(); SQUARES_AMOUNT];

    // iterate over all squares
    let mut square: Square = A1;
    while square < SQUARES_AMOUNT as u8 {
        // and generate the moves for that square
        potential_moves[square as usize] = match piece_type {
            Piece::Knight => generate_knight_moves(square),
            Piece::King => generate_king_moves(square),
            _ => panic!("Invalid piece type"),
        };

        square += 1;
    }

    potential_moves
}

/// Generates a `BitBoard` with all possible moves for a king piece from a given `Square`.
///
/// # Parameters
/// - `square`: A `Square` representing the current position of the king piece.
///
/// # Returns
/// A `BitBoard` containing all valid surrounding squares a king can move to.
const fn generate_king_moves(square: Square) -> BitBoard {
    let mut bitboard = BitBoard::new();

    let rank = get_rank(square);
    let file = get_file(square);

    // since this fn is const, we can't use a loop
    // instead we use a while loop
    // to iterate over the surrounding files
    let mut file_offset: i8 = -1;
    while file_offset <= 1 {
        // and ranks
        let mut rank_offset: i8 = -1;
        while rank_offset <= 1 {
            // skip the current square
            if file_offset == 0 && rank_offset == 0 {
                rank_offset += 1;
                continue;
            }
            // create the relevant square
            let current_rank = rank + rank_offset;
            let current_file = file + file_offset;

            // and add it if it's valid
            if is_square_valid(current_rank, current_file) {
                let current_square = (current_rank * 8 + current_file) as Square;
                let bb = square_to_bb(current_square);
                bitboard.value |= bb.value;
            }

            rank_offset += 1;
        }

        file_offset += 1;
    }
    bitboard
}

/// Same as above but for the knight.
const fn generate_knight_moves(square: Square) -> BitBoard {
    let mut bitboard = BitBoard::new();
    let file = get_file(square);
    let rank = get_rank(square);

    // The offsets for the knight moves. Starting in the top left corner.
    // `_ _ _ _ _ _ _ _`
    // `_ _ 2 _ 3 _ _ _`
    // `_ 1 _ _ _ 4 _ _`
    // `_ _ _ K _ _ _ _`
    // `_ 8 _ _ _ 5 _ _`
    // `_ _ 7 _ 6 _ _ _`
    // `_ _ _ _ _ _ _ _`
    // `_ _ _ _ _ _ _ _`
    let offset_x = [-2, -1, 1, 2, 2, 1, -1, -2];
    let offset_y = [1, 2, 2, 1, -1, -2, -2, -1];

    let mut index = 0;
    while index < 8 {
        // Calculate the current square.
        let x_pos = file + offset_x[index];
        let y_pos = rank + offset_y[index];

        // If it is valid, add it to the bitboard.
        if is_square_valid(x_pos, y_pos) {
            let current_square = (x_pos * 8 + y_pos) as Square;
            let bb = square_to_bb(current_square);
            bitboard.value |= bb.value;
        }

        index += 1;
    }

    bitboard
}

pub const fn gen_pawn_captures() -> [[BitBoard; SQUARES_AMOUNT]; SIDES] {
    let mut quiet_moves = [[BitBoard::new(); SQUARES_AMOUNT]; SIDES];

    let mut side_index = 0;
    while side_index < 2 {
        let active_color = match side_index {
            0 => Side::White,
            1 => Side::Black,
            _ => panic!("Invalid side index"),
        };
        let mut potential_moves = [BitBoard::new(); SQUARES_AMOUNT];

        // iterate over all squares
        let mut square: Square = A1;
        while square < SQUARES_AMOUNT as u8 {
            // generate a square struct from the index

            let bb = generate_pawn_attack_moves(square, active_color);

            // and generate the moves for that square
            potential_moves[square as usize] = bb;

            square += 1;
        }

        quiet_moves[side_index] = potential_moves;

        side_index += 1;
    }

    quiet_moves
}

const fn generate_pawn_attack_moves(square: Square, active_color: Side) -> BitBoard {
    let mut bitboard = BitBoard::new();
    let mut current_square = square;

    match active_color {
        Side::White => {
            if current_square >= A8 {
                return bitboard;
            }
            current_square += 8;
        }
        Side::Black => {
            if current_square <= H1 {
                return bitboard;
            }
            current_square -= 8;
        }
    }

    if !is_square_at_right_edge(current_square) {
        bitboard.value |= square_to_bb(current_square + 1).value;
    }
    if !is_square_at_left_edge(current_square) {
        bitboard.value |= square_to_bb(current_square - 1).value;
    }

    bitboard
}
