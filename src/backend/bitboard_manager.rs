use crate::backend::bitboard::BitBoard;
use crate::backend::piece::{Piece, PieceColor, PieceType};
use crate::backend::square::Square;
use crate::constants::{PIECE_TYPE_COUNT, SIDES};

const WHITE_START_INDEX: usize = 0;
const WHITE_END_INDEX: usize = 5;
const BLACK_START_INDEX: usize = 6;
const BLACK_END_INDEX: usize = 11;

/// A struct that manages bitboards used for representing chess pieces and their positions on a chessboard.
/// # Fields
/// - `bitboards`: An array of bitboards where each entry represents the board state
///
/// - `bitboard_index_to_piece`: An array that maps each index in the `bitboards` array
///   back to its corresponding `Piece`.
pub struct BitBoardManager {
    bitboards: [BitBoard; PIECE_TYPE_COUNT * SIDES],
    bitboard_index_to_piece: [Piece; PIECE_TYPE_COUNT * SIDES],
}

impl BitBoardManager {
    /// Generates a new BitBoardManager with all bitboards set to empty.
    pub fn new() -> BitBoardManager {
        let bitboards = [BitBoard::new(); PIECE_TYPE_COUNT * SIDES];

        let mut bitboard_index_to_piece =
            [Piece::new(PieceType::Pawn, PieceColor::White); PIECE_TYPE_COUNT * SIDES];

        // insert all the white pieces
        bitboard_index_to_piece[0] = Piece::new(PieceType::Pawn, PieceColor::White);
        bitboard_index_to_piece[1] = Piece::new(PieceType::Rook, PieceColor::White);
        bitboard_index_to_piece[2] = Piece::new(PieceType::Knight, PieceColor::White);
        bitboard_index_to_piece[3] = Piece::new(PieceType::Bishop, PieceColor::White);
        bitboard_index_to_piece[4] = Piece::new(PieceType::Queen, PieceColor::White);
        bitboard_index_to_piece[5] = Piece::new(PieceType::King, PieceColor::White);

        // insert all the black pieces
        bitboard_index_to_piece[6] = Piece::new(PieceType::Pawn, PieceColor::Black);
        bitboard_index_to_piece[7] = Piece::new(PieceType::Rook, PieceColor::Black);
        bitboard_index_to_piece[8] = Piece::new(PieceType::Knight, PieceColor::Black);
        bitboard_index_to_piece[9] = Piece::new(PieceType::Bishop, PieceColor::Black);
        bitboard_index_to_piece[10] = Piece::new(PieceType::Queen, PieceColor::Black);
        bitboard_index_to_piece[11] = Piece::new(PieceType::King, PieceColor::Black);

        BitBoardManager {
            bitboards,
            bitboard_index_to_piece,
        }
    }

    pub fn get_bitboard_mut(&mut self, piece: Piece) -> &mut BitBoard {
        let index = self.piece_to_bitboards_index(piece);
        &mut self.bitboards[index]
    }

    pub fn get_bitboard(&self, piece: Piece) -> &BitBoard {
        let index = self.piece_to_bitboards_index(piece);
        &self.bitboards[index]
    }

    pub fn get_piece_at_square(&self, square: Square) -> Option<Piece> {
        let index = self.get_index_for_piece_at_square(square)?;
        Some(self.bitboard_index_to_piece[index])
    }

    pub fn get_bitboard_for_piece_at_square_mut(
        &mut self,
        square: Square,
    ) -> Option<&mut BitBoard> {
        let index = self.get_index_for_piece_at_square(square)?;
        Some(&mut self.bitboards[index])
    }

    pub fn get_bitboard_for_piece_at_square(&self, square: Square) -> Option<BitBoard> {
        let index = self.get_index_for_piece_at_square(square)?;
        Some(self.bitboards[index])
    }

    pub fn get_all_pieces_off(&self, color: PieceColor) -> BitBoard {
        let mut resulting_bitboard = BitBoard::new();

        let start_index = match color {
            PieceColor::White => WHITE_START_INDEX,
            PieceColor::Black => BLACK_START_INDEX,
        };

        let end_index = match color {
            PieceColor::White => WHITE_END_INDEX,
            PieceColor::Black => BLACK_END_INDEX,
        };

        for index in start_index..=end_index {
            resulting_bitboard |= self.bitboards[index];
        }

        resulting_bitboard
    }

    fn get_index_for_piece_at_square(&self, square: Square) -> Option<usize> {
        for index in 0..self.bitboards.len() {
            let bitboard = self.bitboards[index];
            if bitboard.get_square(square) {
                return Some(index);
            }
        }
        None
    }

    /// Converts a `Piece` instance into a corresponding index for bitboard representation.
    ///
    /// # Parameters
    /// - `piece`: A `Piece` instance for which the bitboard index needs to be determined.
    ///
    /// # Returns
    /// - `usize`: The index corresponding to the given `piece` in the bitboard.
    fn piece_to_bitboards_index(&self, piece: Piece) -> usize {
        let mut index = 0;

        // this causes the index to be at 0 for white and 6 for black
        // it works because white as usize == 0 and black as usize == 1
        index += piece.piece_color() as usize * PIECE_TYPE_COUNT;
        // this adds 0 for a pawn, 1 for a rook, etc.
        index += piece.piece_type() as usize;
        // the indices are
        // white: pawn: 0, rook: 1, knight: 2, bishop: 3, queen: 4, king: 5
        // black: pawn: 6, rook: 7, knight: 8, bishop: 9, queen: 10, king: 11
        index
    }
}
