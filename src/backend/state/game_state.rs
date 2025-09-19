use crate::backend::moove::Moove;
use crate::backend::piece::PieceColor;
use crate::backend::state::bitboard_manager::BitBoardManager;
use getset::{CloneGetters, Getters, MutGetters};

#[derive(Debug, Getters, MutGetters, CloneGetters)]
pub struct GameState {
    #[getset(get = "pub", get_mut = "pub")]
    bit_board_manager: BitBoardManager,
    #[getset(get_clone = "pub")]
    active_color: PieceColor,
}

impl GameState {
    /// Creates a new `GameState` instance with default values.
    ///
    /// # Returns
    ///
    /// A `GameState` object with:
    /// - `bit_board_manager` initialized as a new instance of `BitBoardManager`.
    /// - `active_color` set to `PieceColor::White` (indicating White's turn).
    pub fn new() -> GameState {
        GameState {
            bit_board_manager: BitBoardManager::new(),
            active_color: PieceColor::White,
        }
    }

    /// Executes a move.
    ///
    /// # Arguments
    ///
    /// * `chess_move` - A `Moove` object representing the move to be made.
    pub fn make_move(&mut self, chess_move: Moove) {
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

        self.active_color = self.active_color.opposite();
    }

    /// Reverts the last move made, restoring the board state to what it was
    /// before the move.
    ///
    /// # Arguments
    /// * `chess_move` - A `Moove` struct representing the chess move that needs to be reverted.
    pub fn unmake_move(&mut self, chess_move: Moove) {
        // Get the bitboard for the piece that was moved.
        let moved_piece_bitboard = self
            .bit_board_manager
            .get_bitboard_for_piece_at_square_mut(chess_move.to())
            .unwrap();

        // Fill the square that the piece was moved from.
        moved_piece_bitboard.fill_square(chess_move.from());

        // Clear the square it moved to.
        moved_piece_bitboard.clear_square(chess_move.to());

        self.active_color = self.active_color.opposite();
    }
}
