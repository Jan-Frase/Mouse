use crate::backend::bitboard::BitBoard;
use crate::backend::r#move::Move;
use crate::backend::square::Square;
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
            let current_square = Square {
                file: square.file + file_offset,
                rank: square.rank + rank_offset,
            };

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
pub const KING_MOVES: [BitBoard; SQUARES_AMOUNT] = init_king_moves();

/// TODO: This should be the same for all non sliding pieces: pawns, knights and kings.
/// Thus we should use this method for all of them and refactor it for that purpose.
pub fn get_king_moves(square: Square, friendly_pieces_bitboard: BitBoard) -> Vec<Move> {
    // This is a bitboard with a 1 at every square the piece can move to if nothing is blocking it.
    // For a king at A1 it would look like this:
    //  _ _ _ _ _ _ _ _
    //  _ _ _ _ _ _ _ _
    //  _ _ _ _ _ _ _ _
    //  _ _ _ _ _ _ _ _
    //  _ _ _ _ _ _ _ _
    //  _ _ _ _ _ _ _ _
    //  X X _ _ _ _ _ _
    //  _ X _ _ _ _ _ _
    let potential_moves_bitboard = KING_MOVES[square.square_to_index() as usize];

    // The friendly pieces bitboard might look like this if we have the king on A1 and a pawn on A2 and B2:
    //  _ _ _ _ _ _ _ _
    //  _ _ _ _ _ _ _ _
    //  _ _ _ _ _ _ _ _
    //  _ _ _ _ _ _ _ _
    //  _ _ _ _ _ _ _ _
    //  _ _ _ _ _ _ _ _
    //  X X _ _ _ _ _ _
    //  X _ _ _ _ _ _ _

    // If we negate it, it represents all squares that are either empty or occupied by an enemy and looks like this:
    //  X X X X X X X X
    //  X X X X X X X X
    //  X X X X X X X X
    //  X X X X X X X X
    //  X X X X X X X X
    //  X X X X X X X X
    //  _ _ X X X X X X
    //  _ X X X X X X X

    // We can then and this negated bitboard with the potential moves' bitboard.
    // This will result in a bitboard with a 1 at every square the piece can move to.
    // Which looks like this:
    //  _ _ _ _ _ _ _ _
    //  _ _ _ _ _ _ _ _
    //  _ _ _ _ _ _ _ _
    //  _ _ _ _ _ _ _ _
    //  _ _ _ _ _ _ _ _
    //  _ _ _ _ _ _ _ _
    //  _ _ _ _ _ _ _ _
    //  _ X _ _ _ _ _ _
    let moves_bitboard = potential_moves_bitboard & !friendly_pieces_bitboard;

    // Now take the resulting bitboard and convert all true squares to a list of squares.
    let squares_we_can_move_to = moves_bitboard.get_all_true_squares();

    // generate all the moves
    let mut moves: Vec<Move> = Vec::new();
    for to_square in squares_we_can_move_to {
        moves.push(Move::new(square, to_square))
    }
    // Done :)
    moves
}
