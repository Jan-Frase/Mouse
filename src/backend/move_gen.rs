use crate::backend::game_state::GameState;
use crate::backend::moove::Moove;
use crate::backend::move_gen_king::{KING_MOVES, get_moves_for_piece};
use crate::backend::piece::Piece;
use crate::backend::piece::PieceType::King;

pub fn get_moves(game_state: &GameState) -> Vec<Moove> {
    // Idea: have separate functions for each piece type
    // and then call them from here
    // each of them should iterate over their relevant bitboard
    let bitboard_manager = game_state.bit_board_manager();
    let friendly_pieces_bitboard = bitboard_manager.get_all_pieces_off(game_state.active_color());

    let king = Piece::new(King, game_state.active_color());
    let king_bitboard = bitboard_manager.get_bitboard(king);
    get_moves_for_piece(KING_MOVES, *king_bitboard, friendly_pieces_bitboard)
}
