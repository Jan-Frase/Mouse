use crate::backend::state::piece::Piece;
use crate::backend::state::piece::Piece::{Bishop, Knight, Queen, Rook};
use crate::backend::state::square::Square;
use std::fmt::{Display, Formatter};

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
    pub from: Square,
    pub to: Square,
    pub promotion_type: Option<Piece>,
}

impl Moove {
    /// Creates a new `Move` instance with 'promotion_type' set to 0.
    pub const fn new(from: Square, to: Square) -> Moove {
        Moove {
            from,
            to,
            promotion_type: Option::None,
        }
    }

    /// This assumes that the moved piece is a pawn and only checks if the rank changed by 2.
    pub fn is_double_pawn_push(&self) -> bool {
        (self.from.rank() - self.to.rank()).abs() == 2
    }

    /// This assumes that the moved piece is a king and only checks if the file changed by 2.
    pub fn is_castle(&self) -> bool {
        (self.from.file() - self.to.file()).abs() == 2
    }

    /// This assumes that the moved piece is a king and only checks if the target file is 6.
    fn is_short_castle(&self) -> bool {
        self.to.file() == 6
    }

    /// This assumes that the moved piece is a king and only checks if the target file is 2.
    fn is_long_castle(&self) -> bool {
        self.to.file() == 2
    }

    pub fn get_castle_type(&self) -> CastleType {
        if self.is_short_castle() {
            CastleType::Short
        } else {
            CastleType::Long
        }
    }

    pub fn new_from_uci_notation(uci_notation: &str) -> Moove {
        let from = Square::new_from_uci_notation(&uci_notation[0..2]);
        let to = Square::new_from_uci_notation(&uci_notation[2..4]);

        let promotion_char = uci_notation.chars().nth(4);
        let promotion_type = match promotion_char {
            None => Option::None,
            Some(char) => match char {
                'r' => Some(Rook),
                'n' => Some(Knight),
                'b' => Some(Bishop),
                'q' => Some(Queen),
                _ => panic!("Invalid promotion type {:?}", uci_notation),
            },
        };

        Moove {
            from,
            to,
            promotion_type,
        }
    }
}

/// Converts a `Move` instance into an uci formatted string.
impl Display for Moove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        result.push_str(&self.from.to_string());
        result.push_str(&self.to.to_string());
        result.push_str(match self.promotion_type {
            None => "",
            Some(promotion_type) => match promotion_type {
                Rook => "r",
                Knight => "n",
                Bishop => "b",
                Queen => "q",
                _ => panic!("Invalid promotion type {:?}", promotion_type),
            },
        });

        write!(f, "{}", result)
    }
}
