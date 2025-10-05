use crate::backend::state::square::Square;
use crate::constants::{FILES_AMOUNT, SQUARES_AMOUNT};
use std::fmt::{Display, Formatter};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

/// A struct that represents a BitBoard.
/// Each bit in the `u64` value represents a specific position on the board.
///
/// # Fields
/// - `value` (`u64`): The underlying 64-bit integer used to store the board's state.
#[derive(Copy, Clone, Debug)]
pub struct Bitboard {
    value: u64,
}

impl Bitboard {
    /// Creates a new `BitBoard` instance with an initial value of 0.
    /// This cant be converted to a default variant cause i need it to be const.
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Bitboard { value: 0 }
    }

    pub const fn new_from_value(value: u64) -> Self {
        Bitboard { value }
    }

    pub fn new_from_squares(squares: Vec<Square>) -> Self {
        let mut bitboard = Bitboard::new();

        for square in squares {
            bitboard.fill_square(square);
        }

        bitboard
    }

    pub const fn new_from_rank(rank: i8) -> Self {
        let mut bitboard = Bitboard::new();

        let mut file = 0;
        while file < FILES_AMOUNT {
            bitboard.fill_square(Square::new(file as i8, rank));
            file += 1;
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
        let index = Self::square_to_bitmask(square);
        let bitboard_after_mask = self.value & index;
        bitboard_after_mask != 0
    }

    /// Marks a square as filled by setting the corresponding bit in the `value` field.
    ///
    /// # Parameters
    ///
    /// * `square` - A `Square` instance representing the square to be marked as filled.
    pub const fn fill_square(&mut self, square: Square) {
        let bit = Self::square_to_bitmask(square);
        self.value |= bit;
    }

    /// Marks a square on a board as empty by setting the corresponding bit in the `value` field.
    ///
    /// # Parameters
    ///
    /// * `square` - A `Square` instance representing the square to be marked as filled.
    pub fn clear_square(&mut self, square: Square) {
        let bit = Self::square_to_bitmask(square);
        self.value ^= bit;
    }

    /// Retrieves all `Square` instances that correspond to the true bits
    /// in the bitboard.
    /// PERF: Speeding this up would be very useful.
    ///
    /// # Returns
    ///
    /// A `Vec<Square>` containing all `Square` instances corresponding to active bits
    /// in the bitboard.
    pub fn get_all_true_squares(&self) -> Vec<Square> {
        // Init the vec with enough capacity for all pawns.
        let mut squares = Vec::with_capacity(FILES_AMOUNT);
        // Loop over all squares and check if the bit is set.
        let mut bit = 1;
        for i in 0..SQUARES_AMOUNT {
            if self.value & bit != 0 {
                squares.push(Square::index_to_square(i as i8));
            }
            // Shift the mask bit one to the left.
            bit <<= 1;
        }
        squares
    }

    /// Converts a given `Square` into a corresponding 64-bit bitmask.
    ///
    /// # Parameters
    /// - `square`: A `Square` object that represents a position or coordinate
    ///   which can be converted into a bitmask.
    ///
    /// # Returns
    /// - A `u64` value representing the bitmask corresponding to the input `Square`.
    ///   The bitmask will have exactly one bit set, corresponding to the index
    ///   value of the `Square`.
    const fn square_to_bitmask(square: Square) -> u64 {
        let index = square.square_to_index();
        1 << index
    }
}

// Various bitwise operations on BitBoards.
impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Bitboard { value: !self.value }
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard {
            value: self.value & rhs.value,
        }
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.value &= rhs.value;
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard {
            value: self.value | rhs.value,
        }
    }
}

impl BitOrAssign<Bitboard> for &mut Bitboard {
    fn bitor_assign(&mut self, rhs: Bitboard) {
        self.value |= rhs.value;
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.value |= rhs.value;
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard {
            value: self.value ^ rhs.value,
        }
    }
}

impl BitXorAssign<Bitboard> for &mut Bitboard {
    fn bitxor_assign(&mut self, rhs: Bitboard) {
        self.value ^= rhs.value;
    }
}

// Display implementation for BitBoards - useful for debugging.
// Sadly, RustRover does not display them when debugging.
impl Display for Bitboard {
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
