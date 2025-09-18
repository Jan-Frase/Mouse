use crate::backend::square::Square;
use std::fmt::{Display, Formatter};

/// A struct that represents a BitBoard.
/// Each bit in the `u64` value represents a specific position on the board.
///
/// # Fields
/// - `value` (`u64`): The underlying 64-bit integer used to store the board's state.
#[derive(Copy, Clone)]
pub struct BitBoard {
    value: u64,
}

impl BitBoard {
    /// Creates a new `BitBoard` instance with an initial value of 0.
    ///
    /// # Returns
    ///
    /// A `BitBoard` instance with the `value` field set to 0.
    pub const fn new() -> Self {
        BitBoard { value: 0 }
    }

    /// Checks if the value of the current instance is empty or zero.
    ///
    /// # Returns
    /// - `true` if the value of the instance is `0`.
    /// - `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.value == 0
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

impl Display for BitBoard {
    /// Formats the bitboard (`self`) into a string representation,
    /// can be used for debugging purposes.
    ///
    /// The moves of a king on A1 get displayed like this:
    ///_ _ _ _ _ _ _ _
    ///  _ _ _ _ _ _ _ _
    /// _ _ _ _ _ _ _ _
    /// _ _ _ _ _ _ _ _
    /// _ _ _ _ _ _ _ _
    /// _ _ _ _ _ _ _ _
    /// X X _ _ _ _ _ _
    /// _ X _ _ _ _ _ _
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
