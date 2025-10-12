use crate::backend::constants::SQUARES_AMOUNT;
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::piece::Piece;
use crate::backend::state::square::Square;
use std::arch::x86_64::_pdep_u64;

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

pub fn gen_cache_sliders() -> PextData {
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

fn gen_for_piece(
    piece: Piece,
    piece_pext_mask: &mut [BitBoard; SQUARES_AMOUNT],
    piece_pext_index: &mut [usize; SQUARES_AMOUNT],
    pext_table: &mut [BitBoard; PEXT_TABLE_SIZE],
    current_pext_table_index: &mut usize,
) {
    for square_index in 0..64 {
        let square = Square::index_to_square(square_index as i8);

        piece_pext_index[square_index] = *current_pext_table_index;

        let relevant_squares = calculate_slider_move_bitboard(piece, square, BitBoard::new());
        let relevant_squares = relevant_squares & !adjust_edge_of_board(square);
        piece_pext_mask[square_index] = relevant_squares;

        let amount_of_blocker_squares = relevant_squares.value.count_ones();
        let amount_of_possible_blocker_configurations = 2u64.pow(amount_of_blocker_squares);
        // Iterate over all possible permutations of blocker configurations
        for blocker_config_index in 0..amount_of_possible_blocker_configurations {
            let blockers: u64 = unsafe { _pdep_u64(blocker_config_index, relevant_squares.value) };

            let moves_bb =
                calculate_slider_move_bitboard(piece, square, BitBoard { value: blockers });
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
    fn next(&self, square: Square) -> Square {
        match self {
            SlideDirection::Up => Square::new(square.file(), square.rank() + 1),
            SlideDirection::UpRight => Square::new(square.file() + 1, square.rank() + 1),
            SlideDirection::Right => Square::new(square.file() + 1, square.rank()),
            SlideDirection::DownRight => Square::new(square.file() + 1, square.rank() - 1),
            SlideDirection::Down => Square::new(square.file(), square.rank() - 1),
            SlideDirection::DownLeft => Square::new(square.file() - 1, square.rank() - 1),
            SlideDirection::Left => Square::new(square.file() - 1, square.rank()),
            SlideDirection::UpLeft => Square::new(square.file() - 1, square.rank() + 1),
        }
    }

    fn directions_for_piece_type(piece_type: Piece) -> Vec<SlideDirection> {
        match piece_type {
            Piece::Rook => {
                vec![
                    SlideDirection::Up,
                    SlideDirection::Down,
                    SlideDirection::Left,
                    SlideDirection::Right,
                ]
            }
            Piece::Bishop => {
                vec![
                    SlideDirection::UpRight,
                    SlideDirection::DownRight,
                    SlideDirection::DownLeft,
                    SlideDirection::UpLeft,
                ]
            }
            Piece::Queen => {
                vec![
                    SlideDirection::Up,
                    SlideDirection::Down,
                    SlideDirection::Left,
                    SlideDirection::Right,
                    SlideDirection::UpRight,
                    SlideDirection::DownRight,
                    SlideDirection::DownLeft,
                    SlideDirection::UpLeft,
                ]
            }
            _ => panic!("Piece type is not a slider"),
        }
    }
}

pub fn calculate_slider_move_bitboard(
    piece_type: Piece,
    square: Square,
    blocker_bb: BitBoard,
) -> BitBoard {
    let mut move_bitboard: BitBoard = BitBoard::new();
    for direction in SlideDirection::directions_for_piece_type(piece_type) {
        move_bitboard |= calculate_max_slide_range(square, direction, blocker_bb);
    }
    move_bitboard
}

/// Computes a bitboard containing all squares
/// that the piece on the given square can slide to in the given direction
fn calculate_max_slide_range(
    square: Square,
    direction: SlideDirection,
    blocker_bb: BitBoard,
) -> BitBoard {
    let mut result = BitBoard::new();
    let mut next = direction.next(square);
    while next.is_valid() {
        result.fill_square(next);
        if blocker_bb.get_square(next) {
            return result;
        }
        next = direction.next(next);
    }
    result
}
