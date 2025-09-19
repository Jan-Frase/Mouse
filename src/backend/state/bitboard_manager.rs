use crate::backend::piece::{Piece, PieceColor, PieceType};
use crate::backend::square::Square;
use crate::backend::state::bitboard::BitBoard;
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
#[derive(Debug)]
pub struct BitBoardManager {
    bitboards: [BitBoard; PIECE_TYPE_COUNT * SIDES],
    bitboard_index_to_piece: [Piece; PIECE_TYPE_COUNT * SIDES],
}

impl BitBoardManager {
    /// Generates a new BitBoardManager with all bitboards set to empty.
    pub fn new() -> BitBoardManager {
        // This contains all the bitboards.
        let bitboards = [BitBoard::new(); PIECE_TYPE_COUNT * SIDES];

        // This maps each index in the bitboard array back to its corresponding piece.
        // I'm sure there is a better way to do this, but I'm not sure how.
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

    /// Retrieves a mutable reference to the bitboard associated with the given piece.
    ///
    /// # Arguments
    /// * `piece` - The `Piece` for which the mutable reference to the bitboard is requested.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `BitBoard` corresponding to the given `piece`.
    pub fn get_bitboard_mut(&mut self, piece: Piece) -> &mut BitBoard {
        let index = self.piece_to_bitboards_index(piece);
        &mut self.bitboards[index]
    }

    /// Retrieves a reference to the `BitBoard` associated with the specified `Piece`.
    ///
    /// # Parameters:
    /// - `piece`: The piece for which the associated `BitBoard`
    ///   is being retrieved.
    ///
    /// # Returns:
    /// - A reference to the `BitBoard` corresponding to the given `Piece`.
    pub fn get_bitboard(&self, piece: Piece) -> &BitBoard {
        let index = self.piece_to_bitboards_index(piece);
        &self.bitboards[index]
    }

    /// Retrieves the piece located at a specific square on the chessboard.
    ///
    /// # Parameters
    /// - `square`: The square for which the piece is to be retrieved.
    ///
    /// # Returns
    /// - `Some(Piece)` if there is a piece located at the given square.
    /// - `None` if the square is empty.
    pub fn get_piece_at_square(&self, square: Square) -> Option<Piece> {
        let index = self.get_index_for_piece_at_square(square)?;
        Some(self.bitboard_index_to_piece[index])
    }

    /// Retrieves a mutable reference to the `BitBoard` corresponding to the piece located at the specified square.
    ///
    /// # Arguments
    ///
    /// * `square` - A `Square` that specifies the location of the piece whose `BitBoard` is
    ///              to be retrieved.
    ///
    /// # Returns
    ///
    /// * `Some(&mut BitBoard)` - A mutable reference to the `BitBoard
    pub fn get_bitboard_for_piece_at_square_mut(
        &mut self,
        square: Square,
    ) -> Option<&mut BitBoard> {
        let index = self.get_index_for_piece_at_square(square)?;
        Some(&mut self.bitboards[index])
    }

    /// Returns the `BitBoard` associated with a piece at the specified `square` if it exists.
    ///
    /// # Arguments
    ///
    /// * `square` - A `Square` representing the position on the chessboard where the piece is expected to be located.
    ///
    /// # Returns
    ///
    /// * `Option<BitBoard>` - Returns `Some(BitBoard)` if a piece is present at the specified square, otherwise returns `None`.
    pub fn get_bitboard_for_piece_at_square(&self, square: Square) -> Option<BitBoard> {
        let index = self.get_index_for_piece_at_square(square)?;
        Some(self.bitboards[index])
    }

    /// Returns a `BitBoard` containing all the positions currently occupied by pieces
    /// of the specified color.
    ///
    /// # Arguments
    ///
    /// * `color` - A `PieceColor` enum value (`PieceColor::White` or `PieceColor::Black`)
    ///             indicating the color of the pieces whose positions should be retrieved.
    ///
    /// # Returns
    ///
    /// A `BitBoard` that aggregates all the positions of the specified color's pieces.
    pub fn get_all_pieces_off(&self, color: PieceColor) -> BitBoard {
        let mut resulting_bitboard = BitBoard::new();

        // This is quite ugly...
        let start_index = match color {
            PieceColor::White => WHITE_START_INDEX,
            PieceColor::Black => BLACK_START_INDEX,
        };

        let end_index = match color {
            PieceColor::White => WHITE_END_INDEX,
            PieceColor::Black => BLACK_END_INDEX,
        };

        // Simply loop over all relevant bitboards and add them to the resulting bitboard.
        // PERF: Instead of generating this when it's necessary, we could generate it once and
        //       incrementally update it.
        //       Then again, according to the benchmarks, this is not a bottleneck.
        for index in start_index..=end_index {
            resulting_bitboard |= self.bitboards[index];
        }

        resulting_bitboard
    }

    /// Used by `get_bitboard_for_piece_at_square(_mut)` and `get_piece_at_square`.
    /// Returns the index of the piece at that square if there is one.
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
