use crate::backend::compile_time::gen_caches::{KING_MOVES, KNIGHT_MOVES, PAWN_CAPTURE_MOVES};
use crate::backend::movegen::move_gen_sliders::get_slider_moves_at_square;
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::game::state::State;
use crate::backend::state::piece::Piece::{Bishop, Queen, Rook};
use crate::backend::state::piece::{ALL_PIECES, Piece, Side};
use crate::backend::state::square::Square;

pub fn is_in_check_on_square(game_state: &State, color: Side, king_square: Square) -> bool {
    let friendly_bb = game_state.bb_manager().get_all_pieces_bb_off(color);
    let enemy_bb = game_state
        .bb_manager()
        .get_all_pieces_bb_off(color.opposite());

    // Iterate over all pieces. Let`s assume we are checking for knights.
    for piece_type in ALL_PIECES {
        // Get the bitboard that represents all possible attacks.
        let attack_bitboard = get_attack_bitboard_for_piece_and_square(
            piece_type,
            color,
            king_square,
            friendly_bb,
            enemy_bb,
        );

        // Get the bitboard that marks where enemy knights are standing.
        let enemy_piece_bitboard = game_state.bb_manager().get_piece_bb(piece_type) & enemy_bb;

        // Check if at least one of the places we could move to contains an enemy knight.
        let resulting_bitboard = attack_bitboard & enemy_piece_bitboard;
        // If so, we know that the king is in check.
        if resulting_bitboard.is_not_empty() {
            return true;
        }
    }
    false
}

/// Checks if a given player's king is in check in the current game state.
///
/// This function determines whether the king of the specified `color` is under attack
/// by any opposing pieces in the `game_state`. It does so by leveraging precomputed
/// bitboards that describe potential piece movements. For now, the implementation
/// is specifically checking whether the opposing king is delivering a check.
///
/// ### Arguments
///
/// - `game_state`: A reference to the current `GameState` instance, which holds
///   information about the positions of all pieces and their relationships on the board.
/// - `color`: A `PieceColor` value (e.g., `White` or `Black`), representing the side
///   whose king is being evaluated.
///
/// ### Returns
///
/// - A boolean value:
///   - `true` if the king of the specified color is in check.
///   - `false` otherwise.
pub fn is_in_check(game_state: &State, color: Side) -> bool {
    // Idea:
    // If, for example, color == white, we want to figure out if white is currently in check.
    // We then pretend that the white king is one after the other replaced by: pawn, rook, bishop, queen, king.
    // We then calculate all possible attacks for each of these pieces as a bitboard.
    // Since that's what we already do for movegen, we can just reuse that.
    // If we have the white king on A1 and a black bishop on C3:
    // We can now generate all possible attacks for a (imaginary) bishop on A1 as a bitboard.
    // We can & this bitboard with the bitboard for black bishops and realize that it is not empty.
    // Thus, we now know that the white king is in check by a black bishop.
    // I hope this makes sense :)
    let king_square = get_kings_square(game_state, color);
    is_in_check_on_square(game_state, color, king_square)
}

/// Returns the square where the king of the respective side is located.
fn get_kings_square(game_state: &State, color: Side) -> Square {
    let king_bitboard = game_state.bb_manager().get_piece_bb(Piece::King);
    let side_bb = game_state.bb_manager().get_all_pieces_bb_off(color);
    let mut bb = king_bitboard & side_bb;
    bb.next().unwrap()
}

fn get_attack_bitboard_for_piece_and_square(
    piece_type: Piece,
    piece_color: Side,
    square: Square,
    friendly_bb: BitBoard,
    enemy_bb: BitBoard,
) -> BitBoard {
    match piece_type {
        Piece::King => KING_MOVES[square.to_index()],
        Piece::Knight => KNIGHT_MOVES[square.to_index()],
        Piece::Pawn => PAWN_CAPTURE_MOVES[piece_color as usize][square.to_index()],
        Rook => get_slider_moves_at_square(Rook, square, friendly_bb, enemy_bb),
        Bishop => get_slider_moves_at_square(Bishop, square, friendly_bb, enemy_bb),
        Queen => get_slider_moves_at_square(Queen, square, friendly_bb, enemy_bb),
    }
}
