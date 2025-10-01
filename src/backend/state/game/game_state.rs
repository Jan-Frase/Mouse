use crate::backend::movegen::moove::Moove;
use crate::backend::state::board::bitboard_manager::BitBoardManager;
use crate::backend::state::game::fen_parser::parse_fen;
use crate::backend::state::game::irreversible_data::IrreversibleData;
use crate::backend::state::piece::PieceType::Pawn;
use crate::backend::state::piece::{Piece, PieceColor};
use getset::{CloneGetters, Getters, MutGetters};

#[derive(Debug, Getters, MutGetters, CloneGetters)]
pub struct GameState {
    #[getset(get = "pub")]
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
            irreversible_data_stack: vec![IrreversibleData::new()],
            half_move_clock: 0,
        }
    }

    /// Creates a new `GameState` instance based on the fen string.
    pub fn new_from_fen(fen_string: &str) -> GameState {
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

        // Get the type of moved piece.
        let moved_piece = self
            .bit_board_manager
            .get_piece_at_square(moove.from())
            .unwrap();

        // Usually the piece something was captured on (if something was captured at all) the square we moved to...
        let mut capture_square = moove.to();

        // ... unless this is an en passant capture ...
        let ep_square = self
            .irreversible_data_stack
            .last()
            .unwrap()
            .en_passant_square();

        // if we moved a pawn and an en passant square exists
        if let Some(ep_square) = ep_square
            && moved_piece.piece_type() == Pawn
        {
            // and if we moved to the ep_square
            if ep_square == moove.to() {
                // update the captured square to the ep_square - offset
                capture_square = moove.to().back_by_one(self.active_color);
            }
        }

        // Get the type of the captured piece if it exists.
        let captured_piece = self.bit_board_manager.get_piece_at_square(capture_square);

        // Clear the square on the captured piece's bitboard if it exists.
        if let Some(captured_piece) = captured_piece {
            // Store the captured piece type in the irreversible data.
            irreversible_data.set_captured_piece(Some(captured_piece.piece_type()));
            // Remove the captured piece from its bitboard.
            let captured_piece_bitboard = self.bit_board_manager.get_bitboard_mut(captured_piece);
            captured_piece_bitboard.clear_square(capture_square);
        }

        // Check if a double pawn push was played and store the en passant file
        if moved_piece.piece_type() == Pawn && moove.is_double_pawn_push() {
            // the pawn starting square and one forward
            let ep_square = moove.to().back_by_one(self.active_color);

            irreversible_data.set_en_passant_square(Some(ep_square));
        }

        // Get the bitboard for the piece that was moved.
        let mut moved_piece_bitboard = self.bit_board_manager.get_bitboard_mut(moved_piece);

        // Clear the square that the piece was moved from.
        moved_piece_bitboard.clear_square(moove.from());

        // Update the moved piece bb if it was a pawn promotion
        match moove.promotion_type() {
            None => {}
            Some(promotion_type) => {
                moved_piece_bitboard = self
                    .bit_board_manager
                    .get_bitboard_mut(Piece::new(promotion_type, self.active_color));
            }
        }
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
    pub fn unmake_move(&mut self, moove: Moove) {
        // Flip whose turn it is.
        self.active_color = self.active_color.opposite();
        // Get the last irreversible data.
        let irreversible_data = self.irreversible_data_stack.pop().unwrap();

        // Get the bitboard for the piece that was moved.
        let mut moved_piece_bitboard = self
            .bit_board_manager
            .get_bitboard_for_piece_at_square_mut(moove.to())
            .unwrap();

        // Clear the square it moved to.
        moved_piece_bitboard.clear_square(moove.to());

        // Update the moved piece bb if it was a pawn promotion
        match moove.promotion_type() {
            None => {}
            Some(_) => {
                moved_piece_bitboard = self
                    .bit_board_manager
                    .get_bitboard_mut(Piece::new(Pawn, self.active_color));
            }
        }

        // Fill the square that the piece was moved from.
        moved_piece_bitboard.fill_square(moove.from());

        // If some piece was captured, put it back on the board.
        if let Some(captured_piece) = irreversible_data.captured_piece() {
            let piece = Piece::new(captured_piece, self.active_color.opposite());
            let bitboard = self.bit_board_manager.get_bitboard_mut(piece);

            let mut capture_square = moove.to();
            // en passant
            let en_passant_square = self
                .irreversible_data_stack
                .last()
                .unwrap()
                .en_passant_square();

            if let Some(en_passant_square) = en_passant_square
                && moove.to() == en_passant_square
            {
                capture_square = moove.to().back_by_one(self.active_color);
            }

            bitboard.fill_square(capture_square);
        }
    }
}
