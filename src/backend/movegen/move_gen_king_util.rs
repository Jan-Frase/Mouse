use crate::backend::movegen::check_decider::is_in_check_on_square;
use crate::backend::movegen::moove::{CastleType, Moove};
use crate::backend::state::board::bitboard::Bitboard;
use crate::backend::state::game::game_state::GameState;
use crate::backend::state::game::irreversible_data::IrreversibleData;
use crate::backend::state::piece::PieceColor;
use crate::backend::state::square::Square;

// Made these values with: https://tearth.dev/bitboard-viewer/
const WHITE_LONG_CASTLE_MASK: Bitboard = Bitboard::new_from_value(14);
const WHITE_SHORT_CASTLE_MASK: Bitboard = Bitboard::new_from_value(96);
const BLACK_LONG_CASTLE_MASK: Bitboard = Bitboard::new_from_value(1008806316530991104);
const BLACK_SHORT_CASTLE_MASK: Bitboard = Bitboard::new_from_value(6917529027641081856);

const WHITE_LONG_CASTLE_MOVE: Moove = Moove::new(Square::new(4, 0), Square::new(2, 0));
const WHITE_SHORT_CASTLE_MOVE: Moove = Moove::new(Square::new(4, 0), Square::new(6, 0));
const BLACK_LONG_CASTLE_MOVE: Moove = Moove::new(Square::new(4, 7), Square::new(2, 7));
const BLACK_SHORT_CASTLE_MOVE: Moove = Moove::new(Square::new(4, 7), Square::new(6, 7));

const WHITE_LONG_CASTLE_CHECK_SQUARES: [Square; 2] = [Square::new(3, 0), Square::new(2, 0)];
const WHITE_SHORT_CASTLE_CHECK_SQUARES: [Square; 2] = [Square::new(5, 0), Square::new(6, 0)];
const BLACK_LONG_CASTLE_CHECK_SQUARES: [Square; 2] = [Square::new(3, 7), Square::new(2, 7)];
const BLACK_SHORT_CASTLE_CHECK_SQUARES: [Square; 2] = [Square::new(5, 7), Square::new(6, 7)];

pub fn gen_castles(
    all_pseudo_legal_moves: &mut Vec<Moove>,
    game_state: &GameState,
    combined_bb: Bitboard,
) {
    let irreversible_data = game_state.irreversible_data_stack().last().unwrap();

    for castle_type in CastleType::get_all_types() {
        let (castling_rights, squares_the_king_moves_through, between_king_rook_bb, moove) =
            get_needed_constants(irreversible_data, &castle_type, game_state.active_color());

        gen_castle(
            all_pseudo_legal_moves,
            game_state,
            combined_bb,
            castling_rights,
            squares_the_king_moves_through,
            between_king_rook_bb,
            moove,
        );
    }
}

fn gen_castle(
    all_pseudo_legal_moves: &mut Vec<Moove>,
    game_state: &GameState,
    combined_bb: Bitboard,
    castling_rights: bool,
    squares_the_king_moves_through: [Square; 2],
    between_king_rook_bb: Bitboard,
    moove: Moove,
) {
    // do we have castling rights for this type of castle?
    if !castling_rights {
        return;
    }

    // are we moving through checks?
    for square in squares_the_king_moves_through.iter() {
        // if so -> stop
        if is_in_check_on_square(game_state, game_state.active_color(), *square) {
            return;
        }
    }

    // are the squares between the king and the rook empty?
    // TODO: if we had attack bbs, we could also and them here
    let squares_between = combined_bb & between_king_rook_bb;
    // if something is in the way -> stop
    if !squares_between.is_empty() {
        return;
    }

    all_pseudo_legal_moves.push(moove);
}

fn get_needed_constants(
    irreversible_data: &IrreversibleData,
    castle_types: &CastleType,
    piece_color: PieceColor,
) -> (bool, [Square; 2], Bitboard, Moove) {
    match castle_types {
        CastleType::Long => match piece_color {
            PieceColor::White => (
                irreversible_data.get_long_castle_rights(piece_color),
                WHITE_LONG_CASTLE_CHECK_SQUARES,
                WHITE_LONG_CASTLE_MASK,
                WHITE_LONG_CASTLE_MOVE,
            ),
            PieceColor::Black => (
                irreversible_data.get_long_castle_rights(piece_color),
                BLACK_LONG_CASTLE_CHECK_SQUARES,
                BLACK_LONG_CASTLE_MASK,
                BLACK_LONG_CASTLE_MOVE,
            ),
        },
        CastleType::Short => match piece_color {
            PieceColor::White => (
                irreversible_data.get_short_castle_rights(piece_color),
                WHITE_SHORT_CASTLE_CHECK_SQUARES,
                WHITE_SHORT_CASTLE_MASK,
                WHITE_SHORT_CASTLE_MOVE,
            ),
            PieceColor::Black => (
                irreversible_data.get_short_castle_rights(piece_color),
                BLACK_SHORT_CASTLE_CHECK_SQUARES,
                BLACK_SHORT_CASTLE_MASK,
                BLACK_SHORT_CASTLE_MOVE,
            ),
        },
    }
}
