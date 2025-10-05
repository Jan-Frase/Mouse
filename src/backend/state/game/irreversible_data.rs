use crate::backend::movegen::moove::CastleType;
use crate::backend::state::piece::{PieceColor, PieceType};
use crate::backend::state::square::Square;
use getset::{CloneGetters, Setters};

/// The `IrreversibleData` struct stores data that is irreversible.
/// For example, this remembers what kind of piece was captured for `unmake_move()`.
#[derive(CloneGetters, Setters, Debug)]
pub struct IrreversibleData {
    #[getset(get_clone = "pub", set = "pub")]
    half_move_clock: u8,
    #[getset(get_clone = "pub", set = "pub")]
    captured_piece: Option<PieceType>,
    #[getset(get_clone = "pub", set = "pub")]
    en_passant_square: Option<Square>,
    #[getset(get_clone = "pub", set = "pub")]
    white_long_castle_rights: bool,
    #[getset(get_clone = "pub", set = "pub")]
    white_short_castle_rights: bool,
    #[getset(get_clone = "pub", set = "pub")]
    black_long_castle_rights: bool,
    #[getset(get_clone = "pub", set = "pub")]
    black_short_castle_rights: bool,
}

impl IrreversibleData {
    pub fn new() -> IrreversibleData {
        IrreversibleData {
            half_move_clock: 0,
            captured_piece: None,
            en_passant_square: None,
            white_long_castle_rights: false,
            white_short_castle_rights: false,
            black_long_castle_rights: false,
            black_short_castle_rights: false,
        }
    }

    pub fn new_with_castling_true() -> IrreversibleData {
        IrreversibleData {
            half_move_clock: 0,
            captured_piece: None,
            en_passant_square: None,
            white_long_castle_rights: true,
            white_short_castle_rights: true,
            black_long_castle_rights: true,
            black_short_castle_rights: true,
        }
    }

    pub fn new_from_previous_state(previous_state: &IrreversibleData) -> IrreversibleData {
        IrreversibleData {
            half_move_clock: previous_state.half_move_clock + 1,
            captured_piece: None,
            en_passant_square: None,
            white_long_castle_rights: previous_state.white_long_castle_rights,
            white_short_castle_rights: previous_state.white_short_castle_rights,
            black_long_castle_rights: previous_state.black_long_castle_rights,
            black_short_castle_rights: previous_state.black_short_castle_rights,
        }
    }

    pub fn get_long_castle_rights(&self, color: PieceColor) -> bool {
        match color {
            PieceColor::White => self.white_long_castle_rights,
            PieceColor::Black => self.black_long_castle_rights,
        }
    }

    pub fn get_short_castle_rights(&self, color: PieceColor) -> bool {
        match color {
            PieceColor::White => self.white_short_castle_rights,
            PieceColor::Black => self.black_short_castle_rights,
        }
    }

    pub fn remove_long_castle_rights(&mut self, color: PieceColor) {
        match color {
            PieceColor::White => self.white_long_castle_rights = false,
            PieceColor::Black => self.black_long_castle_rights = false,
        }
    }

    pub fn remove_short_castle_rights(&mut self, color: PieceColor) {
        match color {
            PieceColor::White => self.white_short_castle_rights = false,
            PieceColor::Black => self.black_short_castle_rights = false,
        }
    }

    pub fn remove_castle_rights(&mut self, color: PieceColor, castle_type: CastleType) {
        match castle_type {
            CastleType::Long => self.remove_long_castle_rights(color),
            CastleType::Short => self.remove_short_castle_rights(color),
        }
    }
}
