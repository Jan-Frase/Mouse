use crate::backend::compile_time::gen_caches_non_sliders::{
    gen_pawn_captures, gen_potential_moves_cache,
};
use crate::backend::compile_time::gen_caches_sliders::{
    PEXT_TABLE_SIZE, PextData, gen_cache_sliders,
};
use crate::backend::constants::{SIDES, SQUARES_AMOUNT};
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::piece::Piece;

/// All of this gets generated at compile time, in the functions below.
/// At runtime, we only have to read the values.
/// It contains for each square, a bitboard with every square that the king could potentially move to set to 1.
/// Example:
/// This: `KING_MOVES[Square::new(0,0)]` returns at bitboard that looks like this:
/// `_ _ _ _ _ _ _ _`
/// `_ _ _ _ _ _ _ _`
/// `_ _ _ _ _ _ _ _`
/// `_ _ _ _ _ _ _ _`
/// `_ _ _ _ _ _ _ _`
/// `_ _ _ _ _ _ _ _`
/// `X X _ _ _ _ _ _`
/// `_ X _ _ _ _ _ _`
/// At runtime we have to apply some further checks to this bitboard:
/// 1. Are some of these squares blocked by friendly pieces?
/// 2. Would this move put me in check etc...?
pub const KING_MOVES: [BitBoard; SQUARES_AMOUNT] = gen_potential_moves_cache(Piece::King);
pub const KNIGHT_MOVES: [BitBoard; SQUARES_AMOUNT] = gen_potential_moves_cache(Piece::Knight);

// Due for removal, only exists until check_decider gets reworked.
pub const PAWN_CAPTURE_MOVES: [[BitBoard; SQUARES_AMOUNT]; SIDES] = gen_pawn_captures();

const PEXT_DATA: PextData = gen_cache_sliders();

// Various slider caches:
pub const ROOK_PEXT_MASK: [BitBoard; SQUARES_AMOUNT] = PEXT_DATA.rook_pext_mask;
pub const ROOK_PEXT_INDEX: [usize; SQUARES_AMOUNT] = PEXT_DATA.rook_pext_index;

pub const BISHOP_PEXT_MASK: [BitBoard; SQUARES_AMOUNT] = PEXT_DATA.bishop_pext_mask;
pub const BISHOP_PEXT_INDEX: [usize; SQUARES_AMOUNT] = PEXT_DATA.bishop_pext_index;

pub static PEXT_TABLE: [BitBoard; PEXT_TABLE_SIZE] = PEXT_DATA.pext_table;
