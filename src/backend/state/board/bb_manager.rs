use crate::backend::constants::PIECE_TYPE_COUNT;
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::piece::ALL_PIECES;
use crate::backend::state::piece::{PieceColor, PieceType};
use crate::backend::state::square::Square;

/// A struct that manages bitboards used for representing chess pieces and their positions on a chessboard.
/// # Fields
/// - `bitboards`: An array of bitboards where each entry represents the board state
///
/// - `bitboard_index_to_piece`: An array that maps each index in the `bitboards` array
///   back to its corresponding `Piece`.
#[derive(Debug, Clone)]
pub struct BBManager {
    white_bb: BitBoard,
    black_bb: BitBoard,
    piece_bbs: [BitBoard; PIECE_TYPE_COUNT],
}

impl BBManager {
    /// Generates a new BitBoardManager with all bitboards set to empty.
    pub fn new() -> BBManager {
        BBManager {
            white_bb: BitBoard::new(),
            black_bb: BitBoard::new(),
            piece_bbs: [BitBoard::new(); PIECE_TYPE_COUNT],
        }
    }

    /// Retrieves a mutable reference to the bitboard associated with the given piece.
    pub fn get_piece_bb_mut(&mut self, piece_type: PieceType) -> &mut BitBoard {
        &mut self.piece_bbs[piece_type as usize]
    }

    /// Retrieves a copy of the `BitBoard` associated with the specified `Piece`.
    pub fn get_piece_bb(&self, piece_type: PieceType) -> BitBoard {
        self.piece_bbs[piece_type as usize]
    }

    /// Returns a `BitBoard` containing all the positions currently occupied by pieces
    /// of the specified color.
    pub fn get_all_pieces_bb_off(&self, color: PieceColor) -> BitBoard {
        match color {
            PieceColor::White => self.white_bb,
            PieceColor::Black => self.black_bb,
        }
    }

    /// Returns a `BitBoard` containing all the positions currently occupied by pieces
    /// of the specified color.
    pub fn get_all_pieces_bb_off_mut(&mut self, color: PieceColor) -> &mut BitBoard {
        match color {
            PieceColor::White => &mut self.white_bb,
            PieceColor::Black => &mut self.black_bb,
        }
    }

    pub fn get_colored_piece_bb(&self, piece_type: PieceType, color: PieceColor) -> BitBoard {
        let piece_bb = self.get_piece_bb(piece_type);
        let color_bb = self.get_all_pieces_bb_off(color);
        piece_bb & color_bb
    }

    /// Retrieves the piece located at a specific square on the chessboard.
    pub fn get_piece_at_square(&self, square: Square) -> Option<PieceType> {
        for index in 0..self.piece_bbs.len() {
            let bitboard = self.piece_bbs[index];
            if bitboard.get_square(square) {
                return Some(ALL_PIECES[index]);
            }
        }
        None
    }
}

impl Default for BBManager {
    fn default() -> Self {
        Self::new()
    }
}
