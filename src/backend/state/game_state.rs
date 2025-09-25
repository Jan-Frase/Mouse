use crate::backend::moove::Moove;
use crate::backend::piece::PieceColor::{Black, White};
use crate::backend::piece::PieceType::{King, Pawn};
use crate::backend::piece::{Piece, PieceColor};
use crate::backend::state::bitboard_manager::BitBoardManager;
use crate::backend::state::fen_parser::parse_fen;
use crate::backend::state::irreversible_data::IrreversibleData;
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

        // Get the type of moved piece.
        let moved_piece = self
            .bit_board_manager
            .get_piece_at_square(moove.from())
            .unwrap();

        // Usually the piece something was captured on (if something was captured at all) is the square we moved to...
        let mut capture_square = moove.to().clone();

        // ... unless this is an en passant capture ...
        let ep_square = self
            .irreversible_data_stack
            .last()
            .unwrap()
            .en_passant_square();
        if moved_piece.piece_type() == Pawn {
            match ep_square {
                None => {}
                Some(ep_square) => {
                    // ... in this case we need to update the square.
                    let en_passant_capture_square =
                        moove.get_en_passant_capture_square(self.active_color);
                    if moove.to() == ep_square {
                        capture_square = en_passant_capture_square;
                    }
                }
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
            let ep_square = moove.from().forward_by_one(self.active_color);

            irreversible_data.set_en_passant_square(Some(ep_square));
        }

        // Get the bitboard for the piece that was moved.
        let moved_piece_bitboard = self.bit_board_manager.get_bitboard_mut(moved_piece);

        // Clear the square that the piece was moved from.
        moved_piece_bitboard.clear_square(moove.from());

        // Fill the square it moved to.
        moved_piece_bitboard.fill_square(moove.to());

        // Take care of some basics.
        self.active_color = self.active_color.opposite();
        self.irreversible_data_stack.push(irreversible_data);

        if self
            .bit_board_manager
            .get_bitboard(Piece::new(King, White))
            .is_empty()
        {
            panic!("King is missing");
        }

        if self
            .bit_board_manager
            .get_bitboard(Piece::new(King, Black))
            .is_empty()
        {
            panic!("King is missing");
        }
    }

    /// Reverts the last move made, restoring the board state to what it was
    /// before the move.
    ///
    /// # Arguments
    /// * `chess_move` - A `Moove` struct representing the chess move that needs to be reverted.
    pub fn unmake_move(&mut self, chess_move: Moove) {
        // Flip whose turn it is.
        self.active_color = self.active_color.opposite();
        // Get the last irreversible data.
        let irreversible_data = self.irreversible_data_stack.pop().unwrap();

        // Get the bitboard for the piece that was moved.
        let moved_piece_bitboard = self
            .bit_board_manager
            .get_bitboard_for_piece_at_square_mut(chess_move.to())
            .unwrap();

        // Fill the square that the piece was moved from.
        moved_piece_bitboard.fill_square(chess_move.from());

        // Clear the square it moved to.
        moved_piece_bitboard.clear_square(chess_move.to());

        // If some piece was captured, put it back on the board.
        if let Some(captured_piece) = irreversible_data.captured_piece() {
            let piece = Piece::new(captured_piece, self.active_color.opposite());
            let bitboard = self.bit_board_manager.get_bitboard_mut(piece);

            let mut capture_square = chess_move.to();
            // en passant
            let sq = self
                .irreversible_data_stack
                .last()
                .unwrap()
                .en_passant_square();
            match sq {
                None => {}
                Some(s) => {
                    if s == chess_move.to() {
                        capture_square = s;
                    }
                }
            }

            bitboard.fill_square(capture_square);
        }

        if self
            .bit_board_manager
            .get_bitboard(Piece::new(King, White))
            .is_empty()
        {
            panic!("King is missing");
        }

        if self
            .bit_board_manager
            .get_bitboard(Piece::new(King, Black))
            .is_empty()
        {
            panic!("King is missing");
        }
    }
}
