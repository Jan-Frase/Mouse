use crate::backend::caches::{
    BISHOP_PEXT_INDEX, BISHOP_PEXT_MASK, PEXT_TABLE, ROOK_PEXT_INDEX, ROOK_PEXT_MASK,
};
use crate::backend::types::moove::Moove;
use crate::backend::movegen::move_gen::convert_bitboard_to_moves;
use crate::backend::types::bitboard::BitBoard;
use crate::backend::types::piece::Piece;
use crate::backend::types::square::Square;
use std::arch::x86_64::_pext_u64;

pub fn get_slider_moves(
    moves: &mut Vec<Moove>,
    piece_type: Piece,
    piece_bb: BitBoard,
    friendly_pieces_bb: BitBoard,
    enemy_pieces_bb: BitBoard,
    checkmask: BitBoard
) {
    for square in piece_bb {
        let mut moves_for_piece_bb = match piece_type {
            Piece::Rook => get_slider_moves_at_square::<true>(square, friendly_pieces_bb, enemy_pieces_bb),
            Piece::Bishop => get_slider_moves_at_square::<false>(square, friendly_pieces_bb, enemy_pieces_bb),
            Piece::Queen => {
                get_slider_moves_at_square::<true>(square, friendly_pieces_bb, enemy_pieces_bb)
                    | get_slider_moves_at_square::<false>(square, friendly_pieces_bb, enemy_pieces_bb)
            }
            _ => panic!("Piece type is not a slider"),
        };
        
        moves_for_piece_bb &= checkmask;

        convert_bitboard_to_moves(moves, square, moves_for_piece_bb);
    }
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

    let moves = PEXT_TABLE[pext_index + blockers_index];
    moves & !friendly_bb
}
