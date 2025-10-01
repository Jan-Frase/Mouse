use crate::backend::movegen::compile_time::move_cache_non_sliders::{
    KING_MOVES, KNIGHT_MOVES, PAWN_CAPTURE_MOVES, PAWN_DOUBLE_PUSH_MOVES, PAWN_QUIET_MOVES,
};
use crate::backend::movegen::moove::Moove;
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::board::bitboard_manager::BitBoardManager;
use crate::backend::state::game::game_state::GameState;
use crate::backend::state::piece::PieceType::{King, Knight, Pawn, Queen};
use crate::backend::state::piece::{Piece, PieceColor, PieceType};
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
    let friendly_pieces_bitboard = bitboard_manager.get_all_pieces_off(game_state.active_color());
    let enemy_pieces_bitboard =
        bitboard_manager.get_all_pieces_off(game_state.active_color().opposite());

    let mut all_pseudo_legal_moves = Vec::new();
    let active_color = game_state.active_color();

    // King moves
    let piece_bitboard = bitboard_manager.get_bitboard(Piece::new(King, active_color));
    let mut moves =
        get_moves_for_non_slider_piece(KING_MOVES, *piece_bitboard, friendly_pieces_bitboard);
    all_pseudo_legal_moves.append(&mut moves);

    // Knight moves
    let piece_bitboard = bitboard_manager.get_bitboard(Piece::new(Knight, active_color));
    let mut moves =
        get_moves_for_non_slider_piece(KNIGHT_MOVES, *piece_bitboard, friendly_pieces_bitboard);
    all_pseudo_legal_moves.append(&mut moves);

    // Quiet pawn moves
    let piece_bitboard = bitboard_manager.get_bitboard(Piece::new(Pawn, active_color));
    let mut moves = get_moves_for_non_slider_piece(
        PAWN_QUIET_MOVES[active_color as usize],
        *piece_bitboard,
        friendly_pieces_bitboard | enemy_pieces_bitboard,
    );
    promotion_logic(&mut moves);
    all_pseudo_legal_moves.append(&mut moves);

    // Capture pawn moves
    let mut pawn_capture_mask = enemy_pieces_bitboard;
    match game_state
        .irreversible_data_stack()
        .last()
        .unwrap()
        .en_passant_square()
    {
        None => {}
        Some(ep_square) => {
            pawn_capture_mask.fill_square(ep_square);
        }
    }
    pawn_capture_mask = !pawn_capture_mask;
    let piece_bitboard = bitboard_manager.get_bitboard(Piece::new(Pawn, active_color));
    let mut moves = get_moves_for_non_slider_piece(
        PAWN_CAPTURE_MOVES[active_color as usize],
        *piece_bitboard,
        pawn_capture_mask,
    );
    promotion_logic(&mut moves);
    all_pseudo_legal_moves.append(&mut moves);

    // Double pawn push
    let mut moves = get_double_pawn_push_moves(
        bitboard_manager,
        active_color,
        friendly_pieces_bitboard | enemy_pieces_bitboard,
    );
    all_pseudo_legal_moves.append(&mut moves);

    all_pseudo_legal_moves
}

fn get_double_pawn_push_moves(
    bitboard_manager: &BitBoardManager,
    active_color: PieceColor,
    all_pieces_bb: BitBoard,
) -> Vec<Moove> {
    let mut moves: Vec<Moove> = Vec::new();

    let mut pawn_bitboard = bitboard_manager
        .get_bitboard(Piece::new(Pawn, active_color))
        .clone();

    let starting_bitboard = match active_color {
        PieceColor::White => BitBoard::new_from_rank(1),
        PieceColor::Black => BitBoard::new_from_rank(6),
    };

    pawn_bitboard &= starting_bitboard;

    for square in pawn_bitboard.get_all_true_squares() {
        let mut single_push_bb = PAWN_QUIET_MOVES[active_color as usize][square.square_to_index()];
        single_push_bb = single_push_bb & all_pieces_bb;
        if !single_push_bb.is_empty() {
            continue;
        }

        let mut double_push_bb =
            PAWN_DOUBLE_PUSH_MOVES[active_color as usize][square.square_to_index()];
        double_push_bb = double_push_bb & all_pieces_bb;
        if !double_push_bb.is_empty() {
            continue;
        }

        let moove = Moove::new(
            square,
            square
                .forward_by_one(active_color)
                .forward_by_one(active_color),
        );
        moves.push(moove);
    }

    moves
}

fn promotion_logic(moves: &mut Vec<Moove>) {
    if moves.is_empty() {
        return;
    }
    for index in (0..moves.len()).rev() {
        let moove = moves[index];
        if moove.to().is_on_promotion_rank() {
            for piece_type in PieceType::get_promotable_types() {
                if piece_type == Queen {
                    moves[index].set_promotion_type(Some(Queen));
                    continue;
                }
                let mut moove = moove.clone();
                moove.set_promotion_type(Some(piece_type));
                moves.push(moove);
            }
        }
    }
}

fn get_moves_for_non_slider_piece(
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
