use crate::square::Square;

/// A struct that represents a BitBoard, which is a compact representation
/// of a 64-bit board.
/// Each bit in the `u64` value can represent a specific position on the board,
/// providing an efficient way to manage and manipulate game states.
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
    pub fn new() -> Self {
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

    /// Checks if a given square is occupied on the chessboard.
    ///
    /// This function determines whether a specific square on the chessboard
    /// is occupied based on the internal bitboard representation of the chessboard.
    /// It achieves this by converting the square into a bitmask and applying
    /// a bitwise AND operation against the bitboard value. If any bits are set
    /// after this operation, the square is considered occupied.
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
        let is_square_occupied = bitboard_after_mask != 0;
        is_square_occupied
    }

    /// Marks a square on a board as filled by setting the corresponding bit in the `value` field.
    ///
    /// # Parameters
    ///
    /// * `self` - A mutable reference to the struct instance where the square will be marked as filled.
    /// * `square` - A `Square` instance representing the square to be marked as filled.
    pub fn fill_square(&mut self, square: Square) {
        let bit = Self::square_to_bitmask(square);
        self.value |= bit;
    }

    /// Marks a square on a board as empty by setting the corresponding bit in the `value` field.
    ///
    /// # Parameters
    ///
    /// * `self` - A mutable reference to the struct instance where the square will be marked as filled.
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
    fn square_to_bitmask(square: Square) -> u64 {
        let index = square.square_to_index();
        let bit = 1 << index;
        bit
    }
}
