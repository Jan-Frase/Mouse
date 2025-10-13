use crate::backend::compile_time::gen_caches::{KING_MOVES, KNIGHT_MOVES};
use crate::backend::constants::SQUARES_AMOUNT;
use crate::backend::movegen::moove::Moove;
use crate::backend::movegen::move_gen_king_util::gen_castles;
use crate::backend::movegen::move_gen_pawn_util::gen_pawn_moves;
use crate::backend::movegen::move_gen_sliders::get_slider_moves;
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::game::state::State;
use crate::backend::state::piece::Piece::{King, Knight};
use crate::backend::state::piece::{SLIDER_PIECES, TRIVIAL_PIECES};
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
pub fn get_pseudo_legal_moves(game_state: &State) -> Vec<Moove> {
    let bitboard_manager = game_state.bb_manager();
    // Bitboard containing all pieces of the active color. These block moves.
    let friendly_pieces_bb = bitboard_manager.get_all_pieces_bb_off(game_state.active_color());
    // Bitboard containing all pieces of the opponent color. These are relevant for sliders and pawn captures.
    let enemy_pieces_bb =
        bitboard_manager.get_all_pieces_bb_off(game_state.active_color().opposite());

    let mut all_pseudo_legal_moves = Vec::new();
    let active_color = game_state.active_color();

    // Move gen for king and knight (excluding castles)
    for trivial_type in TRIVIAL_PIECES {
        let moves_cache = match trivial_type {
            Knight => KNIGHT_MOVES,
            King => KING_MOVES,
            _ => panic!("This is not a trivial type."),
        };
        let mut moves = iterate_over_bitboard_for_non_slider(
            moves_cache,
            bitboard_manager.get_colored_piece_bb(trivial_type, active_color),
            friendly_pieces_bb,
        );
        all_pseudo_legal_moves.append(&mut moves);
    }

    gen_castles(
        &mut all_pseudo_legal_moves,
        game_state,
        friendly_pieces_bb | enemy_pieces_bb,
    );

    // Gen pawn moves, quiet, captures, double pushes
    gen_pawn_moves(
        game_state,
        bitboard_manager,
        friendly_pieces_bb,
        enemy_pieces_bb,
        &mut all_pseudo_legal_moves,
        active_color,
    );

    // Gen queen, bishop and rook moves
    for slider_type in SLIDER_PIECES {
        let mut moves = get_slider_moves(
            slider_type,
            bitboard_manager.get_colored_piece_bb(slider_type, active_color),
            friendly_pieces_bb,
            enemy_pieces_bb,
        );
        all_pseudo_legal_moves.append(&mut moves);
    }

    all_pseudo_legal_moves
}

// ------------------------------------
// Move gen core logic
// ------------------------------------

pub(crate) fn iterate_over_bitboard_for_non_slider(
    moves_cache: [BitBoard; SQUARES_AMOUNT],
    piece_bitboard: BitBoard,
    mask_bitboard: BitBoard,
) -> Vec<Moove> {
    // PERF: Instead of creating a new vector for each piece, we could reuse the same vector and append to it.
    let mut moves: Vec<Moove> = Vec::new();

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
        moves.append(&mut convert_bitboard_to_moves(
            square,
            potential_moves_bitboard,
        ));
    }

    moves
}

pub fn convert_bitboard_to_moves(square: Square, moves_bitboard: BitBoard) -> Vec<Moove> {
    // generate all the moves
    let mut moves: Vec<Moove> = Vec::new();
    for to_square in moves_bitboard {
        moves.push(Moove::new(square, to_square))
    }
    moves
}
