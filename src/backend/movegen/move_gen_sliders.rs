use crate::backend::movegen::moove::Moove;
use crate::backend::movegen::move_gen_sliders::SlideDirection::{
    Down, DownLeft, DownRight, Left, Right, Up, UpLeft, UpRight,
};
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::piece::PieceType;
use crate::backend::state::square::Square;

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

const ALL_SLIDE_DIRECTIONS: [SlideDirection; 8] = [
    SlideDirection::Up,
    SlideDirection::UpRight,
    SlideDirection::Right,
    SlideDirection::DownRight,
    SlideDirection::Down,
    SlideDirection::DownLeft,
    SlideDirection::Left,
    SlideDirection::UpLeft,
];

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

    fn directions_for_piece_type(piece_type: PieceType) -> Vec<SlideDirection> {
        match piece_type {
            PieceType::Rook => {
                vec![Up, Down, Left, Right]
            }
            PieceType::Bishop => {
                vec![UpRight, DownRight, DownLeft, UpLeft]
            }
            PieceType::Queen => {
                vec![Up, Down, Left, Right, UpRight, DownRight, DownLeft, UpLeft]
            }
            _ => panic!("Piece type is not a slider"),
        }
    }
}

pub fn get_moves_for_non_slider_piece(
    piece_type: PieceType,
    piece_bb: BitBoard,
    friendly_pieces_bb: BitBoard,
    enemy_pieces_bb: BitBoard,
) -> Vec<Moove> {
    let mut moves: Vec<Moove> = Vec::new();
    for square in piece_bb.get_all_true_squares() {
        let moves_for_piece_bb =
            calculate_move_bitboard(piece_type, square, friendly_pieces_bb, enemy_pieces_bb);

        // Now take the resulting bitboard and convert all true squares to a list of squares.
        let squares_we_can_move_to = moves_for_piece_bb.get_all_true_squares();

        // generate all the moves
        for to_square in squares_we_can_move_to {
            moves.push(Moove::new(square, to_square))
        }
    }
    moves
}

pub fn calculate_move_bitboard(
    piece_type: PieceType,
    square: Square,
    friendly_pieces_bitboard: BitBoard,
    enemy_pieces_bitboard: BitBoard,
) -> BitBoard {
    let mut move_bitboard: BitBoard = BitBoard::new();
    for direction in SlideDirection::directions_for_piece_type(piece_type) {
        move_bitboard |= calculate_max_slide_range(
            square,
            direction,
            friendly_pieces_bitboard,
            enemy_pieces_bitboard,
        );
    }
    move_bitboard
}

/// Computes a bitboard containing all squares
/// that the piece on the given square can slide to in the given direction
fn calculate_max_slide_range(
    square: Square,
    direction: SlideDirection,
    friendly_pieces_bitboard: BitBoard,
    enemy_pieces_bitboard: BitBoard,
) -> BitBoard {
    let mut result = BitBoard::new();
    let mut next = direction.next(square);
    while next.is_valid() && !friendly_pieces_bitboard.get_square(next) {
        result.fill_square(next);
        if enemy_pieces_bitboard.get_square(next) {
            return result;
        }
        next = direction.next(next);
    }
    result
}
