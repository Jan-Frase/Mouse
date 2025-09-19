use crate::backend::moove::Moove;
use crate::backend::square::Square;
use crate::backend::state::bitboard::BitBoard;
use crate::constants::SQUARES_AMOUNT;

pub fn get_moves_for_piece(
    moves_cache: [BitBoard; SQUARES_AMOUNT],
    piece_bitboard: BitBoard,
    friendly_pieces_bitboard: BitBoard,
) -> Vec<Moove> {
    let mut moves: Vec<Moove> = Vec::new();

    let squares_with_piece = piece_bitboard.get_all_true_squares();
    for square in squares_with_piece.iter() {
        let mut moves_for_square =
            get_moves_for_square(moves_cache, *square, friendly_pieces_bitboard);
        moves.append(moves_for_square.as_mut());
    }

    moves
}

fn get_moves_for_square(
    moves_cache: [BitBoard; SQUARES_AMOUNT],
    square: Square,
    friendly_pieces_bitboard: BitBoard,
) -> Vec<Moove> {
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
    let potential_moves_bitboard = moves_cache[square.square_to_index() as usize];

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
    let mut moves: Vec<Moove> = Vec::new();
    for to_square in squares_we_can_move_to {
        moves.push(Moove::new(square, to_square))
    }
    // Done :)
    moves
}
