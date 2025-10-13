use crate::backend::movegen::check_decider::is_in_check_on_square;
use crate::backend::movegen::moove::{CastleType, Moove};
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::game::irreversible_data::IrreversibleData;
use crate::backend::state::game::state::State;
use crate::backend::state::piece::Side;
use crate::backend::state::square::{B1, B8, C1, C8, D1, D8, E1, E8, F1, F8, G1, G8, Square};

// Made these values with: https://tearth.dev/bitboard-viewer/
const WHITE_LONG_CASTLE_MASK: BitBoard = BitBoard::new_from_squares(&[B1, C1, D1]);
const WHITE_SHORT_CASTLE_MASK: BitBoard = BitBoard::new_from_squares(&[F1, G1]);
const BLACK_LONG_CASTLE_MASK: BitBoard = BitBoard::new_from_squares(&[B8, C8, D8]);
const BLACK_SHORT_CASTLE_MASK: BitBoard = BitBoard::new_from_squares(&[F8, G8]);

const WHITE_LONG_CASTLE_MOVE: Moove = Moove::new(E1, C1);
const WHITE_SHORT_CASTLE_MOVE: Moove = Moove::new(E1, G1);
const BLACK_LONG_CASTLE_MOVE: Moove = Moove::new(E8, C8);
const BLACK_SHORT_CASTLE_MOVE: Moove = Moove::new(E8, G8);

const WHITE_LONG_CASTLE_CHECK_SQUARES: [Square; 3] = [E1, D1, C1];
const WHITE_SHORT_CASTLE_CHECK_SQUARES: [Square; 3] = [E1, F1, G1];
const BLACK_LONG_CASTLE_CHECK_SQUARES: [Square; 3] = [E8, D8, C8];
const BLACK_SHORT_CASTLE_CHECK_SQUARES: [Square; 3] = [E8, F8, G8];

pub fn gen_castles(moves: &mut Vec<Moove>, game_state: &State, combined_bb: BitBoard) {
    let irreversible_data = &game_state.irreversible_data;

    for castle_type in CastleType::get_all_types() {
        let (castling_rights, squares_the_king_moves_through, between_king_rook_bb, moove) =
            get_needed_constants(irreversible_data, &castle_type, game_state.active_color);

        gen_castle(
            moves,
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
    game_state: &State,
    combined_bb: BitBoard,
    castling_rights: bool,
    squares_the_king_moves_through: [Square; 3],
    between_king_rook_bb: BitBoard,
    moove: Moove,
) {
    // do we have castling rights for this type of castle?
    if !castling_rights {
        return;
    }

    // are we moving through checks?
    for square in squares_the_king_moves_through.iter() {
        // if so -> stop
        if is_in_check_on_square(game_state, game_state.active_color, *square) {
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
    piece_color: Side,
) -> (bool, [Square; 3], BitBoard, Moove) {
    match castle_types {
        CastleType::Long => match piece_color {
            Side::White => (
                irreversible_data.get_long_castle_rights(piece_color),
                WHITE_LONG_CASTLE_CHECK_SQUARES,
                WHITE_LONG_CASTLE_MASK,
                WHITE_LONG_CASTLE_MOVE,
            ),
            Side::Black => (
                irreversible_data.get_long_castle_rights(piece_color),
                BLACK_LONG_CASTLE_CHECK_SQUARES,
                BLACK_LONG_CASTLE_MASK,
                BLACK_LONG_CASTLE_MOVE,
            ),
        },
        CastleType::Short => match piece_color {
            Side::White => (
                irreversible_data.get_short_castle_rights(piece_color),
                WHITE_SHORT_CASTLE_CHECK_SQUARES,
                WHITE_SHORT_CASTLE_MASK,
                WHITE_SHORT_CASTLE_MOVE,
            ),
            Side::Black => (
                irreversible_data.get_short_castle_rights(piece_color),
                BLACK_SHORT_CASTLE_CHECK_SQUARES,
                BLACK_SHORT_CASTLE_MASK,
                BLACK_SHORT_CASTLE_MOVE,
            ),
        },
    }
}
