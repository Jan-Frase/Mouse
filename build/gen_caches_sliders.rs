use crate::gen_util::{is_square_valid, square_to_file, square_to_rank};

pub const PEXT_TABLE_SIZE: usize = 107_648;

// Made with: https://tearth.dev/bitboard-viewer
const EDGE_OF_BOARD_MASK: u64 =
    0b11111111_10000001_10000001_10000001_10000001_10000001_10000001_11111111;

const LEFT_SIDE_MASK: u64 = 282578800148736;

const RIGHT_SIDE_MASK: u64 = 36170086419038208;

const TOP_SIDE_MASK: u64 = 9079256848778919936;

const BOTTOM_SIDE_MASK: u64 = 126;

// ----------------------------------------
// PEXT GEN LOGIC
// ----------------------------------------
// https://lorentzvedeler.com/2025/03/03/Pext-Tables/

enum Piece {
    Rook,
    Bishop,
    Queen,
}

pub struct PextData {
    pub rook_pext_mask: [u64; 64],
    pub rook_pext_index: [usize; 64],
    pub bishop_pext_mask: [u64; 64],
    pub bishop_pext_index: [usize; 64],
    pub pext_table: [u64; PEXT_TABLE_SIZE],
}

pub fn gen_cache_sliders() -> PextData {
    let mut rook_pext_mask = [0; 64];
    let mut rook_pext_index = [0usize; 64];

    let mut bishop_pext_mask = [0; 64];
    let mut bishop_pext_index = [0usize; 64];

    let mut pext_table = [0; 107_648];
    let mut current_pext_table_index = 0;

    gen_for_piece(
        &Piece::Rook,
        &mut rook_pext_mask,
        &mut rook_pext_index,
        &mut pext_table,
        &mut current_pext_table_index,
    );

    gen_for_piece(
        &Piece::Bishop,
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
    piece: &Piece,
    piece_pext_mask: &mut [u64; 64],
    piece_pext_index: &mut [usize; 64],
    pext_table: &mut [u64; PEXT_TABLE_SIZE],
    current_pext_table_index: &mut usize,
) {
    for square in 0..64 {
        piece_pext_index[square as usize] = *current_pext_table_index;

        let mut relevant_squares = calculate_slider_move_bitboard(piece, square, 0);
        relevant_squares = relevant_squares & !adjust_edge_of_board(square);
        piece_pext_mask[square as usize] = relevant_squares;

        let amount_of_blocker_squares = relevant_squares.count_ones();
        let amount_of_possible_blocker_configurations = 2u64.pow(amount_of_blocker_squares);
        // Iterate over all possible permutations of blocker configurations
        let mut blocker_config_index = 0;
        while blocker_config_index < amount_of_possible_blocker_configurations {
            let blockers: u64 = pdep64(blocker_config_index, relevant_squares);

            let moves_bb = calculate_slider_move_bitboard(piece, square, blockers);
            pext_table[*current_pext_table_index] = moves_bb;
            *current_pext_table_index += 1;

            blocker_config_index += 1;
        }
    }
}

fn adjust_edge_of_board(square: i8) -> u64 {
    let rank = square_to_rank(square);
    let file = square_to_file(square);

    let mut adjustment_mask = 0u64;
    if file == 0 {
        adjustment_mask |= LEFT_SIDE_MASK;
    }
    if file == 7 {
        adjustment_mask |= RIGHT_SIDE_MASK;
    }
    if rank == 0 {
        adjustment_mask |= BOTTOM_SIDE_MASK;
    }
    if rank == 7 {
        adjustment_mask |= TOP_SIDE_MASK;
    }

    adjustment_mask = EDGE_OF_BOARD_MASK & !adjustment_mask;
    adjustment_mask
}

/// this exists to provide a const alternative to the pdep intrinsic.
/// its implementations is based on:
/// https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_pdep_u64&ig_expand=490
fn pdep64(word: u64, mask: u64) -> u64 {
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
    fn next(&self, file: i8, rank: i8) -> (i8, i8) {
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

fn calculate_slider_move_bitboard(piece_type: &Piece, square: i8, blocker_bb: u64) -> u64 {
    let mut move_bitboard = 0u64;

    let mut i = 0;
    match piece_type {
        Piece::Rook => {
            while i < 4 {
                move_bitboard |= calculate_max_slide_range(square, &ROOK_DIR[i], blocker_bb);
                i += 1;
            }
        }
        Piece::Bishop => {
            while i < 4 {
                move_bitboard |= calculate_max_slide_range(square, &BISHOP_DIR[i], blocker_bb);
                i += 1;
            }
        }
        Piece::Queen => {
            while i < 8 {
                move_bitboard |= calculate_max_slide_range(square, &QUEEN_DIR[i], blocker_bb);
                i += 1;
            }
        }
        _ => unreachable!(),
    }

    move_bitboard
}

/// Computes a bitboard containing all squares
/// that the piece on the given square can slide to in the given direction
fn calculate_max_slide_range(square: i8, direction: &SlideDirection, blocker_bb: u64) -> u64 {
    let mut result = 0u64;
    let rank = square_to_rank(square);
    let file = square_to_file(square);
    let mut next = direction.next(file, rank);

    while is_square_valid(next.1, next.0) {
        let mut bb = 0u64;
        bb = 1 << (next.1 * 8 + next.0);
        result |= bb;
        if blocker_bb & bb != 0 {
            return result;
        }
        next = direction.next(next.0, next.1);
    }
    result
}
