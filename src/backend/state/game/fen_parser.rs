use crate::backend::state::board::bitboard_manager::BitboardManager;
use crate::backend::state::game::irreversible_data::IrreversibleData;
use crate::backend::state::piece::{Piece, PieceColor, PieceType};
use crate::backend::state::square::Square;

/// Parses a FEN (Forsyth-Edwards Notation) string and updates the corresponding game state.
/// https://www.chessprogramming.org/Forsyth-Edwards_Notation
///
/// # Arguments
///
/// * `fen_string` - A string slice containing the FEN representation of the chess game.
/// * `bit_board_manager` - A mutable reference to a `BitBoardManager` to update the board positions.
/// * `active_color` - A mutable reference to a `PieceColor` to set the active player.
/// * `irreversible_data` - A mutable reference to `IrreversibleData`.
/// * `half_move_clock` - A mutable reference to a `u16` to update the current half-move clock count.
pub fn parse_fen(
    fen_string: &str,
    bit_board_manager: &mut BitboardManager,
    active_color: &mut PieceColor,
    irreversible_data: &mut IrreversibleData,
    half_move_clock: &mut u16,
) {
    let fen_string = fen_string.split_whitespace().collect::<Vec<&str>>();

    let positions_string = fen_string[0];
    parse_position(bit_board_manager, positions_string);

    let active_color_string = fen_string[1];
    parse_active_color(active_color, active_color_string);

    let castling_rights_string = fen_string[2];
    parse_castling_rights(irreversible_data, castling_rights_string);

    let en_passant_file_string = fen_string[3];
    parse_en_passant(irreversible_data, en_passant_file_string);

    let half_move_clock_string = fen_string[4];
    *half_move_clock = half_move_clock_string.parse::<u16>().unwrap();

    let _full_move_number_string = fen_string[5];
    // I don't store this data as it isn't used for anything.
}

fn parse_en_passant(irreversible_data: &mut IrreversibleData, en_passant_file_string: &str) {
    if en_passant_file_string == "-" {
        irreversible_data.set_en_passant_square(None);
        return;
    }

    let mut en_passant_square = Square::new(0, 0);
    for char in en_passant_file_string.chars() {
        match char {
            'a'..='h' => {
                en_passant_square.set_file(char.to_digit(36).unwrap() as i8 - 10);
            }
            '3' | '6' => {
                en_passant_square.set_rank(char.to_digit(10).unwrap() as i8 - 1);
            }
            _ => panic!("Invalid character in FEN string"),
        }
    }
    irreversible_data.set_en_passant_square(Some(en_passant_square));
}

fn parse_castling_rights(irreversible_data: &mut IrreversibleData, castling_rights_string: &str) {
    for char in castling_rights_string.chars() {
        match char {
            '-' => {
                irreversible_data.set_white_long_castle_rights(false);
                irreversible_data.set_white_short_castle_rights(false);
                irreversible_data.set_black_long_castle_rights(false);
                irreversible_data.set_black_short_castle_rights(false);
            }
            'K' => {
                irreversible_data.set_white_short_castle_rights(true);
            }
            'k' => {
                irreversible_data.set_black_short_castle_rights(true);
            }
            'Q' => {
                irreversible_data.set_white_long_castle_rights(true);
            }
            'q' => {
                irreversible_data.set_black_long_castle_rights(true);
            }
            _ => panic!("Invalid character in FEN string"),
        }
    }
}

fn parse_active_color(active_color: &mut PieceColor, active_color_string: &str) {
    match active_color_string {
        "w" => *active_color = PieceColor::White,
        "b" => *active_color = PieceColor::Black,
        _ => {
            panic!("Invalid character in FEN string");
        }
    }
}

fn parse_position(bit_board_manager: &mut BitboardManager, positions_string: &str) {
    let mut file = 0;
    let mut rank = 7;
    for char in positions_string.chars() {
        match char {
            '1'..='8' => {
                file += char.to_digit(10).unwrap() as i8;
            }
            '/' => {
                file = 0;
                rank -= 1;
            }
            'P' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::Pawn, PieceColor::White))
                    .fill_square(Square::new(file, rank));
                file += 1;
            }
            'p' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::Pawn, PieceColor::Black))
                    .fill_square(Square::new(file, rank));
                file += 1;
            }
            'R' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::Rook, PieceColor::White))
                    .fill_square(Square::new(file, rank));
                file += 1;
            }
            'r' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::Rook, PieceColor::Black))
                    .fill_square(Square::new(file, rank));
                file += 1;
            }
            'N' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::Knight, PieceColor::White))
                    .fill_square(Square::new(file, rank));
                file += 1;
            }
            'n' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::Knight, PieceColor::Black))
                    .fill_square(Square::new(file, rank));
                file += 1;
            }
            'B' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::Bishop, PieceColor::White))
                    .fill_square(Square::new(file, rank));
                file += 1;
            }
            'b' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::Bishop, PieceColor::Black))
                    .fill_square(Square::new(file, rank));
                file += 1;
            }
            'Q' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::Queen, PieceColor::White))
                    .fill_square(Square::new(file, rank));
                file += 1;
            }
            'q' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::Queen, PieceColor::Black))
                    .fill_square(Square::new(file, rank));
                file += 1;
            }
            'K' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::King, PieceColor::White))
                    .fill_square(Square::new(file, rank));
                file += 1;
            }
            'k' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::King, PieceColor::Black))
                    .fill_square(Square::new(file, rank));
                file += 1;
            }
            _ => {
                panic!("Invalid character in FEN string");
            }
        }
    }
}
