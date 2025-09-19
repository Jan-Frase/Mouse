use crate::backend::move_gen::move_gen_king::KING_MOVES;
use crate::backend::piece::PieceType::King;
use crate::backend::piece::{Piece, PieceColor, PieceType};
use crate::backend::square::Square;
use crate::backend::state::game_state::GameState;

pub fn is_in_check(game_state: &GameState, color: PieceColor) -> bool {
    // Idea:
    // If color == white, we want to figure out if white is currently in check.
    // We then pretend that the white king is one after the other replaced by: pawn, rook, bishop, queen, king.
    // We then calculate all possible attacks for each of these pieces as a bitboard.
    // Since that's what we already do for movegen, we can just reuse that.
    // If we have the white king on A1 and a black bishop on C3:
    // We can now generate all possible attacks for a bishop on A1 as a bitboard.
    // We can & this bitboard with the bitboard for black bishops and realise that it is not empty.
    // Thus, we now know that the white king is in check by a black bishop.
    // I hope this makes sense :)
    let king_square = get_kings_square(game_state, color);

    // For now, I will just implement it for the king.
    let king_move_bitboard = KING_MOVES[king_square.square_to_index() as usize];
    let enemey_king_bitboard = game_state
        .bit_board_manager()
        .get_bitboard(Piece::new(King, color.opposite()));
    let resulting_bitboard = king_move_bitboard & *enemey_king_bitboard;
    resulting_bitboard.is_not_empty()
}

fn get_kings_square(game_state: &GameState, color: PieceColor) -> Square {
    let king = Piece::new(PieceType::King, color);
    let king_bitboard = game_state.bit_board_manager().get_bitboard(king);
    let king_square = king_bitboard.get_all_true_squares();
    king_square[0]
}
