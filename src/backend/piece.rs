use getset::CloneGetters;

/// Represents the different pieces.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl PieceType {
    /// Returns an array containing all piece types. For now, it's just Knight and King.
    pub fn get_all_types() -> [PieceType; 3] {
        [PieceType::Knight, PieceType::King, PieceType::Pawn]
    }
}

/// Represents the color of a piece.
#[derive(Copy, Clone, Debug)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    /// Returns the opposite color of the current `PieceColor`.
    pub fn opposite(self) -> PieceColor {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}

/// The `Piece` struct stores the type and color of a chess piece.
#[derive(Copy, Clone, Debug, CloneGetters)]
pub struct Piece {
    #[getset(get_clone = "pub")]
    piece_type: PieceType,
    #[getset(get_clone = "pub")]
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
}
