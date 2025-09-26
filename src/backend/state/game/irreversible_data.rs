use crate::backend::state::piece::PieceType;
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
            white_long_castle_rights: true,
            white_short_castle_rights: true,
            black_long_castle_rights: true,
            black_short_castle_rights: true,
        }
    }
}
