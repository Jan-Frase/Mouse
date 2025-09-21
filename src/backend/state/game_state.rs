use crate::backend::moove::Moove;
use crate::backend::piece::{Piece, PieceColor};
use crate::backend::state::bitboard_manager::BitBoardManager;
use crate::backend::state::fen_parser::parse_fen;
use crate::backend::state::irreversible_data::IrreversibleData;
use getset::{CloneGetters, Getters, MutGetters};

#[derive(Debug, Getters, MutGetters, CloneGetters)]
pub struct GameState {
    // TODO: Remove this mutable getter. It`s only needed for the tests atm.
    #[getset(get = "pub", get_mut = "pub")]
    bit_board_manager: BitBoardManager,
    #[getset(get = "pub")]
    irreversible_data_stack: Vec<IrreversibleData>,
    #[getset(get_clone = "pub")]
    active_color: PieceColor,
    #[getset(get = "pub")]
    half_move_clock: u16,
}

impl GameState {
    /// Creates a new `GameState` instance with default values.
    pub fn new() -> GameState {
        GameState {
            bit_board_manager: BitBoardManager::new(),
            active_color: PieceColor::White,
            irreversible_data_stack: vec![],
            half_move_clock: 0,
        }
    }

    /// Creates a new `GameState` instance based on the fen string.
    pub fn new_parse_fen(fen_string: &str) -> GameState {
        let mut bit_board_manager = BitBoardManager::new();
        let mut active_color = PieceColor::White;
        let mut irreversible_data = IrreversibleData::new();
        let mut half_move_clock = 0;

        parse_fen(
            fen_string,
            &mut bit_board_manager,
            &mut active_color,
            &mut irreversible_data,
            &mut half_move_clock,
        );

        GameState {
            bit_board_manager,
            active_color,
            irreversible_data_stack: vec![irreversible_data],
            half_move_clock,
        }
    }

    /// Executes a move.
    ///
    /// # Arguments
    ///
    /// * `chess_move` - A `Moove` object representing the move to be made.
    pub fn make_move(&mut self, moove: Moove) {
        // The new irreversible data.
        let mut irreversible_data = IrreversibleData::new();

        // Get the bitboard for the piece that was captured if it exists.
        let captured_piece = self.bit_board_manager.get_piece_at_square(moove.to());

        // Clear the square on the captured piece's bitboard if it exists.
        if let Some(captured_piece) = captured_piece {
            // Store the captured piece type in the irreversible data.
            irreversible_data.set_captured_piece(Some(captured_piece.piece_type()));
            // Remove the captured piece from its bitboard.
            let captured_piece_bitboard = self.bit_board_manager.get_bitboard_mut(captured_piece);
            captured_piece_bitboard.clear_square(moove.to());
        }

        // Get the bitboard for the piece that was moved.
        let moved_piece_bitboard = self
            .bit_board_manager
            .get_bitboard_for_piece_at_square_mut(moove.from())
            .unwrap();

        // Clear the square that the piece was moved from.
        moved_piece_bitboard.clear_square(moove.from());

        // Fill the square it moved to.
        moved_piece_bitboard.fill_square(moove.to());

        // Take care of some basics.
        self.active_color = self.active_color.opposite();
        self.irreversible_data_stack.push(irreversible_data);
    }

    /// Reverts the last move made, restoring the board state to what it was
    /// before the move.
    ///
    /// # Arguments
    /// * `chess_move` - A `Moove` struct representing the chess move that needs to be reverted.
    pub fn unmake_move(&mut self, chess_move: Moove) {
        // Flip whose turn it is.
        self.active_color = self.active_color.opposite();

        // Get the bitboard for the piece that was moved.
        let moved_piece_bitboard = self
            .bit_board_manager
            .get_bitboard_for_piece_at_square_mut(chess_move.to())
            .unwrap();

        // Fill the square that the piece was moved from.
        moved_piece_bitboard.fill_square(chess_move.from());

        // Clear the square it moved to.
        moved_piece_bitboard.clear_square(chess_move.to());

        // Get the last irreversible data.
        let irreversible_data = self.irreversible_data_stack.pop().unwrap();

        // If some piece was captured, put it back on the board.
        if let Some(captured_piece) = irreversible_data.captured_piece() {
            let piece = Piece::new(captured_piece, self.active_color.opposite());
            let bitboard = self.bit_board_manager.get_bitboard_mut(piece);
            bitboard.fill_square(chess_move.to());
        }
    }
}
