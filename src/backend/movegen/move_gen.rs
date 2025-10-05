use crate::backend::constants::SQUARES_AMOUNT;
use crate::backend::movegen::compile_time::move_cache_non_sliders::{KING_MOVES, KNIGHT_MOVES};
use crate::backend::movegen::moove::Moove;
use crate::backend::movegen::move_gen_king_util::gen_castles;
use crate::backend::movegen::move_gen_pawn_util::gen_pawn_moves;
use crate::backend::movegen::move_gen_sliders::get_moves_for_non_slider_piece;
use crate::backend::state::board::bitboard::Bitboard;
use crate::backend::state::game::game_state::GameState;
use crate::backend::state::piece::PieceType::{King, Knight};
use crate::backend::state::piece::{Piece, PieceType};
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
pub fn get_pseudo_legal_moves(game_state: &GameState) -> Vec<Moove> {
    let bitboard_manager = game_state.bb_manager();
    // Bitboard containing all pieces of the active color. These block moves.
    let friendly_pieces_bb = bitboard_manager.get_all_pieces_off(game_state.active_color());
    // Bitboard containing all pieces of the opponent color. These are relevant for sliders and pawn captures.
    let enemy_pieces_bb = bitboard_manager.get_all_pieces_off(game_state.active_color().opposite());

    let mut all_pseudo_legal_moves = Vec::new();
    let active_color = game_state.active_color();

    // Move gen for king and knight (excluding castles)
    for trivial_type in PieceType::get_trivial_types() {
        let moves_cache = match trivial_type {
            Knight => KNIGHT_MOVES,
            King => KING_MOVES,
            _ => panic!("This is not a trivial type."),
        };
        let mut moves = iterate_over_bitboard_for_non_slider(
            moves_cache,
            *bitboard_manager.get_bitboard(Piece::new(trivial_type, active_color)),
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
    for slider_type in PieceType::get_slider_types() {
        let mut moves = get_moves_for_non_slider_piece(
            slider_type,
            *bitboard_manager.get_bitboard(Piece::new(slider_type, active_color)),
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
    moves_cache: [Bitboard; SQUARES_AMOUNT],
    piece_bitboard: Bitboard,
    mask_bitboard: Bitboard,
) -> Vec<Moove> {
    // PERF: Instead of creating a new vector for each piece, we could reuse the same vector and append to it.
    let mut moves: Vec<Moove> = Vec::new();

    // Example: We are doing this for all knights.
    // The `moves_cache` array would for each square contain all viable moves for a knight.

    // Assuming we are in the starting position as white `squares_with_piece` would be [B1, G1].
    let squares_with_piece = piece_bitboard.get_all_true_squares();
    // We then iterate over all these squares...
    for square in squares_with_piece.iter() {
        // ... get the potential moves for the piece on that square...
        // SLIDER: (This only works this easily for non-sliders)
        let mut potential_moves_bitboard = moves_cache[square.square_to_index()];
        // ... apply the mask ...
        potential_moves_bitboard &= !mask_bitboard;

        //... and convert the resulting bitboard to a list of moves.
        moves.append(&mut convert_bitboard_to_moves(
            *square,
            potential_moves_bitboard,
        ));
    }

    moves
}

fn convert_bitboard_to_moves(square: Square, moves_bitboard: Bitboard) -> Vec<Moove> {
    // Now take the resulting bitboard and convert all true squares to a list of squares.
    let squares_we_can_move_to = moves_bitboard.get_all_true_squares();

    // generate all the moves
    let mut moves: Vec<Moove> = Vec::with_capacity(squares_we_can_move_to.len());
    for to_square in squares_we_can_move_to {
        moves.push(Moove::new(square, to_square))
    }
    moves
}
