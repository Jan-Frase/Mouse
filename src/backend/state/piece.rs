/// Represents the different pieces.
#[derive(Copy, Clone, Debug, Ord, Eq, PartialEq, PartialOrd)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

pub const ALL_PIECES: [Piece; 6] = [
    Piece::Pawn,
    Piece::Rook,
    Piece::Knight,
    Piece::Bishop,
    Piece::Queen,
    Piece::King,
];

pub const PROMOTABLE_PIECES: [Piece; 4] = [Piece::Rook, Piece::Knight, Piece::Bishop, Piece::Queen];

pub const SLIDER_PIECES: [Piece; 3] = [Piece::Rook, Piece::Bishop, Piece::Queen];

pub const TRIVIAL_PIECES: [Piece; 2] = [Piece::Knight, Piece::King];

/// Represents the color of a piece.
#[derive(Copy, Clone, Debug)]
pub enum Side {
    White,
    Black,
}

impl Side {
    /// Returns the opposite color of the current `PieceColor`.
    pub fn opposite(self) -> Side {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }

    pub fn get_all_colors() -> [Side; 2] {
        [Side::White, Side::Black]
    }
}
