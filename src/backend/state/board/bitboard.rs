use crate::backend::constants::{FILES_AMOUNT, RANKS_AMOUNT};
use crate::backend::state::square::Square;
use std::fmt::{Display, Formatter};
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

/// A struct that represents a BitBoard.
/// Each bit in the `u64` value represents a specific position on the board.
///
/// # Fields
/// - `value` (`u64`): The underlying 64-bit integer used to store the board's state.
#[derive(Copy, Clone, Debug)]
pub struct BitBoard {
    pub value: u64,
}

impl BitBoard {
    /// Creates a new `BitBoard` instance with an initial value of 0.
    /// This can't be converted to a default variant because I need it to be const.
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        BitBoard { value: 0 }
    }

    /// Converts a given `Square` into a corresponding bitboard.
    const fn new_from_square(square: Square) -> BitBoard {
        let index = square.square_to_index();
        BitBoard { value: 1 << index }
    }

    pub const fn new_from_squares(squares: &[Square]) -> Self {
        let mut bitboard = BitBoard::new();

        let mut index = 0;
        while index < squares.len() {
            bitboard.fill_square(squares[index]);
            index += 1;
        }

        bitboard
    }

    pub const fn new_from_rank(rank: i8) -> Self {
        let mut bitboard = BitBoard::new();

        let mut file = 0;
        while file < FILES_AMOUNT {
            bitboard.fill_square(Square::new(file as i8, rank));
            file += 1;
        }

        bitboard
    }

    pub const fn new_from_file(file: i8) -> Self {
        let mut bitboard = BitBoard::new();

        let mut rank = 0;
        while rank < RANKS_AMOUNT {
            bitboard.fill_square(Square::new(file, rank as i8));
            rank += 1;
        }

        bitboard
    }

    /// Checks if the value of the current instance is empty or zero.
    ///
    /// # Returns
    /// - `true` if the value of the instance is `0`.
    /// - `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.value == 0
    }

    /// Checks if the value of the current instance is empty or zero.
    ///
    /// # Returns
    /// - `true` if the value of the instance is `0`.
    /// - `false` otherwise.
    pub fn is_not_empty(&self) -> bool {
        self.value != 0
    }

    /// Checks if a given square is occupied.
    ///
    /// # Arguments
    /// * `square` - The square to check, represented as a `Square` instance.
    ///
    /// # Returns
    /// * `true` if the square is occupied.
    /// * `false` if the square is unoccupied.
    pub fn get_square(&self, square: Square) -> bool {
        let index = Self::new_from_square(square);
        let bitboard_after_mask = self.value & index.value;
        bitboard_after_mask != 0
    }

    /// Marks a square as filled by setting the corresponding bit in the `value` field.
    ///
    /// # Parameters
    ///
    /// * `square` - A `Square` instance representing the square to be marked as filled.
    pub const fn fill_square(&mut self, square: Square) {
        let bit = Self::new_from_square(square);
        self.value |= bit.value;
    }

    /// Marks a square on a board as empty by setting the corresponding bit in the `value` field.
    ///
    /// # Parameters
    ///
    /// * `square` - A `Square` instance representing the square to be marked as filled.
    pub fn clear_square(&mut self, square: Square) {
        let bit = Self::new_from_square(square);
        self.value ^= bit.value;
    }
}

impl Iterator for BitBoard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value == 0 {
            return None;
        }

        let index = self.value.trailing_zeros();
        let square = Square::index_to_square(index as i8);
        self.clear_square(square);

        Some(square)
    }
}

impl ShlAssign<i32> for BitBoard {
    fn shl_assign(&mut self, rhs: i32) {
        self.value <<= rhs;
    }
}

impl Shl<i32> for BitBoard {
    type Output = Self;

    fn shl(self, rhs: i32) -> Self::Output {
        BitBoard {
            value: self.value << rhs,
        }
    }
}

impl ShrAssign<u32> for BitBoard {
    fn shr_assign(&mut self, rhs: u32) {
        self.value >>= rhs;
    }
}

impl Shr<i32> for BitBoard {
    type Output = Self;

    fn shr(self, rhs: i32) -> Self::Output {
        BitBoard {
            value: self.value >> rhs,
        }
    }
}

// Various bitwise operations on BitBoards.
impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        BitBoard { value: !self.value }
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard {
            value: self.value & rhs.value,
        }
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.value &= rhs.value;
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard {
            value: self.value | rhs.value,
        }
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.value |= rhs.value;
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard {
            value: self.value ^ rhs.value,
        }
    }
}

impl BitXorAssign<BitBoard> for BitBoard {
    fn bitxor_assign(&mut self, rhs: BitBoard) {
        self.value ^= rhs.value;
    }
}

// Display implementation for BitBoards - useful for debugging.
// Sadly, RustRover does not display them when debugging.
impl Display for BitBoard {
    /// Formats the bitboard (`self`) into a string representation,
    /// can be used for debugging purposes.
    ///
    /// The moves of a king on A1 get displayed like this:
    /// `_ _ _ _ _ _ _ _`
    /// `_ _ _ _ _ _ _ _`
    /// `_ _ _ _ _ _ _ _`
    /// `_ _ _ _ _ _ _ _`
    /// `_ _ _ _ _ _ _ _`
    /// `_ _ _ _ _ _ _ _`
    /// `X X _ _ _ _ _ _`
    /// `_ X _ _ _ _ _ _`
    ///
    /// # Parameters
    /// - `f`: A mutable reference to a formatter that implements the `Formatter` trait, used
    ///   to write the formatted output.
    ///
    /// # Returns
    /// - A `std::fmt::Result` indicating the success or failure of the formatting operation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for rank in (0..8).rev() {
            for file in 0..8 {
                let index = file + rank * 8;
                let bit = (self.value >> index) & 1;
                result.push(if bit == 1 { 'X' } else { '_' });
                result.push(' ');
            }
            result.push('\n');
        }

        write!(f, "{}", result)
    }
}
