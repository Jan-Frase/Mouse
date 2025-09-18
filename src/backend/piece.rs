/// Represents the different pieces.
#[derive(Copy, Clone)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

/// Represents the color of a piece.
#[derive(Copy, Clone)]
pub enum PieceColor {
    White,
    Black,
}

/// The `Piece` struct stores the type and color of a chess piece.
#[derive(Copy, Clone)]
pub struct Piece {
    piece_type: PieceType,
    piece_color: PieceColor,
}

impl Piece {
    /// Creates a new instance of a `Piece` with the specified type and color.
    pub fn new(piece_type: PieceType, piece_color: PieceColor) -> Self {
        Piece {
            piece_type,
            piece_color,
        }
    }

    /// Returns the `PieceType` of the current piece.
    ///
    /// # Returns
    /// * `PieceType` - The type of the piece.
    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }

    /// Returns the color of the chess piece.
    ///
    /// # Returns
    /// * `PieceColor` - The color of the chess piece, represented by the `PieceColor` enum.
    pub fn piece_color(&self) -> PieceColor {
        self.piece_color
    }
}
