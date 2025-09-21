use crate::backend::piece::{Piece, PieceColor, PieceType};
use crate::backend::square::Square;
use crate::backend::state::bitboard_manager::BitBoardManager;
use crate::backend::state::irreversible_data::IrreversibleData;

pub fn parse_fen(
    fen_string: &str,
    bit_board_manager: &mut BitBoardManager,
    active_color: &mut PieceColor,
    irreversible_data: &mut IrreversibleData,
    half_move_clock: &mut u16,
) {
    let fen_string = fen_string.split_whitespace().collect::<Vec<&str>>();

    let positions_string = fen_string[0];
    let mut file = 0;
    let mut rank = 7;
    for char in positions_string.chars() {
        match char {
            '1'..='8' => {
                file += char.to_digit(10).unwrap() as i8 - 1;
            }
            '/' => {
                file = 0;
                rank -= 1;
            }
            'P' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::Pawn, PieceColor::White))
                    .fill_square(Square::new(file, rank));
            }
            'p' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::Pawn, PieceColor::Black))
                    .fill_square(Square::new(file, rank));
            }
            'R' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::Rook, PieceColor::White))
                    .fill_square(Square::new(file, rank));
            }
            'r' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::Rook, PieceColor::Black))
                    .fill_square(Square::new(file, rank));
            }
            'N' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::Knight, PieceColor::White))
                    .fill_square(Square::new(file, rank));
            }
            'n' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::Knight, PieceColor::Black))
                    .fill_square(Square::new(file, rank));
            }
            'B' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::Bishop, PieceColor::White))
                    .fill_square(Square::new(file, rank));
            }
            'b' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::Bishop, PieceColor::Black))
                    .fill_square(Square::new(file, rank));
            }
            'Q' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::Queen, PieceColor::White))
                    .fill_square(Square::new(file, rank));
            }
            'q' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::Queen, PieceColor::Black))
                    .fill_square(Square::new(file, rank));
            }
            'K' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::King, PieceColor::White))
                    .fill_square(Square::new(file, rank));
            }
            'k' => {
                bit_board_manager
                    .get_bitboard_mut(Piece::new(PieceType::King, PieceColor::Black))
                    .fill_square(Square::new(file, rank));
            }
            _ => {
                panic!("Invalid character in FEN string");
            }
        }
    }

    let active_color_string = fen_string[1];
    match active_color_string {
        "w" => *active_color = PieceColor::White,
        "b" => *active_color = PieceColor::Black,
        _ => {
            panic!("Invalid character in FEN string");
        }
    }

    let castling_rights_string = fen_string[2];
    for char in castling_rights_string.chars() {
        match char {
            'K' => {
                irreversible_data.set_white_long_castle_rights(true);
            }
            'k' => {
                irreversible_data.set_white_short_castle_rights(true);
            }
            'Q' => {
                irreversible_data.set_black_long_castle_rights(true);
            }
            'q' => {
                irreversible_data.set_black_short_castle_rights(true);
            }
            _ => panic!("Invalid character in FEN string"),
        }
    }

    let en_passant_file_string = fen_string[3];
    for char in en_passant_file_string.chars() {
        match char {
            '-' => {
                irreversible_data.set_en_passant_file(None);
            }
            'a'..='h' => {
                irreversible_data.set_en_passant_file(Some(char.to_digit(10).unwrap() as i8 - 1));
            }
            '3' | '6' => {
                // I don't store this data as it is redundant.
            }
            _ => panic!("Invalid character in FEN string"),
        }
    }

    let half_move_clock_string = fen_string[4];
    *half_move_clock = half_move_clock_string.parse::<u16>().unwrap();

    let full_move_number_string = fen_string[5];
    // I don't store this data as it isn't used for anything.
}
