use crate::backend::caches::{BISHOP_PEXT_INDEX, BISHOP_PEXT_MASK, BISHOP_XRAY_PEXT_INDEX, BISHOP_XRAY_PEXT_MASK, PEXT_TABLE, ROOK_PEXT_INDEX, ROOK_PEXT_MASK, ROOK_XRAY_PEXT_INDEX, ROOK_XRAY_PEXT_MASK, XRAY_PEXT_TABLE};
use crate::backend::types::moove::Moove;
use crate::backend::movegen::move_gen::convert_bitboard_to_moves;
use crate::backend::types::bitboard::BitBoard;
use crate::backend::types::piece::Piece;
use crate::backend::types::square::Square;
use std::arch::x86_64::_pext_u64;
use std::hint::unreachable_unchecked;
use crate::backend::types::piece::Piece::Queen;

pub fn get_slider_moves(
    moves: &mut Vec<Moove>,
    piece_type: Piece,
    piece_bb: BitBoard,
    friendly_pieces_bb: BitBoard,
    enemy_pieces_bb: BitBoard,
    checkmask: BitBoard,
    pin_mask: BitBoard
) {
    // pieces that are not pinned
    for square in piece_bb & !pin_mask {
        let mut moves_for_piece_bb = calc_seen_squares(piece_type, friendly_pieces_bb, enemy_pieces_bb, square);
        moves_for_piece_bb &= checkmask;
        convert_bitboard_to_moves(moves, square, moves_for_piece_bb);
    }

    for square in piece_bb & pin_mask {
        let mut moves_for_piece_bb = calc_seen_squares(piece_type, friendly_pieces_bb, enemy_pieces_bb, square);
        moves_for_piece_bb &= checkmask & pin_mask;
        convert_bitboard_to_moves(moves, square, moves_for_piece_bb);
    }
}

fn calc_seen_squares(piece_type: Piece, friendly_pieces_bb: BitBoard, enemy_pieces_bb: BitBoard, square: Square) -> BitBoard {
    match piece_type {
        Piece::Rook => get_slider_moves_at_square::<true>(square, friendly_pieces_bb, enemy_pieces_bb),
        Piece::Bishop => get_slider_moves_at_square::<false>(square, friendly_pieces_bb, enemy_pieces_bb),
        Piece::Queen => get_slider_moves_at_square::<true>(square, friendly_pieces_bb, enemy_pieces_bb) | get_slider_moves_at_square::<false>(square, friendly_pieces_bb, enemy_pieces_bb),
        _ => unsafe {unreachable_unchecked()}
    }
}

pub fn get_slider_xray_moves_at_square<const IS_STRAIGHT: bool>(square: Square, occ_bb: BitBoard) -> BitBoard {
    let pext_mask = match IS_STRAIGHT {
        true => ROOK_XRAY_PEXT_MASK[square as usize],
        false => BISHOP_XRAY_PEXT_MASK[square as usize],
    };

    let pext_index = match IS_STRAIGHT {
        true => ROOK_XRAY_PEXT_INDEX[square as usize],
        false => BISHOP_XRAY_PEXT_INDEX[square as usize],
    };

    let blockers_index: usize = unsafe { _pext_u64(occ_bb.value, pext_mask.value) as usize };

    XRAY_PEXT_TABLE[pext_index + blockers_index]
}

/// Computes the sliding piece moves (either rook-like or bishop-like)
/// for a given square on the chessboard based on occupancy bitboards.
///
/// # Type Parameters
/// - IS_STRAIGHT:
///   - True for rook-like (horizontal and vertical) moves.
///   - False for bishop-like (diagonal) moves.
///
/// # Returns
/// - BitBoard
///   A bitboard representing all legal moves for the sliding piece from the given
///   square. This excludes moves that are blocked by friendly pieces.
pub fn get_slider_moves_at_square<const IS_STRAIGHT: bool>(square: Square, friendly_bb: BitBoard, enemy_bb: BitBoard) -> BitBoard {
    let pext_mask = match IS_STRAIGHT {
        true => ROOK_PEXT_MASK[square as usize],
        false => BISHOP_PEXT_MASK[square as usize],
    };

    let pext_index = match IS_STRAIGHT {
        true => ROOK_PEXT_INDEX[square as usize],
        false => BISHOP_PEXT_INDEX[square as usize],
    };

    let occ_bb = friendly_bb | enemy_bb;
    let blockers_index: usize = unsafe { _pext_u64(occ_bb.value, pext_mask.value) as usize };

    PEXT_TABLE[pext_index + blockers_index] & !friendly_bb
}

