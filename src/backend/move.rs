use crate::backend::square::Square;

/// Represents the various types of promotions that can occur in a game of chess.
///
/// Has an additional `NONE` option to represent no promotion.
pub enum PromotionType {
    Rook,
    Knight,
    Bishop,
    Queen,
    None,
}

/// This encodes a single move.
/// It knows where a piece moved from and where it moved to.
/// Also stores to which piece a pawn promoted if one did at all.
///
/// PERFORMANCE: This could be squeezed into a bitfield like, for example, stockfish does;
/// Around line 370: https://github.com/official-stockfish/Stockfish/blob/master/src/types.h
/// I have not done this yet for two reasons:
/// 1. I'm not sure, without any benchmarks if it gains any performance.
///    Sure, the move would be smaller, but accessing a variable would be slower, since it requires bit shifting etc.
///    In the end it comes down to a trade-off between cache locality and number of instructions per read.
/// 2. It would certainly make the code less readable.
pub struct Move {
    from: Square,
    to: Square,
    promotion_type: PromotionType,
}

impl Move {
    /// Creates a new `Move` instance.
    ///
    /// # Parameters
    /// - `from`: A `Square` representing the starting position of the move.
    /// - `to`: A `Square` representing the destination position of the move.
    ///
    /// # Returns
    /// A `Move` struct with the specified `from` and `to` positions. The `promotion`
    /// field is initialized to `0` by default, indicating that no promotion is
    /// applied.
    pub fn new(from: Square, to: Square) -> Move {
        Move {
            from,
            to,
            promotion_type: PromotionType::None,
        }
    }

    /// Returns the `from` field of the current instance.
    /// # Returns
    /// A `Square` object representing the value of the `from` field.
    pub fn from(&self) -> Square {
        self.from
    }

    /// Returns the `to` field of the current instance.
    /// # Returns
    /// A `Square` object representing the value of the `to` field.
    pub fn to(&self) -> Square {
        self.to
    }

    /// Returns the `promotion_type` field of the current instance.
    /// # Returns
    /// A `PromotionType` object representing the value of the `promotion_type` field.
    pub fn promotion_type(&self) -> &PromotionType {
        &self.promotion_type
    }

    /// Sets the starting square (`from`) for a move.
    ///
    /// # Parameters
    /// - `from`: A `Square` value representing the starting position of a move.
    pub fn set_from(&mut self, from: Square) {
        self.from = from;
    }

    /// Sets the target square (`to`) for a move.
    ///
    /// # Parameters
    /// - `to`: A `Square` value representing the target position of a move.
    pub fn set_to(&mut self, to: Square) {
        self.to = to;
    }

    /// Sets the `promotion_type` for a move.
    ///
    /// # Parameters
    /// - `promotion_type`: A `PromotionType` value representing the promotion type of a move.
    pub fn set_promotion(&mut self, promotion_type: PromotionType) {
        self.promotion_type = promotion_type;
    }
}
