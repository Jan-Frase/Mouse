use std::fmt::{Display, Formatter};
use crate::backend::types::piece::Piece;
use crate::backend::types::square::{get_file, square_to_string, Square};

#[derive(Copy, Clone)]
pub enum CastleType {
    Long,
    Short,
}

impl CastleType {
    pub fn get_all_types() -> [CastleType; 2] {
        [CastleType::Long, CastleType::Short]
    }
}

/// This encodes a single move. Sidenote: This is called Moove, since Move is a keyword in Rust...
/// It knows where a piece moved from and where it moved to.
/// Also stores to which piece a pawn promoted if one did at all.
///
/// PERF: This could be squeezed into a bitfield like, for example, stockfish does:
/// Around line 370: https://github.com/official-stockfish/Stockfish/blob/master/src/types.h
/// I have not done this yet for two reasons:
/// 1. I'm not sure, without any benchmarks if it gains any performance.
///    Sure, the move would be smaller, but accessing a variable would be slower, since it requires bit shifting etc.
///    In the end it comes down to a trade-off between cache locality and number of instructions per read.
/// 2. It would certainly make the code less readable.
///
/// Alternatively, each Move could store a bitboard, with one bit set where we currently are and one where we are going to.
/// To "make" this move we then would only need to xor it with the bitboard for the piece.
#[derive(Copy, Clone, Debug, Ord, Eq, PartialEq, PartialOrd)]
pub struct Moove {
    from: Square,
    to: Square,
    promotion_type: Option<Piece>,
}

impl Moove {
    /// Creates a new `Move` instance with 'promotion_type' set to 0.
    pub const fn new(from: Square, to: Square) -> Moove {
        Moove {
            from,
            to,
            promotion_type: None,
        }
    }

    pub const fn new_promotion(from: Square, to: Square, promotion_type: Piece) -> Moove {
        Moove {
            from,
            to,
            promotion_type: Some(promotion_type),
        }
    }

    pub fn get_from(&self) -> Square {
        self.from
    }

    pub fn get_to(&self) -> Square {
        self.to
    }

    pub fn get_promotion_type(&self) -> Option<Piece> {
        self.promotion_type
    }

    /// This assumes that the moved piece is a pawn and only checks if the rank changed by 2.
    pub fn is_double_pawn_push(&self) -> bool {
        self.from.abs_diff(self.to) == 16
    }

    /// This assumes that the moved piece is a king and only checks if the file changed by 2.
    /// TODO: Not sure if this is bug free but i think so lol
    pub fn is_castle(&self) -> bool {
        self.from.abs_diff(self.to) == 2
    }

    pub fn get_castle_type(&self) -> CastleType {
        if get_file(self.to) == 6 {
            CastleType::Short
        } else {
            CastleType::Long
        }
    }
}

/// Converts a `Move` instance into an uci formatted string.
impl Display for Moove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        result.push_str(&square_to_string(self.from));
        result.push_str(&square_to_string(self.to));
        result.push_str(match self.promotion_type {
            None => "",
            Some(promotion_type) => match promotion_type {
                Piece::Rook => "r",
                Piece::Knight => "n",
                Piece::Bishop => "b",
                Piece::Queen => "q",
                _ => panic!("Invalid promotion type {:?}", promotion_type),
            },
        });

        write!(f, "{}", result)
    }
}
