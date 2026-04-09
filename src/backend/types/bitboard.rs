use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};
use crate::backend::types::square::Square;

/// A struct that represents a BitBoard.
/// Each bit in the `u64` value represents a specific position on the board.
///
/// # Fields
/// - `value` (`u64`): The underlying 64-bit integer used to store the board's game_state.
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
    fn new_from_square(square: Square) -> BitBoard {
        BitBoard { value: 1 << square }
    }

    /// Checks if the value of the current instance is empty or zero.
    pub fn is_empty(&self) -> bool {
        self.value == 0
    }

    /// Checks if the value of the current instance is empty or zero.
    pub fn is_not_empty(&self) -> bool {
        self.value != 0
    }

    /// Checks if a given square is occupied.
    pub fn get_square(&self, square: Square) -> bool {
        let index = Self::new_from_square(square);
        let bitboard_after_mask = self.value & index.value;
        bitboard_after_mask != 0
    }

    /// Marks a square as filled by setting the corresponding bit in the `value` field.
    pub fn fill_square(&mut self, square: Square) {
        let bit = Self::new_from_square(square);
        self.value |= bit.value;
    }

    /// Marks a square on a board as empty by setting the corresponding bit in the `value` field.
    pub fn clear_square(&mut self, square: Square) {
        let bit = Self::new_from_square(square);
        self.value ^= bit.value;
    }
}

// ---------------------------------------
// Iterator, Operator overloads, and Display.
// ---------------------------------------
impl Iterator for BitBoard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value == 0 {
            return None;
        }

        let square = self.value.trailing_zeros() as Square;
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
