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

pub const ALL_PIECES: [PieceType; 6] = [
    PieceType::Pawn,
    PieceType::Rook,
    PieceType::Knight,
    PieceType::Bishop,
    PieceType::Queen,
    PieceType::King,
];

pub const PROMOTABLE_PIECES: [PieceType; 4] = [
    PieceType::Rook,
    PieceType::Knight,
    PieceType::Bishop,
    PieceType::Queen,
];

pub const SLIDER_PIECES: [PieceType; 3] = [PieceType::Rook, PieceType::Bishop, PieceType::Queen];

pub const TRIVIAL_PIECES: [PieceType; 2] = [PieceType::Knight, PieceType::King];

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
