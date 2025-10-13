use crate::backend::compile_time::gen_caches::{
    BISHOP_PEXT_INDEX, BISHOP_PEXT_MASK, PEXT_TABLE, ROOK_PEXT_INDEX, ROOK_PEXT_MASK,
};
use crate::backend::movegen::moove::Moove;
use crate::backend::movegen::move_gen::convert_bitboard_to_moves;
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::piece::Piece;
use crate::backend::state::square::Square;
use std::arch::x86_64::_pext_u64;

pub fn get_slider_moves(
    moves: &mut Vec<Moove>,
    piece_type: Piece,
    piece_bb: BitBoard,
    friendly_pieces_bb: BitBoard,
    enemy_pieces_bb: BitBoard,
) {
    for square in piece_bb {
        let moves_for_piece_bb =
            get_slider_moves_at_square(piece_type, square, friendly_pieces_bb, enemy_pieces_bb);

        moves.append(&mut convert_bitboard_to_moves(square, moves_for_piece_bb));
    }
}

pub fn get_slider_moves_at_square(
    piece: Piece,
    square: Square,
    friendly_bb: BitBoard,
    enemy_bb: BitBoard,
) -> BitBoard {
    match piece {
        Piece::Rook => get_rook_moves_at_square(square, friendly_bb, enemy_bb),
        Piece::Bishop => get_bishop_moves_at_square(square, friendly_bb, enemy_bb),
        Piece::Queen => {
            get_rook_moves_at_square(square, friendly_bb, enemy_bb)
                | get_bishop_moves_at_square(square, friendly_bb, enemy_bb)
        }
        _ => panic!("Piece type is not a slider"),
    }
}

fn get_rook_moves_at_square(square: Square, friendly_bb: BitBoard, enemy_bb: BitBoard) -> BitBoard {
    let pext_mask = ROOK_PEXT_MASK[square.square_to_index()];
    let pext_index = ROOK_PEXT_INDEX[square.square_to_index()];

    let occ_bb = friendly_bb | enemy_bb;

    let blockers_index: usize = unsafe { _pext_u64(occ_bb.value, pext_mask.value) as usize };

    let moves = PEXT_TABLE[pext_index + blockers_index];
    moves & !friendly_bb
}

fn get_bishop_moves_at_square(
    square: Square,
    friendly_bb: BitBoard,
    enemy_bb: BitBoard,
) -> BitBoard {
    let pext_mask = BISHOP_PEXT_MASK[square.square_to_index()];
    let pext_index = BISHOP_PEXT_INDEX[square.square_to_index()];

    let occ_bb = friendly_bb | enemy_bb;

    let blockers_index: usize = unsafe { _pext_u64(occ_bb.value, pext_mask.value) as usize };

    let moves = PEXT_TABLE[pext_index + blockers_index];
    moves & !friendly_bb
}
