use crate::backend::movegen::moove::Moove;
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::game::state::State;
use crate::backend::state::piece::Piece::Pawn;
use crate::backend::state::piece::{PROMOTABLE_PIECES, Side};
use crate::backend::state::square::Square;

const BLACK_PROMOTION_RANK_BB: BitBoard = BitBoard::new_from_rank(0);
const WHITE_PROMOTION_RANK_BB: BitBoard = BitBoard::new_from_rank(7);
const WHITE_PAWN_START_RANK_BB: BitBoard = BitBoard::new_from_rank(1);
const BLACK_PAWN_START_RANK_BB: BitBoard = BitBoard::new_from_rank(6);
const PROMOTION_RANKS_BB: BitBoard = BitBoard {
    value: (BLACK_PROMOTION_RANK_BB.value | WHITE_PROMOTION_RANK_BB.value),
};
const LEFT_SIDE_BB: BitBoard = BitBoard::new_from_file(0);
const RIGHT_SIDE_BB: BitBoard = BitBoard::new_from_file(7);

pub fn gen_pawn_moves(
    moves: &mut Vec<Moove>,
    state: &State,
    friendly_pieces_bb: BitBoard,
    enemy_pieces_bb: BitBoard,
    active_color: Side,
) {
    let occupancy_bb = friendly_pieces_bb | enemy_pieces_bb;
    let pawn_bb = state.bb_manager.get_colored_piece_bb(Pawn, active_color);

    let rank_offset = match active_color {
        Side::White => -1,
        Side::Black => 1,
    };

    // single push
    single_push(moves, active_color, occupancy_bb, pawn_bb, rank_offset);

    // double push
    double_push(moves, active_color, occupancy_bb, pawn_bb, rank_offset);

    let mut possible_captures_bb = enemy_pieces_bb;
    match state.irreversible_data.en_passant_square {
        None => {}
        Some(square) => {
            possible_captures_bb.fill_square(square);
        }
    }
    // left captures
    let shift = match active_color {
        Side::White => 7,
        Side::Black => -9,
    };
    one_dir_capture(
        moves,
        possible_captures_bb,
        pawn_bb,
        rank_offset,
        shift,
        LEFT_SIDE_BB,
        1,
    );

    // right captures
    let shift = match active_color {
        Side::White => 9,
        Side::Black => -7,
    };
    one_dir_capture(
        moves,
        possible_captures_bb,
        pawn_bb,
        rank_offset,
        shift,
        RIGHT_SIDE_BB,
        -1,
    );
}

fn single_push(
    moves: &mut Vec<Moove>,
    active_color: Side,
    occupancy_bb: BitBoard,
    pawn_bb: BitBoard,
    rank_offset: i8,
) {
    let mut push_pawn_bb = match active_color {
        Side::White => pawn_bb << 8,
        Side::Black => pawn_bb >> 8,
    };
    // cant go there if something is there
    push_pawn_bb &= !occupancy_bb;

    let no_promotion_push_pawn_bb = push_pawn_bb & !PROMOTION_RANKS_BB;
    pawn_bb_to_moves_no_promotion(moves, no_promotion_push_pawn_bb, 0, rank_offset);

    let promotion_push_pawn_bb = push_pawn_bb & PROMOTION_RANKS_BB;
    pawn_bb_to_moves_promotion(moves, promotion_push_pawn_bb, 0, rank_offset);
}

fn double_push(
    moves: &mut Vec<Moove>,
    active_color: Side,
    occupancy_bb: BitBoard,
    pawn_bb: BitBoard,
    rank_offset: i8,
) {
    let double_push_bb = match active_color {
        Side::White => {
            (((pawn_bb & WHITE_PAWN_START_RANK_BB) << 8) & !occupancy_bb) << 8 & !occupancy_bb
        }
        Side::Black => {
            (((pawn_bb & BLACK_PAWN_START_RANK_BB) >> 8) & !occupancy_bb) >> 8 & !occupancy_bb
        }
    };
    pawn_bb_to_moves_no_promotion(moves, double_push_bb, 0, 2 * rank_offset);
}

fn one_dir_capture(
    moves: &mut Vec<Moove>,
    enemy_pieces_bb: BitBoard,
    mut pawn_bb: BitBoard,
    rank_offset: i8,
    shift: i32,
    mask: BitBoard,
    file_offset: i8,
) {
    pawn_bb &= !mask;

    if shift.is_negative() {
        pawn_bb >>= shift.unsigned_abs();
    } else {
        pawn_bb <<= shift;
    }
    let capture_bb = pawn_bb & enemy_pieces_bb;

    let capture_no_promotion = capture_bb & !PROMOTION_RANKS_BB;
    pawn_bb_to_moves_no_promotion(moves, capture_no_promotion, file_offset, rank_offset);

    let captures_promotion = capture_bb & PROMOTION_RANKS_BB;
    pawn_bb_to_moves_promotion(moves, captures_promotion, file_offset, rank_offset);
}

fn pawn_bb_to_moves_no_promotion(
    moves: &mut Vec<Moove>,
    pawn_bb: BitBoard,
    file_offset: i8,
    rank_offset: i8,
) {
    for square in pawn_bb {
        let from_square = Square {
            file: square.file + file_offset,
            rank: square.rank + rank_offset,
        };
        let moove = Moove::new(from_square, square);
        moves.push(moove);
    }
}

fn pawn_bb_to_moves_promotion(
    moves: &mut Vec<Moove>,
    pawn_bb: BitBoard,
    file_offset: i8,
    rank_offset: i8,
) {
    for square in pawn_bb {
        let from_square = Square {
            file: square.file + file_offset,
            rank: square.rank + rank_offset,
        };
        for piece_type in PROMOTABLE_PIECES {
            let mut moove = Moove::new(from_square, square);
            moove.promotion_type = Some(piece_type);
            moves.push(moove);
        }
    }
}
