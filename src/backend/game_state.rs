use crate::backend::bitboard_manager::BitBoardManager;
use crate::backend::chess_move::ChessMove;
use crate::backend::piece::PieceColor;
use crate::backend::piece::PieceColor::{Black, White};
use getset::{CloneGetters, Getters, MutGetters};

#[derive(Getters, MutGetters, CloneGetters)]
pub struct GameState {
    #[getset(get = "pub", get_mut = "pub")]
    bit_board_manager: BitBoardManager,
    #[getset(get_clone = "pub")]
    active_color: PieceColor,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            bit_board_manager: BitBoardManager::new(),
            active_color: PieceColor::White,
        }
    }
    pub fn make_move(&mut self, chess_move: ChessMove) {
        // Get the bitboard for the piece that was captured if it exists.
        let captured_piece_bitboard = self
            .bit_board_manager
            .get_bitboard_for_piece_at_square_mut(chess_move.to());

        // Clear the square on the captured piece's bitboard if it exists.
        if let Some(captured_piece_bitboard) = captured_piece_bitboard {
            captured_piece_bitboard.clear_square(chess_move.to());
        }

        // Get the bitboard for the piece that was moved.
        let moved_piece_bitboard = self
            .bit_board_manager
            .get_bitboard_for_piece_at_square_mut(chess_move.from())
            .unwrap();

        // Clear the square that the piece was moved from.
        moved_piece_bitboard.clear_square(chess_move.from());

        // Fill the square it moved to.
        moved_piece_bitboard.fill_square(chess_move.to());

        self.active_color = match self.active_color() {
            PieceColor::White => Black,
            PieceColor::Black => PieceColor::White,
        }
    }

    pub fn unmake_move(&mut self, chess_move: ChessMove) {
        // Get the bitboard for the piece that was moved.
        let moved_piece_bitboard = self
            .bit_board_manager
            .get_bitboard_for_piece_at_square_mut(chess_move.to())
            .unwrap();

        // Fill the square that the piece was moved from.
        moved_piece_bitboard.clear_square(chess_move.from());

        // Clear the square it moved to.
        moved_piece_bitboard.fill_square(chess_move.to());

        self.active_color = match self.active_color() {
            White => Black,
            Black => White,
        }
    }
}
