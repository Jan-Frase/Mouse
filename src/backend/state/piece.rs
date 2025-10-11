/// Represents the different pieces.
#[derive(Copy, Clone, Debug, Ord, Eq, PartialEq, PartialOrd)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl PieceType {
    /// Returns an array containing all piece types.
    #[inline(always)]
    pub fn get_all_types() -> [PieceType; 6] {
        [
            PieceType::Pawn,
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
        ]
    }

    /// Returns an array containing all piece types that a pawn can promote to.
    pub fn get_promotable_types() -> [PieceType; 4] {
        [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
        ]
    }

    pub fn get_slider_types() -> [PieceType; 3] {
        [PieceType::Rook, PieceType::Bishop, PieceType::Queen]
    }

    pub fn get_trivial_types() -> [PieceType; 2] {
        [PieceType::Knight, PieceType::King]
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
    pub const fn opposite(self) -> PieceColor {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }

    pub fn get_all_colors() -> [PieceColor; 2] {
        [PieceColor::White, PieceColor::Black]
    }
}
