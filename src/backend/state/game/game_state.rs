use crate::backend::movegen::moove::{CastleType, Moove};
use crate::backend::state::board::bitboard::Bitboard;
use crate::backend::state::board::bitboard_manager::BitboardManager;
use crate::backend::state::game::fen_parser::parse_fen;
use crate::backend::state::game::irreversible_data::IrreversibleData;
use crate::backend::state::piece::PieceType::{Pawn, Rook};
use crate::backend::state::piece::{Piece, PieceColor, PieceType};
use crate::backend::state::square::Square;
use getset::{CloneGetters, Getters, MutGetters};

#[derive(Debug, Getters, MutGetters, CloneGetters)]
pub struct GameState {
    #[getset(get = "pub")]
    bb_manager: BitboardManager,
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
            bb_manager: BitboardManager::new(),
            active_color: PieceColor::White,
            irreversible_data_stack: vec![IrreversibleData::new_with_castling_true()],
            half_move_clock: 0,
        }
    }

    /// Creates a new `GameState` instance based on the fen string.
    pub fn new_from_fen(fen_string: &str) -> GameState {
        let mut bit_board_manager = BitboardManager::new();
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
            bb_manager: bit_board_manager,
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
        let mut irreversible_data =
            IrreversibleData::new_from_previous_state(self.irreversible_data_stack.last().unwrap());

        // Get the type of moved piece.
        let moved_piece = self.bb_manager.get_piece_at_square(moove.from()).unwrap();

        // Usually the square something was captured on (if something was captured at all) is the square we moved to...
        let mut capture_square = moove.to();
        if moved_piece.piece_type() == Pawn {
            // ... unless this is an en passant capture, we then need to update the capture square.
            self.make_move_ep_capture(moove, &mut capture_square);
            // Check if a double pawn push was played and store the en passant file
            self.make_move_double_pawn_push(moove, &mut irreversible_data);
        }

        // If something was captured, remove the piece and update irreversible data.
        self.make_move_capture(&mut irreversible_data, capture_square);

        // Get the bitboard for the piece that was moved.
        let mut moved_piece_bitboard = self.bb_manager.get_bitboard_mut(moved_piece);

        // Clear the square that the piece was moved from.
        moved_piece_bitboard.clear_square(moove.from());

        // Update the moved piece bb if it was a pawn promotion
        match moove.promotion_type() {
            None => {}
            Some(promotion_type) => {
                moved_piece_bitboard = self
                    .bb_manager
                    .get_bitboard_mut(Piece::new(promotion_type, self.active_color));
            }
        }
        // Fill the square it moved to.
        moved_piece_bitboard.fill_square(moove.to());

        // Some special king handling
        if moved_piece.piece_type() == PieceType::King {
            // If the king moved we can't castle anymore
            irreversible_data.remove_long_castle_rights(self.active_color);
            irreversible_data.remove_short_castle_rights(self.active_color);

            // If we castled, we need to move the rook
            if moove.is_castle() {
                let mut rook_bb = self
                    .bb_manager
                    .get_bitboard_mut(Piece::new(Rook, self.active_color));
                let rook_swap_bb =
                    Self::get_rook_swap_bb(moove.get_castle_type(), self.active_color);
                rook_bb ^= rook_swap_bb;
            }
        }

        // Take care of some basics.
        self.active_color = self.active_color.opposite();
        self.irreversible_data_stack.push(irreversible_data);
    }

    fn make_move_ep_capture(&mut self, moove: Moove, capture_square: &mut Square) {
        let ep_square = self
            .irreversible_data_stack
            .last()
            .unwrap()
            .en_passant_square();

        // if an en passant square exists
        if let Some(ep_square) = ep_square
            // and if we moved to the ep_square
            && ep_square == moove.to()
        {
            // update the captured square to the ep_square - offset
            *capture_square = moove.to().back_by_one(self.active_color);
        }
    }

    fn make_move_capture(
        &mut self,
        irreversible_data: &mut IrreversibleData,
        capture_square: Square,
    ) {
        // Get the type of the captured piece if it exists.
        let captured_piece = self.bb_manager.get_piece_at_square(capture_square);
        // Clear the square on the captured piece's bitboard if it exists.
        if let Some(captured_piece) = captured_piece {
            // Store the captured piece type in the irreversible data.
            irreversible_data.set_captured_piece(Some(captured_piece.piece_type()));
            // Remove the captured piece from its bitboard.
            let captured_piece_bitboard = self.bb_manager.get_bitboard_mut(captured_piece);
            captured_piece_bitboard.clear_square(capture_square);
        }
    }

    fn make_move_double_pawn_push(
        &mut self,
        moove: Moove,
        irreversible_data: &mut IrreversibleData,
    ) {
        if moove.is_double_pawn_push() {
            // the pawn starting square and one forward
            let ep_square = moove.to().back_by_one(self.active_color);

            irreversible_data.set_en_passant_square(Some(ep_square));
        }
    }

    fn get_rook_swap_bb(castle_type: CastleType, active_color: PieceColor) -> Bitboard {
        match castle_type {
            CastleType::Long => match active_color {
                PieceColor::White => {
                    Bitboard::new_from_squares(vec![Square::new(0, 0), Square::new(3, 0)])
                }
                PieceColor::Black => {
                    Bitboard::new_from_squares(vec![Square::new(0, 7), Square::new(3, 7)])
                }
            },
            CastleType::Short => match active_color {
                PieceColor::White => {
                    Bitboard::new_from_squares(vec![Square::new(7, 0), Square::new(5, 0)])
                }
                PieceColor::Black => {
                    Bitboard::new_from_squares(vec![Square::new(7, 7), Square::new(5, 7)])
                }
            },
        }
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

        let moved_piece = self.bb_manager.get_piece_at_square(moove.to()).unwrap();
        // Get the bitboard for the piece that was moved.
        let mut moved_piece_bitboard = self.bb_manager.get_bitboard_mut(moved_piece);

        // Clear the square it moved to.
        moved_piece_bitboard.clear_square(moove.to());

        // Update the moved piece bb if it was a pawn promotion
        match moove.promotion_type() {
            None => {}
            Some(_) => {
                moved_piece_bitboard = self
                    .bb_manager
                    .get_bitboard_mut(Piece::new(Pawn, self.active_color));
            }
        }

        // Fill the square that the piece was moved from.
        moved_piece_bitboard.fill_square(moove.from());

        // Some special king handling
        if moved_piece.piece_type() == PieceType::King && moove.is_castle() {
            let mut rook_bb = self
                .bb_manager
                .get_bitboard_mut(Piece::new(Rook, self.active_color));
            let rook_swap_bb = Self::get_rook_swap_bb(moove.get_castle_type(), self.active_color);
            rook_bb ^= rook_swap_bb;
        }

        // If some piece was captured, put it back on the board.
        if let Some(captured_piece) = irreversible_data.captured_piece() {
            let piece = Piece::new(captured_piece, self.active_color.opposite());
            let bitboard = self.bb_manager.get_bitboard_mut(piece);

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
