use crate::backend::movegen::move_gen_sliders::calculate_slider_move_bitboard;
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::piece::Piece;
use crate::backend::state::square::Square;
use std::arch::x86_64::_pdep_u64;

// Made with: https://tearth.dev/bitboard-viewer
const EDGE_OF_BOARD_MASK: BitBoard = BitBoard {
    value: 18411139144890810879,
};

const LEFT_SIDE_MASK: BitBoard = BitBoard {
    value: 282578800148736,
};

const RIGHT_SIDE_MASK: BitBoard = BitBoard {
    value: 36170086419038208,
};

const TOP_SIDE_MASK: BitBoard = BitBoard {
    value: 9079256848778919936,
};

const BOTTOM_SIDE_MASK: BitBoard = BitBoard { value: 126 };

pub struct PextData {
    pub rook_pext_mask: [BitBoard; 64],
    pub rook_pext_index: [usize; 64],
    pub bishop_pext_mask: [BitBoard; 64],
    pub bishop_pext_index: [usize; 64],
    pub pext_table: [BitBoard; 107_648],
}

pub fn gen_cache_sliders() -> PextData {
    let mut rook_pext_mask = [BitBoard::new(); 64];
    let mut rook_pext_index = [0usize; 64];

    let mut bishop_pext_mask = [BitBoard::new(); 64];
    let mut bishop_pext_index = [0usize; 64];

    let mut pext_table = [BitBoard::new(); 107_648];
    let mut current_pext_table_index = 0;

    gen_for_piece(
        Piece::Rook,
        &mut rook_pext_mask,
        &mut rook_pext_index,
        &mut pext_table,
        &mut current_pext_table_index,
    );

    gen_for_piece(
        Piece::Bishop,
        &mut bishop_pext_mask,
        &mut bishop_pext_index,
        &mut pext_table,
        &mut current_pext_table_index,
    );

    PextData {
        rook_pext_mask,
        rook_pext_index,
        bishop_pext_mask,
        bishop_pext_index,
        pext_table,
    }
}

fn gen_for_piece(
    piece: Piece,
    piece_pext_mask: &mut [BitBoard; 64],
    piece_pext_index: &mut [usize; 64],
    pext_table: &mut [BitBoard; 107648],
    current_pext_table_index: &mut usize,
) {
    for square_index in 0..64 {
        let square = Square::index_to_square(square_index as i8);

        piece_pext_index[square_index] = *current_pext_table_index;

        let relevant_squares =
            calculate_slider_move_bitboard(piece, square, BitBoard::new(), BitBoard::new());
        let relevant_squares = relevant_squares & !adjust_edge_of_board(square);
        piece_pext_mask[square_index] = relevant_squares;

        let amount_of_blocker_squares = relevant_squares.value.count_ones();
        let amount_of_possible_blocker_configurations = 2u64.pow(amount_of_blocker_squares);
        // Iterate over all possible permutations of blocker configurations
        for blocker_config_index in 0..amount_of_possible_blocker_configurations {
            let blockers: u64 = unsafe { _pdep_u64(blocker_config_index, relevant_squares.value) };

            let moves_bb = calculate_slider_move_bitboard(
                piece,
                square,
                BitBoard::new(),
                BitBoard { value: blockers },
            );
            pext_table[*current_pext_table_index] = moves_bb;
            *current_pext_table_index += 1;
        }
    }
}

fn adjust_edge_of_board(square: Square) -> BitBoard {
    let mut adjustment_mask = BitBoard::new();
    if square.file() == 0 {
        adjustment_mask |= LEFT_SIDE_MASK;
    }
    if square.file() == 7 {
        adjustment_mask |= RIGHT_SIDE_MASK;
    }
    if square.rank() == 0 {
        adjustment_mask |= BOTTOM_SIDE_MASK;
    }
    if square.rank() == 7 {
        adjustment_mask |= TOP_SIDE_MASK;
    }
    EDGE_OF_BOARD_MASK & !adjustment_mask
}
