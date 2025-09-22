use crate::backend::moove::Moove;
use crate::backend::square::Square;
use crate::backend::state::bitboard::BitBoard;

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
    fn next(&self, square: &Square) -> Square {
        match self {
            SlideDirection::Up => Square::new(square.file(), square.rank() + 1),
            SlideDirection::UpRight => Square::new(square.file() + 1, square.rank() + 1),
            SlideDirection::Right => Square::new(square.file() + 1, square.rank()),
            SlideDirection::DownRight => Square::new(square.file() - 1, square.rank() + 1),
            SlideDirection::Down => Square::new(square.file(), square.rank() - 1),
            SlideDirection::DownLeft => Square::new(square.file() - 1, square.rank() - 1),
            SlideDirection::Left => Square::new(square.file() - 1, square.rank()),
            SlideDirection::UpLeft => Square::new(square.file() - 1, square.rank() + 1),
        }
    }
}

pub fn get_moves_for_non_slider_piece(
    piece_bitboard: BitBoard,
    friendly_pieces_bitboard: BitBoard,
    enemy_pieces_bitboard: BitBoard,
) -> Vec<Moove> {
    let mut moves: Vec<Moove> = Vec::new();
    for piece in piece_bitboard.get_all_true_squares() {
        let mut moves_for_piece = get_moves_for_square(&piece, &friendly_pieces_bitboard, &enemy_pieces_bitboard);
        moves.append(&mut moves_for_piece);
    }
    moves
}

/// Computes all possible moves for a given square.
/// # Parameters
///
/// - `square`: The `Square` for which moves are being calculated. This represents
///   the current position of the piece.
/// - `friendly_pieces_bitboard`: A `BitBoard` representing positions of all friendly
///   pieces. Pieces located in these positions will block movement.
///
/// - `enemy_pieces_bitboard`: A `BitBoard` representing positions of all enemy
///   pieces. Pieces located in these positions will block movement beyond their position.
///
/// # Returns
///
/// A `Vec` of `Moove` structs that represent all legal moves
/// the piece on the provided square can make.
fn get_moves_for_square(
    square: &Square,
    friendly_pieces_bitboard: &BitBoard,
    enemy_pieces_bitboard: &BitBoard,
) -> Vec<Moove> {
    let moves_bitboard = calculate_move_bitboard(square, friendly_pieces_bitboard, enemy_pieces_bitboard);

    // Now take the resulting bitboard and convert all true squares to a list of squares.
    let squares_we_can_move_to = moves_bitboard.get_all_true_squares();

    // generate all the moves
    let mut moves: Vec<Moove> = Vec::with_capacity(squares_we_can_move_to.len());
    for to_square in squares_we_can_move_to {
        moves.push(Moove::new(*square, to_square))
    }
    // Done :)
    moves
}


fn calculate_move_bitboard(
    square: &Square,
    friendly_pieces_bitboard: &BitBoard,
    enemy_pieces_bitboard: &BitBoard,
) -> BitBoard {
    let mut move_bitboard: BitBoard = BitBoard::new();
    for direction in ALL_SLIDE_DIRECTIONS.iter() {
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
    square: &Square,
    direction: &SlideDirection,
    friendly_pieces_bitboard: &BitBoard,
    enemy_pieces_bitboard: &BitBoard,
) -> BitBoard {
    let mut result = BitBoard::new();
    let mut next = direction.next(square);
    while next.is_valid() && !friendly_pieces_bitboard.get_square(next) {
        result.fill_square(next);
        if enemy_pieces_bitboard.get_square(next) {
            return result;
        }
        next = direction.next(square);
    }
    result
}
