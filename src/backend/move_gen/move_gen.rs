use crate::backend::moove::Moove;
use crate::backend::move_gen::move_gen_king::KING_MOVES;
use crate::backend::move_gen::move_gen_non_sliders::get_moves_for_piece;
use crate::backend::piece::Piece;
use crate::backend::piece::PieceType::King;
use crate::backend::state::game_state::GameState;

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
    let friendly_pieces_bitboard = bitboard_manager.get_all_pieces_off(game_state.active_color());

    // For now only the king is implemented.
    let king = Piece::new(King, game_state.active_color());
    let king_bitboard = bitboard_manager.get_bitboard(king);
    get_moves_for_piece(KING_MOVES, *king_bitboard, friendly_pieces_bitboard)
}
