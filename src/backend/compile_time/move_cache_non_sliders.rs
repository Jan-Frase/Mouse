use crate::backend::compile_time::generated::cache_king::CACHE_KING;
use crate::backend::constants::{SIDES, SQUARES_AMOUNT};
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::piece::{Piece, Side};
use crate::backend::state::square::Square;

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
pub const KING_MOVES: [BitBoard; SQUARES_AMOUNT] = parse_generated();

/// The same as above, but for the knight.
pub const KNIGHT_MOVES: [BitBoard; SQUARES_AMOUNT] = calculate_potential_moves_cache(Piece::Knight);

enum PawnMoveType {
    Quiet,
    Capture,
    DoublePush,
}

/// All quiet moves for pawns.
pub const PAWN_QUIET_MOVES: [[BitBoard; SQUARES_AMOUNT]; SIDES] =
    generate_pawn_moves(PawnMoveType::Quiet);

/// All capture moves for pawns.
pub const PAWN_CAPTURE_MOVES: [[BitBoard; SQUARES_AMOUNT]; SIDES] =
    generate_pawn_moves(PawnMoveType::Capture);

/// All capture moves for pawns.
pub const PAWN_DOUBLE_PUSH_MOVES: [[BitBoard; SQUARES_AMOUNT]; SIDES] =
    generate_pawn_moves(PawnMoveType::DoublePush);

const fn parse_generated() -> [BitBoard; SQUARES_AMOUNT] {
    let mut potential_moves = [BitBoard::new(); SQUARES_AMOUNT];

    let mut square_index: usize = 0;
    while square_index < SQUARES_AMOUNT {
        let value = CACHE_KING[square_index];
        let bb = BitBoard { value };
        potential_moves[square_index] = bb;

        square_index += 1;
    }

    potential_moves
}

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
const fn calculate_potential_moves_cache(piece_type: Piece) -> [BitBoard; SQUARES_AMOUNT] {
    let mut potential_moves = [BitBoard::new(); SQUARES_AMOUNT];

    // iterate over all squares
    let mut square_index: usize = 0;
    while square_index < SQUARES_AMOUNT {
        // generate a square struct from the index
        let square = Square::index_to_square(square_index as i8);

        // and generate the moves for that square
        potential_moves[square_index] = match piece_type {
            Piece::Knight => generate_knight_moves(square),
            Piece::King => generate_king_moves(square),
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

const fn generate_pawn_moves(pawn_move_type: PawnMoveType) -> [[BitBoard; SQUARES_AMOUNT]; SIDES] {
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
        let mut square_index: usize = 0;
        while square_index < SQUARES_AMOUNT {
            let mut bitboard = BitBoard::new();
            // generate a square struct from the index
            let square = Square::index_to_square(square_index as i8);

            match pawn_move_type {
                PawnMoveType::Quiet => {
                    generate_pawn_quiet_moves(square, &mut bitboard, active_color);
                }
                PawnMoveType::Capture => {
                    generate_pawn_attack_moves(square, &mut bitboard, active_color);
                }
                PawnMoveType::DoublePush => {
                    generate_pawn_double_push_moves(square, &mut bitboard, active_color);
                }
            }

            // and generate the moves for that square
            potential_moves[square_index] = bitboard;

            square_index += 1;
        }

        quiet_moves[side_index] = potential_moves;

        side_index += 1;
    }

    quiet_moves
}

const fn generate_pawn_quiet_moves(square: Square, bitboard: &mut BitBoard, active_color: Side) {
    let forward_square = square.forward_by_one(active_color);
    if forward_square.is_valid() {
        bitboard.fill_square(forward_square);
    }
}

const fn generate_pawn_attack_moves(square: Square, bitboard: &mut BitBoard, active_color: Side) {
    let right_diagonal_square = square.right_by_one().forward_by_one(active_color);
    let left_diagonal_square = square.left_by_one().forward_by_one(active_color);

    if right_diagonal_square.is_valid() {
        bitboard.fill_square(right_diagonal_square);
    }
    if left_diagonal_square.is_valid() {
        bitboard.fill_square(left_diagonal_square);
    }
}

const fn generate_pawn_double_push_moves(
    square: Square,
    bitboard: &mut BitBoard,
    active_color: Side,
) {
    if !square.is_pawn_start(active_color) {
        return;
    }

    let double_pushed_square = square
        .forward_by_one(active_color)
        .forward_by_one(active_color);
    bitboard.fill_square(double_pushed_square);
}
