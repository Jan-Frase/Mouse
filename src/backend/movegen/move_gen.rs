use crate::backend::movegen::compile_time::move_cache_non_sliders::{
    KING_MOVES, KNIGHT_MOVES, PAWN_CAPTURE_MOVES, PAWN_QUIET_MOVES,
};
use crate::backend::movegen::moove::Moove;
use crate::backend::movegen::move_gen_non_sliders::get_moves_for_non_slider_piece;
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::board::bitboard_manager::BitBoardManager;
use crate::backend::state::game::game_state::GameState;
use crate::backend::state::piece::PieceType::{King, Knight, Pawn};
use crate::backend::state::piece::{Piece, PieceColor, PieceType};
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
    let enemy_pieces_bitboard =
        bitboard_manager.get_all_pieces_off(game_state.active_color().opposite());

    let mut all_pseudo_legal_moves = Vec::new();
    let active_color = game_state.active_color();

    // King moves
    get_moves_for_trivial_piece(
        &mut all_pseudo_legal_moves,
        King,
        active_color,
        KING_MOVES,
        bitboard_manager,
        friendly_pieces_bitboard,
    );

    // Rook moves
    get_moves_for_trivial_piece(
        &mut all_pseudo_legal_moves,
        Knight,
        active_color,
        KNIGHT_MOVES,
        bitboard_manager,
        friendly_pieces_bitboard,
    );

    let mut pawn_quiet_mask = friendly_pieces_bitboard | enemy_pieces_bitboard;
    let pawn_bitboard = game_state
        .bit_board_manager()
        .get_bitboard(Piece::new(Pawn, active_color));
    pawn_quiet_mask &= !*pawn_bitboard;
    pawn_quiet_mask.copy_double_pawn_push_rank_one_forward(game_state.active_color());
    // Quiet pawn moves
    get_moves_for_trivial_piece(
        &mut all_pseudo_legal_moves,
        Pawn,
        active_color,
        PAWN_QUIET_MOVES[active_color as usize],
        bitboard_manager,
        // pawns cant capture forward so we need to mask for friendly and enemy pieces
        pawn_quiet_mask,
    );

    let mut pawn_capture_mask = enemy_pieces_bitboard;
    match game_state
        .irreversible_data_stack()
        .last()
        .unwrap()
        .en_passant_square()
    {
        None => {}
        Some(ep_square) => {
            pawn_capture_mask.fill_square(ep_square);
        }
    }
    pawn_capture_mask = !pawn_capture_mask;

    // Capture pawn moves
    get_moves_for_trivial_piece(
        &mut all_pseudo_legal_moves,
        Pawn,
        active_color,
        PAWN_CAPTURE_MOVES[active_color as usize],
        bitboard_manager,
        pawn_capture_mask,
    );

    all_pseudo_legal_moves
}

/// Gets all pseudo legal moves for one piece.
fn get_moves_for_trivial_piece(
    all_pseudo_legal_moves: &mut Vec<Moove>,
    piece_type: PieceType,
    active_color: PieceColor,
    moves_cache: [BitBoard; SQUARES_AMOUNT],
    bitboard_manager: &BitBoardManager,
    mask_bitboard: BitBoard,
) {
    let piece = Piece::new(piece_type, active_color);
    let piece_bitboard = bitboard_manager.get_bitboard(piece);
    let mut moves = get_moves_for_non_slider_piece(moves_cache, *piece_bitboard, mask_bitboard);
    all_pseudo_legal_moves.append(&mut moves);
}
