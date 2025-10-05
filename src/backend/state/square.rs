use crate::backend::state::piece::PieceColor;
use getset::Setters;
use std::fmt::{Display, Formatter};

pub const A1: Square = Square::new(0, 0);
pub const B1: Square = Square::new(1, 0);
pub const C1: Square = Square::new(2, 0);
pub const D1: Square = Square::new(3, 0);
pub const E1: Square = Square::new(4, 0);
pub const F1: Square = Square::new(5, 0);
pub const G1: Square = Square::new(6, 0);
pub const H1: Square = Square::new(7, 0);
pub const A8: Square = Square::new(0, 7);
pub const B8: Square = Square::new(1, 7);
pub const C8: Square = Square::new(2, 7);
pub const D8: Square = Square::new(3, 7);
pub const E8: Square = Square::new(4, 7);
pub const F8: Square = Square::new(5, 7);
pub const G8: Square = Square::new(6, 7);
pub const H8: Square = Square::new(7, 7);

/// This represents a square on the chess board.
/// The square A1 is at file == 0 and rank == 0.
/// The square H1 is at file == 7 and rank == 0.
///
/// To make it easier to memorize: file => the letter part, rank => the number part
/// or put differently: file => vertical / x part, rank => horizontal / y part
#[derive(Copy, Clone, Debug, Setters, Ord, Eq, PartialEq, PartialOrd)]
pub struct Square {
    #[getset(set = "pub")]
    file: i8,
    #[getset(set = "pub")]
    rank: i8,
}

impl Square {
    /// Creates a new 'Square' instance.
    pub const fn new(file: i8, rank: i8) -> Self {
        Self { file, rank }
    }

    pub fn new_from_uci_notation(uci_notation: &str) -> Self {
        let mut file = 0;
        let mut rank = 0;

        for char in uci_notation.chars() {
            match char {
                'a'..='h' => file = char.to_digit(36).unwrap() - 10,
                '1'..='8' => rank = char.to_digit(10).unwrap() - 1,
                _ => panic!("Invalid uci notation"),
            }
        }

        Square {
            file: file as i8,
            rank: rank as i8,
        }
    }

    /// Converts a given index (i8) to a `Square` object.
    ///
    /// # Parameters
    /// - `index`: An `i8` representing the index of a square on a chessboard.
    ///   The valid range for the index is 0 to 63, where 0 corresponds to
    ///   A1 (bottom-left corner) and 63 corresponds to H8 (top-right corner).
    ///
    /// # Returns
    /// - `Square`: A struct representing the chessboard square with the following fields:
    ///   - `file`: The column (0 to 7) computed as `index % 8`.
    ///   - `rank`: The row (0 to 7) computed as `index / 8`.
    pub const fn index_to_square(index: i8) -> Square {
        Square {
            file: index % 8,
            rank: index / 8,
        }
    }
    /// Calculates the zero-based index of a square on a chessboard based on its file and rank.
    ///
    /// This is useful for representing a chessboard square as a single integer from 0 to 63
    /// # Returns
    /// A usize representing the square's index.
    pub const fn square_to_index(&self) -> usize {
        (self.file + self.rank * 8) as usize
    }

    /// Checks if the current position is valid.
    ///
    /// A position is considered valid if both the `file` and `rank` values fall within
    /// the bounds of a chessboard.
    ///
    /// # Returns
    ///
    /// * `true` - If the position is valid.
    /// * `false` - Otherwise.
    pub const fn is_valid(&self) -> bool {
        self.file >= 0 && self.file < 8 && self.rank >= 0 && self.rank < 8
    }

    /// Determines if the object is not valid. Simply negates "is_valid()".
    ///
    /// # Returns
    /// * `true` - If the object is not valid.
    /// * `false` - If the object is valid.
    pub fn is_not_valid(&self) -> bool {
        !self.is_valid()
    }

    /// Returns the 'file' attribute of the current instance. These aren't derived from getset because they need to be const.
    ///
    /// # Returns
    /// * `i8` - The value of the `file` field.
    pub const fn file(&self) -> i8 {
        self.file
    }

    /// Returns the 'rank' attribute of the current instance. These aren't derived from getset because they need to be const.
    ///
    /// # Returns
    /// * `i8` - The value of the `file` field.
    pub const fn rank(&self) -> i8 {
        self.rank
    }

    pub fn is_on_promotion_rank(&self) -> bool {
        self.rank == 0 || self.rank == 7
    }

    pub const fn is_pawn_start(&self, color: PieceColor) -> bool {
        match color {
            PieceColor::White => self.rank == 1,
            PieceColor::Black => self.rank == 6,
        }
    }

    // forward means the direction that the pawns travel in for that side
    pub const fn forward_by_one(&self, color: PieceColor) -> Square {
        match color {
            PieceColor::White => Square::new(self.file, self.rank + 1),
            PieceColor::Black => Square::new(self.file, self.rank - 1),
        }
    }

    pub const fn back_by_one(&self, color: PieceColor) -> Square {
        self.forward_by_one(color.opposite())
    }

    pub const fn right_by_one(&self) -> Square {
        Square::new(self.file + 1, self.rank)
    }

    pub const fn left_by_one(&self) -> Square {
        Square::new(self.file - 1, self.rank)
    }
}

/// Turns a `Square` instance into a string like "a1".
impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        result.push_str(match self.file {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            7 => "h",
            _ => panic!("Invalid file value"),
        });

        result.push_str(match self.rank {
            0 => "1",
            1 => "2",
            2 => "3",
            3 => "4",
            4 => "5",
            5 => "6",
            6 => "7",
            7 => "8",
            _ => panic!("Invalid rank value"),
        });

        write!(f, "{}", result)
    }
}
