use crate::backend::moove::Moove;
use crate::backend::square::Square;
use crate::backend::state::bitboard::BitBoard;
use crate::constants::SQUARES_AMOUNT;

/// Calculates all possible moves for a given piece or set of pieces.
///
/// # Parameters
/// - `moves_cache`: A precomputed array of potential moves for each square on the board, represented
///   as a [`BitBoard`] array.
/// - `piece_bitboard`: A [`BitBoard`] representing the position(s) of the piece(s) whose moves
///   need to be calculated.
/// - `friendly_pieces_bitboard`: A [`BitBoard`] representing the positions of friendly pieces
///   on the board. These pieces are used to determine which moves are blocked and thus not valid.
///
/// # Returns
/// A `Vec<Moove>` containing all the possible moves for the given piece(s).
pub fn get_moves_for_non_slider_piece(
    moves_cache: [BitBoard; SQUARES_AMOUNT],
    piece_bitboard: BitBoard,
    friendly_pieces_bitboard: BitBoard,
) -> Vec<Moove> {
    // PERF: Instead of creating a new vector for each piece, we could reuse the same vector and append to it.
    let mut moves: Vec<Moove> = Vec::new();

    // Example: We are doing this for all knights.
    // The `moves_cache` array would for each square contain all viable moves for a knight.

    // Assuming we are in the starting position as white `squares_with_piece` would be [B1, G1].
    let squares_with_piece = piece_bitboard.get_all_true_squares();
    // We then iterate over all these squares...
    for square in squares_with_piece.iter() {
        // ... get the potential moves for the piece on that square...
        // SLIDER: (This only works this easily for non-sliders)
        let potential_moves_bitboard = moves_cache[square.square_to_index()];
        //... and get all the moves for the piece on that square.
        let mut moves_for_square =
            get_moves_for_square(potential_moves_bitboard, *square, friendly_pieces_bitboard);
        // Lastly, we append them :)
        moves.append(moves_for_square.as_mut());
    }

    moves
}

/// Computes all possible moves for a given square and friendly piece positions.
/// SLIDER: Should also work for sliders.
///
/// # Parameters
///
/// - `moves_cache`: A precomputed array of `BitBoard`s where each entry represents
///   the potential moves for a piece on a specific square if no obstructions are present.
/// - `square`: The `Square` for which moves are being calculated. This represents
///   the current position of the piece.
/// - `friendly_pieces_bitboard`: A `BitBoard` representing positions of all friendly
///   pieces. Pieces located in these positions will block movement.
///
/// # Returns
///
/// A `Vec` of `Moove` structs that represent all legal moves
/// the piece on the provided square can make.
fn get_moves_for_square(
    potential_moves_bitboard: BitBoard,
    square: Square,
    friendly_pieces_bitboard: BitBoard,
) -> Vec<Moove> {
    // SLIDER: I think the following code should also work for sliders.

    // `potential_moves_bitboard` is a BitBoard with a 1 at every square the piece can move to if nothing is blocking it.
    // For a king at A1 it would look like this:
    //  _ _ _ _ _ _ _ _
    //  _ _ _ _ _ _ _ _
    //  _ _ _ _ _ _ _ _
    //  _ _ _ _ _ _ _ _
    //  _ _ _ _ _ _ _ _
    //  _ _ _ _ _ _ _ _
    //  X X _ _ _ _ _ _
    //  _ X _ _ _ _ _ _

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
    let mut moves: Vec<Moove> = Vec::with_capacity(squares_we_can_move_to.len() + 20);
    for to_square in squares_we_can_move_to {
        moves.push(Moove::new(square, to_square))
    }
    // Done :)
    moves
}
