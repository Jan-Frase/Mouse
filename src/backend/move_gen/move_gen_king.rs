use crate::backend::square::Square;
use crate::backend::state::bitboard::BitBoard;
use crate::constants::SQUARES_AMOUNT;

/// Generates a `BitBoard` with all possible moves for a king piece from a given `Square`.
///
/// # Parameters
/// - `square`: A `Square` representing the current position of the king piece.
///
/// # Returns
/// A `BitBoard` containing all valid surrounding squares a king can move to.
const fn generate_king_move(square: Square) -> BitBoard {
    let mut bitboard = BitBoard::new();

    // since this fn is const, we can't use a loop
    // instead we use a while loop
    // to iterate over the surrounding files
    let mut file_offset = -1;
    while file_offset <= 1 {
        // and ranks
        let mut rank_offset = -1;
        while rank_offset <= 1 {
            // skip the current square
            if file_offset == 0 && rank_offset == 0 {
                rank_offset += 1;
                continue;
            }
            // create the relevant square
            let current_square =
                Square::new(square.file() + file_offset, square.rank() + rank_offset);

            // and add it if it's valid
            if current_square.is_valid() {
                bitboard.fill_square(current_square);
            }

            rank_offset += 1;
        }

        file_offset += 1;
    }
    bitboard
}

/// Initializes a collection of bitboards representing all possible king moves for each square.
///
/// Since this function is const, it can be evaluated at compile time.
///
/// # Returns
/// An array of `BitBoard` of size `SQUARES_AMOUNT`, where each entry corresponds to the
/// possible king moves for the square at the same index.
pub const fn init_king_moves() -> [BitBoard; SQUARES_AMOUNT] {
    let mut attacks = [BitBoard::new(); SQUARES_AMOUNT];

    // iterate over all squares
    let mut square_index: usize = 0;
    while square_index < SQUARES_AMOUNT {
        // generate a square struct from the index
        let square = Square::index_to_square(square_index as i8);

        // and generate the king moves for that square
        attacks[square_index] = generate_king_move(square);

        square_index += 1;
    }

    attacks
}

/// All of this gets generated at compile time, in the functions above.
/// At runtime, we only have to read the values.
/// It contains for each square, a bitboard with every square that the king could potentially move to set to 1.
/// Example:
/// This: `KING_MOVES[Square::new(0,0)]` returns at bitboard that looks like this:
/// `_ _ _ _ _ _ _ _`
/// `_ _ _ _ _ _ _ _`
/// `_ _ _ _ _ _ _ _`
/// `_ _ _ _ _ _ _ _`
/// `_ _ _ _ _ _ _ _`
/// `_ _ _ _ _ _ _ _`
/// `X X _ _ _ _ _ _`
/// `_ X _ _ _ _ _ _`
pub const KING_MOVES: [BitBoard; SQUARES_AMOUNT] = init_king_moves();
