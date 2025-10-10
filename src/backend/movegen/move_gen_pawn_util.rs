use crate::backend::movegen::compile_time::move_cache_non_sliders::{
    PAWN_CAPTURE_MOVES, PAWN_DOUBLE_PUSH_MOVES, PAWN_QUIET_MOVES,
};
use crate::backend::movegen::moove::Moove;
use crate::backend::state::board::bb_manager::BBManager;
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::game::state::State;
use crate::backend::state::piece::PieceType::{Pawn, Queen};
use crate::backend::state::piece::{Piece, PieceColor, PieceType};

pub fn gen_pawn_moves(
    game_state: &State,
    bitboard_manager: &BBManager,
    friendly_pieces_bb: BitBoard,
    enemy_pieces_bb: BitBoard,
    all_pseudo_legal_moves: &mut Vec<Moove>,
    active_color: PieceColor,
) {
    // Quiet pawn moves
    let mut moves = crate::backend::movegen::move_gen::iterate_over_bitboard_for_non_slider(
        PAWN_QUIET_MOVES[active_color as usize],
        *bitboard_manager.get_bitboard(Piece::new(Pawn, active_color)),
        friendly_pieces_bb | enemy_pieces_bb,
    );
    promotion_logic(&mut moves);
    all_pseudo_legal_moves.append(&mut moves);

    // Capture pawn moves
    let mut moves = crate::backend::movegen::move_gen::iterate_over_bitboard_for_non_slider(
        PAWN_CAPTURE_MOVES[active_color as usize],
        *bitboard_manager.get_bitboard(Piece::new(Pawn, active_color)),
        create_pawn_capture_mask(game_state, enemy_pieces_bb),
    );
    promotion_logic(&mut moves);
    all_pseudo_legal_moves.append(&mut moves);

    // Double pawn push moves
    let mut moves = get_double_pawn_push_moves(
        bitboard_manager,
        active_color,
        friendly_pieces_bb | enemy_pieces_bb,
    );
    all_pseudo_legal_moves.append(&mut moves);
}
pub fn get_double_pawn_push_moves(
    bitboard_manager: &BBManager,
    active_color: PieceColor,
    all_pieces_bb: BitBoard,
) -> Vec<Moove> {
    let mut moves: Vec<Moove> = Vec::new();

    let mut pawn_bitboard = *bitboard_manager.get_bitboard(Piece::new(Pawn, active_color));

    let starting_bitboard = match active_color {
        PieceColor::White => BitBoard::new_from_rank(1),
        PieceColor::Black => BitBoard::new_from_rank(6),
    };

    pawn_bitboard &= starting_bitboard;

    for square in pawn_bitboard {
        let mut single_push_bb = PAWN_QUIET_MOVES[active_color as usize][square.square_to_index()];
        single_push_bb &= all_pieces_bb;
        if !single_push_bb.is_empty() {
            continue;
        }

        let mut double_push_bb =
            PAWN_DOUBLE_PUSH_MOVES[active_color as usize][square.square_to_index()];
        double_push_bb &= all_pieces_bb;
        if !double_push_bb.is_empty() {
            continue;
        }

        let moove = Moove::new(
            square,
            square
                .forward_by_one(active_color)
                .forward_by_one(active_color),
        );
        moves.push(moove);
    }

    moves
}

pub fn create_pawn_capture_mask(game_state: &State, enemy_pieces_bitboard: BitBoard) -> BitBoard {
    // Capture pawn moves
    let mut pawn_capture_mask = enemy_pieces_bitboard;
    match game_state.irreversible_data().en_passant_square() {
        None => {}
        Some(ep_square) => {
            pawn_capture_mask.fill_square(ep_square);
        }
    }
    pawn_capture_mask = !pawn_capture_mask;
    pawn_capture_mask
}

pub fn promotion_logic(moves: &mut Vec<Moove>) {
    if moves.is_empty() {
        return;
    }
    for index in (0..moves.len()).rev() {
        let moove = moves[index];
        if moove.to().is_on_promotion_rank() {
            for piece_type in PieceType::get_promotable_types() {
                if piece_type == Queen {
                    moves[index].set_promotion_type(Some(Queen));
                    continue;
                }
                let mut moove = moove;
                moove.set_promotion_type(Some(piece_type));
                moves.push(moove);
            }
        }
    }
}
