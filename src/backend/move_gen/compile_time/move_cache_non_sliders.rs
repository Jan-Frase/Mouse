use crate::backend::piece::PieceType;
use crate::backend::square::Square;
use crate::backend::state::bitboard::BitBoard;
use crate::constants::SQUARES_AMOUNT;

/// All of this gets generated at compile time, in the functions below.
/// At runtime, we only have to read the values.
/// It contains for each square, a bitboard with every square that the king could potentially move to set to 1.
/// Example:
/// This: `KING_MOVES[Square::new(0,0)]` returns at bitboard that looks like this:
/// `_ _ _ _ _ _ _ _`
/// `_ _ _ _ _ _ _ _`
/// `_ _ _ _ _ _ _ _`
/// `_ _ _ _ _ _ _ _`
/// `_ _ _ _ _ _ _ _`
/// `_ _ _ _ _ _ _ _`
/// `X X _ _ _ _ _ _`
/// `_ X _ _ _ _ _ _`
/// At runtime we have to apply some further checks to this bitboard:
/// 1. Are some of these squares blocked by friendly pieces?
/// 2. Would this move put me in check etc...?
pub const KING_MOVES: [BitBoard; SQUARES_AMOUNT] = calculate_potential_moves_cache(PieceType::King);

/// The same as above, but for the knight.
pub const KNIGHT_MOVES: [BitBoard; SQUARES_AMOUNT] =
    calculate_potential_moves_cache(PieceType::Knight);

pub fn get_moves_cache_for_piece(piece_type: PieceType) -> [BitBoard; SQUARES_AMOUNT] {
    match piece_type {
        PieceType::Knight => KNIGHT_MOVES,
        PieceType::King => KING_MOVES,
        _ => panic!("Invalid piece type"),
    }
}

/// Initializes a collection of bitboards representing all possible king moves for each square.
///
/// Since this function is const, it can be evaluated at compile time.
/// # Parameters
/// - `generate_moves`: A function that generates a `BitBoard` with all possible moves for the piece from a given `Square`.
///   This function is called for each square on the board.
///
/// # Returns
/// An array of `BitBoard` of size `SQUARES_AMOUNT`, where each entry corresponds to the
/// possible moves for the square at the same index.
const fn calculate_potential_moves_cache(piece_type: PieceType) -> [BitBoard; SQUARES_AMOUNT] {
    let mut potential_moves = [BitBoard::new(); SQUARES_AMOUNT];

    // iterate over all squares
    let mut square_index: usize = 0;
    while square_index < SQUARES_AMOUNT {
        // generate a square struct from the index
        let square = Square::index_to_square(square_index as i8);

        // and generate the moves for that square
        potential_moves[square_index] = match piece_type {
            PieceType::Knight => generate_knight_moves(square),
            PieceType::King => generate_king_moves(square),
            _ => panic!("Invalid piece type"),
        };

        square_index += 1;
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

    // since this fn is const, we can't use a loop
    // instead we use a while loop
    // to iterate over the surrounding files
    let mut file_offset = -1;
    while file_offset <= 1 {
        // and ranks
        let mut rank_offset = -1;
        while rank_offset <= 1 {
            // skip the current square
            if file_offset == 0 && rank_offset == 0 {
                rank_offset += 1;
                continue;
            }
            // create the relevant square
            let current_square =
                Square::new(square.file() + file_offset, square.rank() + rank_offset);

            // and add it if it's valid
            if current_square.is_valid() {
                bitboard.fill_square(current_square);
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
        let x_pos = square.file() + offset_x[index];
        let y_pos = square.rank() + offset_y[index];
        let current_square = Square::new(x_pos, y_pos);

        // If it is valid, add it to the bitboard.
        if current_square.is_valid() {
            bitboard.fill_square(current_square);
        }

        index += 1;
    }

    bitboard
}
