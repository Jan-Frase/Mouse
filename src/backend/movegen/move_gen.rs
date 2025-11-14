use crate::backend::compile_time::gen_caches::{KING_MOVES, KNIGHT_MOVES};
use crate::backend::constants::SQUARES_AMOUNT;
use crate::backend::movegen::moove::Moove;
use crate::backend::movegen::move_gen_king_util::gen_castles;
use crate::backend::movegen::move_gen_pawn_util::gen_pawn_moves;
use crate::backend::movegen::move_gen_sliders::get_slider_moves;
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::game::state::State;
use crate::backend::state::piece::Piece::*;
use crate::backend::state::square::Square;

/// Generates and returns all the pseudo legal moves for the current player's pieces
/// based on the provided game state. This is the entry point for the move generation.
///
/// # Parameters
///
/// * `game_state`: A reference to the current game state, which contains
///   information about the board, piece positions, and active color.
///
/// # Returns
///
/// * A `Vec<Moove>` containing all the computed pseudo legal moves for the current player's
///   pieces.
pub fn get_pseudo_legal_moves(state: &State) -> Vec<Moove> {
    let friendly_pieces_bb = state.bb_manager.get_all_pieces_bb_off(state.active_color);
    let enemy_pieces_bb = state
        .bb_manager
        .get_all_pieces_bb_off(state.active_color.opposite());

    let mut moves = Vec::with_capacity(50);

    // Move gen for king and knight (excluding castles)
    iterate_over_bitboard_for_non_slider(
        &mut moves,
        KING_MOVES,
        state
            .bb_manager
            .get_colored_piece_bb(King, state.active_color),
        friendly_pieces_bb,
    );

    iterate_over_bitboard_for_non_slider(
        &mut moves,
        KNIGHT_MOVES,
        state
            .bb_manager
            .get_colored_piece_bb(Knight, state.active_color),
        friendly_pieces_bb,
    );

    gen_castles(&mut moves, state, state.bb_manager.get_all_pieces_bb());

    // Gen pawn moves, quiet, captures, double pushes
    gen_pawn_moves(
        &mut moves,
        state,
        friendly_pieces_bb,
        enemy_pieces_bb,
        state.active_color,
    );

    // Gen queen, bishop and rook moves
    get_slider_moves(
        &mut moves,
        Queen,
        state
            .bb_manager
            .get_colored_piece_bb(Queen, state.active_color),
        friendly_pieces_bb,
        enemy_pieces_bb,
    );
    get_slider_moves(
        &mut moves,
        Bishop,
        state
            .bb_manager
            .get_colored_piece_bb(Bishop, state.active_color),
        friendly_pieces_bb,
        enemy_pieces_bb,
    );
    get_slider_moves(
        &mut moves,
        Rook,
        state
            .bb_manager
            .get_colored_piece_bb(Rook, state.active_color),
        friendly_pieces_bb,
        enemy_pieces_bb,
    );

    moves
}

// ------------------------------------
// Move gen core logic
// ------------------------------------

pub(crate) fn iterate_over_bitboard_for_non_slider(
    moves: &mut Vec<Moove>,
    moves_cache: [BitBoard; SQUARES_AMOUNT],
    piece_bitboard: BitBoard,
    mask_bitboard: BitBoard,
) {
    // Example: We are doing this for all knights.
    // The `moves_cache` array would for each square contain all viable moves for a knight.

    // We iterate over all squares with a knight on it...
    for square in piece_bitboard {
        // ... get the potential moves for the piece on that square...
        // SLIDER: (This only works this easily for non-sliders)
        let mut potential_moves_bitboard = moves_cache[square.square_to_index()];
        // ... apply the mask ...
        potential_moves_bitboard &= !mask_bitboard;

        //... and convert the resulting bitboard to a list of moves.
        convert_bitboard_to_moves(
            moves,
            square,
            potential_moves_bitboard,
        );
    }
}

pub fn convert_bitboard_to_moves(moves: &mut Vec<Moove>,square: Square, moves_bitboard: BitBoard) {
    // generate all the moves
    for to_square in moves_bitboard {
        moves.push(Moove::new(square, to_square))
    }
}
