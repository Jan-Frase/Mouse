use crate::backend::caches::{KING_MOVES, KNIGHT_MOVES, PAWN_CAPTURE_MOVES};
use crate::backend::movegen::move_gen_sliders::get_slider_moves_at_square;
use crate::backend::game_state::state::State;
use crate::backend::types::bitboard::BitBoard;
use crate::backend::types::piece::{ALL_PIECES, Piece, Side};
use crate::backend::types::piece::Piece::King;
use crate::backend::types::square::Square;

/// Checks if a given player's king is in check in the current game game_state.
///
/// This function determines whether the king of the specified `color` is under attack
/// by any opposing pieces in the `game_state`. It does so by leveraging precomputed
/// bitboards that describe potential piece movements. For now, the implementation
/// is specifically checking whether the opposing king is delivering a check.
pub fn is_in_check(state: &State, color: Side) -> bool {
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
    let king_bb = state.bb_mngr.get_piece_bb(Piece::King);
    let side_bb = state.bb_mngr.get_all_pieces_bb_off(color);
    let mut bb = king_bb & side_bb;
    let king_square = bb.next().unwrap();

    is_in_check_on_square(state, color, king_square)
}

pub fn is_in_check_on_square(state: &State, side: Side, king_square: Square) -> bool {
    let friendly_bb = state
        .bb_mngr
        .get_all_pieces_bb_off(side);

    let enemy_bb = state
        .bb_mngr
        .get_all_pieces_bb_off(side.oppo());

    // Iterate over all pieces. Let`s assume we are checking for knights.
    for piece_type in ALL_PIECES {
        // Get the bitboard that represents all possible attacks.
        let attack_bb = get_attack_bb(side, king_square, friendly_bb, enemy_bb, piece_type);

        // Get the bitboard that marks where enemy knights are standing.
        let enemy_piece_bb = state.bb_mngr.get_piece_bb(piece_type) & enemy_bb;

        // Check if at least one of the places we could move to contains an enemy knight.
        let resulting_bb = attack_bb & enemy_piece_bb;
        // If so, we know that the king is in check.
        if resulting_bb.is_not_empty() {
            return true;
        }
    }
    false
}

pub fn get_checking_squares(state: &State) -> BitBoard {
    let mut squares_with_attackers_bb = BitBoard::new();
    let side = state.active_color;
    let mut king_square = state.bb_mngr.get_piece_bb(King) & state.bb_mngr.get_all_pieces_bb_off(side);
    let king_square = king_square.next().unwrap();

    let friendly_bb = state.bb_mngr.get_all_pieces_bb_off(side);

    let enemy_bb = state.bb_mngr.get_all_pieces_bb_off(side.oppo());

    // Iterate over all pieces excluding the king. Let's assume we are checking for knights.
    // A king can't check another king.
    for piece_type in[Piece::Rook, Piece::Knight, Piece::Bishop, Piece::Queen, Piece::Pawn] {
        // Get the bitboard that represents all possible attacks.
        let attack_bb = get_attack_bb(side, king_square, friendly_bb, enemy_bb, piece_type);

        // Get the bitboard that marks where enemy knights are standing.
        let enemy_piece_bb = state.bb_mngr.get_piece_bb(piece_type) & enemy_bb;

        // Check if at least one of the places we could move to contains an enemy knight.
        let resulting_bb = attack_bb & enemy_piece_bb;
        
        squares_with_attackers_bb |= resulting_bb;
    }

    squares_with_attackers_bb
}

fn get_attack_bb(side: Side, king_square: Square, friendly_bb: BitBoard, enemy_bb: BitBoard, piece_type: Piece) -> BitBoard {
    match piece_type {
        Piece::King => KING_MOVES[king_square as usize],
        Piece::Knight => KNIGHT_MOVES[king_square as usize],
        Piece::Pawn => PAWN_CAPTURE_MOVES[side as usize][king_square as usize],
        Piece::Rook => get_slider_moves_at_square::<true>(king_square, friendly_bb, enemy_bb),
        Piece::Bishop => get_slider_moves_at_square::<false>(king_square, friendly_bb, enemy_bb),
        Piece::Queen => 
            get_slider_moves_at_square::<true>(king_square, friendly_bb, enemy_bb) 
                | get_slider_moves_at_square::<false>(king_square, friendly_bb, enemy_bb),
    }
}