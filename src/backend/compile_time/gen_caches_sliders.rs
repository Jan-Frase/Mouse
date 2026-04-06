use crate::backend::compile_time::gen_util::is_square_valid;
use crate::backend::constants::{A1, SQUARES_AMOUNT};
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::piece::Piece;
use crate::backend::state::square::{Square, get_file, get_rank};

pub const PEXT_TABLE_SIZE: usize = 107_648;

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

// ----------------------------------------
// PEXT GEN LOGIC
// ----------------------------------------
// https://lorentzvedeler.com/2025/03/03/Pext-Tables/

pub struct PextData {
    pub rook_pext_mask: [BitBoard; SQUARES_AMOUNT],
    pub rook_pext_index: [usize; SQUARES_AMOUNT],
    pub bishop_pext_mask: [BitBoard; SQUARES_AMOUNT],
    pub bishop_pext_index: [usize; SQUARES_AMOUNT],
    pub pext_table: [BitBoard; PEXT_TABLE_SIZE],
}

pub const fn gen_cache_sliders() -> PextData {
    let mut rook_pext_mask = [BitBoard::new(); SQUARES_AMOUNT];
    let mut rook_pext_index = [0usize; SQUARES_AMOUNT];

    let mut bishop_pext_mask = [BitBoard::new(); SQUARES_AMOUNT];
    let mut bishop_pext_index = [0usize; SQUARES_AMOUNT];

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

const fn gen_for_piece(
    piece: Piece,
    piece_pext_mask: &mut [BitBoard; SQUARES_AMOUNT],
    piece_pext_index: &mut [usize; SQUARES_AMOUNT],
    pext_table: &mut [BitBoard; PEXT_TABLE_SIZE],
    current_pext_table_index: &mut usize,
) {
    let mut square: Square = A1;
    while square < SQUARES_AMOUNT as u8 {
        piece_pext_index[square as usize] = *current_pext_table_index;

        let mut relevant_squares = calculate_slider_move_bitboard(piece, square, BitBoard::new());
        relevant_squares.value = relevant_squares.value & !adjust_edge_of_board(square).value;
        piece_pext_mask[square as usize] = relevant_squares;

        let amount_of_blocker_squares = relevant_squares.value.count_ones();
        let amount_of_possible_blocker_configurations = 2u64.pow(amount_of_blocker_squares);
        // Iterate over all possible permutations of blocker configurations
        let mut blocker_config_index = 0;
        while blocker_config_index < amount_of_possible_blocker_configurations {
            let blockers: u64 = pdep64(blocker_config_index, relevant_squares.value);

            let moves_bb =
                calculate_slider_move_bitboard(piece, square, BitBoard { value: blockers });
            pext_table[*current_pext_table_index] = moves_bb;
            *current_pext_table_index += 1;

            blocker_config_index += 1;
        }

        square += 1;
    }
}

const fn adjust_edge_of_board(square: Square) -> BitBoard {
    let file = get_file(square);
    let rank = get_rank(square);

    let mut adjustment_mask = BitBoard::new();
    if file == 0 {
        adjustment_mask.value |= LEFT_SIDE_MASK.value;
    }
    if file == 7 {
        adjustment_mask.value |= RIGHT_SIDE_MASK.value;
    }
    if rank == 0 {
        adjustment_mask.value |= BOTTOM_SIDE_MASK.value;
    }
    if rank == 7 {
        adjustment_mask.value |= TOP_SIDE_MASK.value;
    }

    adjustment_mask.value = EDGE_OF_BOARD_MASK.value & !adjustment_mask.value;
    adjustment_mask
}

/// this exists to provide a const alternative to the pdep intrinsic.
/// its implementations is based on:
/// https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_pdep_u64&ig_expand=490
const fn pdep64(word: u64, mask: u64) -> u64 {
    let mut out = 0;
    let mut input_idx = 0;

    let mut i = 0;
    while i < 64 {
        let ith_mask_bit = (mask >> i) & 1;
        if ith_mask_bit == 1 {
            let next_word_bit = (word >> input_idx) & 1;
            out |= next_word_bit << i;
            input_idx += 1;
        }

        i += 1;
    }

    out
}

// ----------------------------------------
// NORMAL SLIDER MOVE GEN
// ----------------------------------------

enum SlideDirection {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl SlideDirection {
    const fn next(&self, file: i8, rank: i8) -> (i8, i8) {
        match self {
            SlideDirection::Up => (file, rank + 1),
            SlideDirection::UpRight => (file + 1, rank + 1),
            SlideDirection::Right => (file + 1, rank),
            SlideDirection::DownRight => (file + 1, rank - 1),
            SlideDirection::Down => (file, rank - 1),
            SlideDirection::DownLeft => (file - 1, rank - 1),
            SlideDirection::Left => (file - 1, rank),
            SlideDirection::UpLeft => (file - 1, rank + 1),
        }
    }
}

const ROOK_DIR: [SlideDirection; 4] = [
    SlideDirection::Up,
    SlideDirection::Down,
    SlideDirection::Left,
    SlideDirection::Right,
];
const BISHOP_DIR: [SlideDirection; 4] = [
    SlideDirection::UpRight,
    SlideDirection::DownRight,
    SlideDirection::DownLeft,
    SlideDirection::UpLeft,
];
const QUEEN_DIR: [SlideDirection; 8] = [
    SlideDirection::Up,
    SlideDirection::UpRight,
    SlideDirection::Right,
    SlideDirection::DownRight,
    SlideDirection::Down,
    SlideDirection::DownLeft,
    SlideDirection::Left,
    SlideDirection::UpLeft,
];

const fn calculate_slider_move_bitboard(
    piece_type: Piece,
    square: Square,
    blocker_bb: BitBoard,
) -> BitBoard {
    let mut move_bitboard: BitBoard = BitBoard::new();

    let mut i = 0;
    match piece_type {
        Piece::Rook => {
            while i < 4 {
                move_bitboard.value |= calculate_max_slide_range(square, &ROOK_DIR[i], blocker_bb);
                i += 1;
            }
        }
        Piece::Bishop => {
            while i < 4 {
                move_bitboard.value |=
                    calculate_max_slide_range(square, &BISHOP_DIR[i], blocker_bb);
                i += 1;
            }
        }
        Piece::Queen => {
            while i < 8 {
                move_bitboard.value |= calculate_max_slide_range(square, &QUEEN_DIR[i], blocker_bb);
                i += 1;
            }
        }
        _ => unreachable!(),
    }

    move_bitboard
}

/// Computes a bitboard containing all squares
/// that the piece on the given square can slide to in the given direction
const fn calculate_max_slide_range(
    square: Square,
    direction: &SlideDirection,
    blocker_bb: BitBoard,
) -> u64 {
    let mut result = BitBoard::new();
    let file = get_file(square);
    let rank = get_rank(square);
    let mut next = direction.next(file, rank);

    while is_square_valid(next.1, next.0) {
        let mut bb = BitBoard::new();
        bb.value = 1 << (next.1 * 8 + next.0);
        result.value |= bb.value;
        if blocker_bb.value & bb.value != 0 {
            return result.value;
        }
        next = direction.next(next.0, next.1);
    }
    result.value
}
