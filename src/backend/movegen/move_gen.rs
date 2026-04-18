use crate::backend::caches::{BETWEEN_TABLE, KING_MOVES, KNIGHT_MOVES};
use crate::backend::constants::SQUARES_AMOUNT;
use crate::backend::types::moove::Moove;
use crate::backend::movegen::move_gen_king::gen_castles;
use crate::backend::movegen::move_gen_pawn::gen_pawn_moves;
use crate::backend::movegen::move_gen_sliders::get_slider_moves;
use crate::backend::types::bitboard::BitBoard;
use crate::backend::game_state::state::State;
use crate::backend::movegen::check_decider::get_checking_squares;
use crate::backend::types::piece::Piece::*;
use crate::backend::types::square::Square;

/// Generates and returns all the pseudo legal moves for the current player's pieces
/// based on the provided game game_state. This is the entry point for the move generation.
///
/// # Parameters
///
/// * `game_state`: A reference to the current game game_state, which contains
///   information about the board, piece positions, and active color.
///
/// # Returns
///
/// * A `Vec<Moove>` containing all the computed pseudo legal moves for the current player's
///   pieces.
pub fn get_pseudo_legal_moves(state: &State) -> Vec<Moove> {
    let friendly_pieces_bb = state.bb_mngr.get_all_pieces_bb_off(state.active_color);
    let enemy_pieces_bb = state
        .bb_mngr
        .get_all_pieces_bb_off(state.active_color.oppo());

    // This is a bitboard marking each piece that is currently checking the king (at most 2).
    let checking_squares = get_checking_squares(state);
    let true_bit_amount = checking_squares.value.count_ones();
    let double_check = true_bit_amount == 2;
    let no_check = true_bit_amount == 0;

    let mut moves = Vec::with_capacity(50);

    // Move gen for king (excluding castles) ...
    iterate_over_bitboard_for_non_slider(
        &mut moves,
        KING_MOVES,
        state
            .bb_mngr
            .get_colored_piece_bb(King, state.active_color),
        friendly_pieces_bb,
    );

    // Early return if we are in double check.
    if double_check { return moves; }

    // Completely filled with ones by default.
    // Idea from here: https://web.archive.org/web/20250910134338/https://www.codeproject.com/articles/Worlds-fastest-Bitboard-Chess-Movegenerator
    let mut checkmask = BitBoard{ value: u64::MAX };
    if !no_check { checkmask.value = 0; }
    let king_square = state.bb_mngr
        .get_colored_piece_bb(King, state.active_color)
        .next().unwrap();
    for square in checking_squares {
        checkmask |= BETWEEN_TABLE[square as usize][king_square as usize];
    }

    // ... and knights.
    iterate_over_bitboard_for_non_slider(
        &mut moves,
        KNIGHT_MOVES,
        state
            .bb_mngr
            .get_colored_piece_bb(Knight, state.active_color),
        friendly_pieces_bb,
    );

    // Can't castle when in check.
    if no_check {
        gen_castles(&mut moves, state, state.bb_mngr.get_all_pieces_bb());
    }

    // Gen pawn moves, quiet, captures, double pushes
    gen_pawn_moves(
        &mut moves,
        state,
        friendly_pieces_bb,
        enemy_pieces_bb,
        state.active_color,
    );

    // Gen queen..
    get_slider_moves(
        &mut moves,
        Queen,
        state
            .bb_mngr
            .get_colored_piece_bb(Queen, state.active_color),
        friendly_pieces_bb,
        enemy_pieces_bb,
        checkmask
    );
    // ..bishop..
    get_slider_moves(
        &mut moves,
        Bishop,
        state
            .bb_mngr
            .get_colored_piece_bb(Bishop, state.active_color),
        friendly_pieces_bb,
        enemy_pieces_bb,
        checkmask
    );
    //..and rook moves :)
    get_slider_moves(
        &mut moves,
        Rook,
        state
            .bb_mngr
            .get_colored_piece_bb(Rook, state.active_color),
        friendly_pieces_bb,
        enemy_pieces_bb,
        checkmask
    );

    moves
}

// ------------------------------------
// Move gen core logic
// ------------------------------------

pub(crate) fn iterate_over_bitboard_for_non_slider(
    moves: &mut Vec<Moove>,
    moves_cache: [BitBoard; SQUARES_AMOUNT],
    piece_bb: BitBoard,
    mask_bitboard: BitBoard,
) {
    // Example: We are doing this for all knights.
    // The `moves_cache` array would for each square contain all viable moves for a knight.

    // We iterate over all squares with a knight on it...
    for square in piece_bb {
        // ... get the potential moves for the piece on that square...
        // SLIDER: (This only works this easily for non-sliders)
        let mut potential_moves_bb = moves_cache[square as usize];
        // ... apply the mask ...
        potential_moves_bb &= !mask_bitboard;

        //... and convert the resulting bitboard to a list of moves.
        convert_bitboard_to_moves(moves, square, potential_moves_bb);
    }
}

pub fn convert_bitboard_to_moves(moves: &mut Vec<Moove>, square: Square, moves_bitboard: BitBoard) {
    // generate all the moves
    for to_square in moves_bitboard {
        moves.push(Moove::new(square, to_square))
    }
}
