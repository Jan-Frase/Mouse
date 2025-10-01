use crate::backend::movegen::compile_time::move_cache_non_sliders::{
    KING_MOVES, KNIGHT_MOVES, PAWN_CAPTURE_MOVES, PAWN_QUIET_MOVES,
};
use crate::backend::movegen::moove::Moove;
use crate::backend::movegen::move_gen_pawn_util::{
    create_pawn_capture_mask, get_double_pawn_push_moves, promotion_logic,
};
use crate::backend::movegen::move_gen_sliders::get_moves_for_non_slider_piece;
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::board::bitboard_manager::BitBoardManager;
use crate::backend::state::game::game_state::GameState;
use crate::backend::state::piece::PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::backend::state::piece::{Piece, PieceColor};
use crate::backend::state::square::Square;
use crate::constants::SQUARES_AMOUNT;

/// Generates and returns all the pseudo legal moves for the current player's pieces
/// based on the provided game state. This is the entry point for the move generation.
///
/// # Parameters
///
/// * `game_state`: A reference to the current game state, which contains
///   information about the board, piece positions, and active color.
///
/// # Returns
///
/// * A `Vec<Moove>` containing all the computed legal moves for the current player's
///   pieces. Currently, this implementation only calculates the moves for the king piece.
pub fn get_pseudo_legal_moves(game_state: &GameState) -> Vec<Moove> {
    // Idea: have separate functions for each piece type
    // and then call them from here
    // each of them should iterate over their relevant bitboard
    let bitboard_manager = game_state.bit_board_manager();
    // Bitboard containing all pieces of the active color. These block moves.
    let friendly_pieces_bb = bitboard_manager.get_all_pieces_off(game_state.active_color());
    let enemy_pieces_bb = bitboard_manager.get_all_pieces_off(game_state.active_color().opposite());

    let mut all_pseudo_legal_moves = Vec::new();
    let active_color = game_state.active_color();

    gen_king_moves(
        bitboard_manager,
        friendly_pieces_bb,
        &mut all_pseudo_legal_moves,
        active_color,
    );

    gen_knight_moves(
        bitboard_manager,
        friendly_pieces_bb,
        &mut all_pseudo_legal_moves,
        active_color,
    );

    gen_pawn_moves(
        game_state,
        bitboard_manager,
        friendly_pieces_bb,
        enemy_pieces_bb,
        &mut all_pseudo_legal_moves,
        active_color,
    );

    let mut moves = get_moves_for_non_slider_piece(
        Rook,
        *bitboard_manager.get_bitboard(Piece::new(Rook, active_color)),
        friendly_pieces_bb,
        enemy_pieces_bb,
    );
    all_pseudo_legal_moves.append(&mut moves);

    let mut moves = get_moves_for_non_slider_piece(
        Bishop,
        *bitboard_manager.get_bitboard(Piece::new(Bishop, active_color)),
        friendly_pieces_bb,
        enemy_pieces_bb,
    );
    all_pseudo_legal_moves.append(&mut moves);

    let mut moves = get_moves_for_non_slider_piece(
        Queen,
        *bitboard_manager.get_bitboard(Piece::new(Queen, active_color)),
        friendly_pieces_bb,
        enemy_pieces_bb,
    );
    all_pseudo_legal_moves.append(&mut moves);

    all_pseudo_legal_moves
}

fn gen_king_moves(
    bitboard_manager: &BitBoardManager,
    friendly_pieces_bb: BitBoard,
    all_pseudo_legal_moves: &mut Vec<Moove>,
    active_color: PieceColor,
) {
    // King moves
    let piece_bitboard = bitboard_manager.get_bitboard(Piece::new(King, active_color));
    let mut moves =
        iterate_over_bitboard_for_non_slider(KING_MOVES, *piece_bitboard, friendly_pieces_bb);
    all_pseudo_legal_moves.append(&mut moves);
}

fn gen_knight_moves(
    bitboard_manager: &BitBoardManager,
    friendly_pieces_bb: BitBoard,
    all_pseudo_legal_moves: &mut Vec<Moove>,
    active_color: PieceColor,
) {
    // Knight moves
    let piece_bitboard = bitboard_manager.get_bitboard(Piece::new(Knight, active_color));
    let mut moves =
        iterate_over_bitboard_for_non_slider(KNIGHT_MOVES, *piece_bitboard, friendly_pieces_bb);
    all_pseudo_legal_moves.append(&mut moves);
}

fn gen_pawn_moves(
    game_state: &GameState,
    bitboard_manager: &BitBoardManager,
    friendly_pieces_bb: BitBoard,
    enemy_pieces_bb: BitBoard,
    all_pseudo_legal_moves: &mut Vec<Moove>,
    active_color: PieceColor,
) {
    // Quiet pawn moves
    let mut moves = iterate_over_bitboard_for_non_slider(
        PAWN_QUIET_MOVES[active_color as usize],
        *bitboard_manager.get_bitboard(Piece::new(Pawn, active_color)),
        friendly_pieces_bb | enemy_pieces_bb,
    );
    promotion_logic(&mut moves);
    all_pseudo_legal_moves.append(&mut moves);

    // Capture pawn moves
    let mut moves = iterate_over_bitboard_for_non_slider(
        PAWN_CAPTURE_MOVES[active_color as usize],
        *bitboard_manager.get_bitboard(Piece::new(Pawn, active_color)),
        create_pawn_capture_mask(game_state, enemy_pieces_bb),
    );
    promotion_logic(&mut moves);
    all_pseudo_legal_moves.append(&mut moves);

    // Double pawn push moves
    let mut moves = get_double_pawn_push_moves(
        bitboard_manager,
        active_color,
        friendly_pieces_bb | enemy_pieces_bb,
    );
    all_pseudo_legal_moves.append(&mut moves);
}

// ------------------------------------
// Move gen core logic
// ------------------------------------

fn iterate_over_bitboard_for_non_slider(
    moves_cache: [BitBoard; SQUARES_AMOUNT],
    piece_bitboard: BitBoard,
    mask_bitboard: BitBoard,
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
            convert_moves_and_mask_bb_to_moves(potential_moves_bitboard, mask_bitboard, *square);
        // Lastly, we append them :)
        moves.append(moves_for_square.as_mut());
    }

    moves
}

fn convert_moves_and_mask_bb_to_moves(
    potential_moves_bitboard: BitBoard,
    mask_bitboard: BitBoard,
    square: Square,
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
    let moves_bitboard = potential_moves_bitboard & !mask_bitboard;

    // Now take the resulting bitboard and convert all true squares to a list of squares.
    let squares_we_can_move_to = moves_bitboard.get_all_true_squares();

    // generate all the moves
    let mut moves: Vec<Moove> = Vec::with_capacity(squares_we_can_move_to.len());
    for to_square in squares_we_can_move_to {
        moves.push(Moove::new(square, to_square))
    }
    // Done :)
    moves
}
