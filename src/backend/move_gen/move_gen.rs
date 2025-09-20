use crate::backend::moove::Moove;
use crate::backend::move_gen::compile_time::move_cache_non_sliders::get_moves_cache_for_piece;
use crate::backend::move_gen::move_gen_non_sliders::get_moves_for_non_slider_piece;
use crate::backend::piece::{Piece, PieceColor, PieceType};
use crate::backend::state::bitboard::BitBoard;
use crate::backend::state::bitboard_manager::BitBoardManager;
use crate::backend::state::game_state::GameState;
use crate::constants::SQUARES_AMOUNT;

/// Generates and returns all the valid moves for the current player's pieces
/// based on the provided game state. This is the entry point for the move generation.
///
/// # Parameters
///
/// * `game_state`: A reference to the current game state, which contains
///   information about the board, piece positions, and active color.
///
/// # Returns
///
/// * A `Vec<Moove>` containing all the computed legal moves for the current player's
///   pieces. Currently, this implementation only calculates the moves for the king piece.
pub fn get_moves(game_state: &GameState) -> Vec<Moove> {
    // Idea: have separate functions for each piece type
    // and then call them from here
    // each of them should iterate over their relevant bitboard
    let bitboard_manager = game_state.bit_board_manager();
    // Bitboard containing all pieces of the active color. These block moves.
    let friendly_pieces_bitboard = bitboard_manager.get_all_pieces_off(game_state.active_color());

    let mut all_pseudo_legal_moves = Vec::new();
    let active_color = game_state.active_color();

    // Iterate over all piece types. This might need adjustment for sliding pieces.
    for piece_type in PieceType::get_all_types() {
        // Get the moves cache for the current piece type.
        let moves_cache = get_moves_cache_for_piece(piece_type);
        // Get all moves for the current piece type.
        get_moves_for_piece(
            &mut all_pseudo_legal_moves,
            piece_type,
            active_color,
            moves_cache,
            bitboard_manager,
            friendly_pieces_bitboard,
        );
    }

    all_pseudo_legal_moves
}

/// Gets all pseudo legal moves for one piece.
fn get_moves_for_piece(
    all_pseudo_legal_moves: &mut Vec<Moove>,
    piece_type: PieceType,
    active_color: PieceColor,
    moves_cache: [BitBoard; SQUARES_AMOUNT],
    bitboard_manager: &BitBoardManager,
    friendly_pieces_bitboard: BitBoard,
) {
    let piece = Piece::new(piece_type, active_color);
    let piece_bitboard = bitboard_manager.get_bitboard(piece);
    let mut moves =
        get_moves_for_non_slider_piece(moves_cache, *piece_bitboard, friendly_pieces_bitboard);
    all_pseudo_legal_moves.append(&mut moves);
}
