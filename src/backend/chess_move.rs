use crate::backend::square::Square;
use getset::{CloneGetters, Setters};

/// Represents the various types of promotions that can occur in a game of chess.
///
/// Has an additional `NONE` option to represent no promotion.
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone, CloneGetters, Setters)]
pub struct ChessMove {
    #[getset(get_clone = "pub", set = "pub")]
    from: Square,
    #[getset(get_clone = "pub", set = "pub")]
    to: Square,
    #[getset(get_clone = "pub", set = "pub")]
    promotion_type: PromotionType,
}

impl ChessMove {
    /// Creates a new `Move` instance with 'promotion_type' set to 0.
    pub fn new(from: Square, to: Square) -> ChessMove {
        ChessMove {
            from,
            to,
            promotion_type: PromotionType::None,
        }
    }
}
