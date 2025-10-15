use crate::backend::compile_time::gen_caches::PAWN_CAPTURE_MOVES;
use crate::backend::movegen::moove::Moove;
use crate::backend::movegen::move_gen::iterate_over_bitboard_for_non_slider;
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::game::state::State;
use crate::backend::state::piece::Piece::{Pawn, Queen};
use crate::backend::state::piece::{PROMOTABLE_PIECES, Side};

const WHITE_PAWN_START_RANK_BB: BitBoard = BitBoard::new_from_rank(1);
const BLACK_PAWN_START_RANK_BB: BitBoard = BitBoard::new_from_rank(6);

pub fn gen_pawn_moves(
    moves: &mut Vec<Moove>,
    state: &State,
    friendly_pieces_bb: BitBoard,
    enemy_pieces_bb: BitBoard,
    active_color: Side,
) {
    let occupancy_bb = friendly_pieces_bb | enemy_pieces_bb;
    let pawn_bb = state.bb_manager.get_colored_piece_bb(Pawn, active_color);

    // single push
    let mut push_pawn_bb = match active_color {
        Side::White => pawn_bb << 8,
        Side::Black => pawn_bb >> 8,
    };
    push_pawn_bb &= !occupancy_bb;
    for square in push_pawn_bb {
        let moove = Moove::new(square.back_by_one(active_color), square);
        if square.is_on_promotion_rank() {
            for piece_type in PROMOTABLE_PIECES {
                let mut promotion_move = moove;
                promotion_move.promotion_type = Some(piece_type);
                moves.push(promotion_move);
            }
        } else {
            moves.push(moove);
        }
    }

    // double push
    let double_push_pawn_bb = match active_color {
        Side::White => {
            (((pawn_bb & WHITE_PAWN_START_RANK_BB) << 8) & !occupancy_bb) << 8 & !occupancy_bb
        }
        Side::Black => {
            (((pawn_bb & BLACK_PAWN_START_RANK_BB) >> 8) & !occupancy_bb) >> 8 & !occupancy_bb
        }
    };
    for square in double_push_pawn_bb {
        let moove = Moove::new(
            square.back_by_one(active_color).back_by_one(active_color),
            square,
        );
        moves.push(moove);
    }

    let mut pawn_moves = Vec::new();
    // Capture pawn moves
    iterate_over_bitboard_for_non_slider(
        &mut pawn_moves,
        PAWN_CAPTURE_MOVES[active_color as usize],
        state.bb_manager.get_colored_piece_bb(Pawn, active_color),
        create_pawn_capture_mask(state, enemy_pieces_bb),
    );
    promotion_logic(&mut pawn_moves);
    moves.append(&mut pawn_moves);
}

fn create_pawn_capture_mask(game_state: &State, enemy_pieces_bitboard: BitBoard) -> BitBoard {
    // Capture pawn moves
    let mut pawn_capture_mask = enemy_pieces_bitboard;
    match game_state.irreversible_data.en_passant_square {
        None => {}
        Some(ep_square) => {
            pawn_capture_mask.fill_square(ep_square);
        }
    }
    pawn_capture_mask = !pawn_capture_mask;
    pawn_capture_mask
}

fn promotion_logic(moves: &mut Vec<Moove>) {
    if moves.is_empty() {
        return;
    }
    for index in (0..moves.len()).rev() {
        let moove = moves[index];
        if moove.to.is_on_promotion_rank() {
            for piece_type in PROMOTABLE_PIECES {
                if piece_type == Queen {
                    moves[index].promotion_type = Some(Queen);
                    continue;
                }
                let mut moove = moove;
                moove.promotion_type = Some(piece_type);
                moves.push(moove);
            }
        }
    }
}
