use crate::backend::compile_time::gen_caches_non_sliders::{
    PawnMoveType, calculate_potential_moves_cache, generate_pawn_moves,
};
use crate::backend::compile_time::gen_caches_sliders::{PEXT_TABLE_SIZE, gen_cache_sliders};
use crate::backend::compile_time::generated::caches::{
    CACHE_BISHOP_PEXT_INDEX, CACHE_BISHOP_PEXT_MASK, CACHE_CAPTURE_PAWN, CACHE_KING, CACHE_KNIGHT,
    CACHE_PEXT_TABLE, CACHE_ROOK_PEXT_INDEX, CACHE_ROOK_PEXT_MASK,
};
use crate::backend::constants::{SIDES, SQUARES_AMOUNT};
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::piece::Piece;
use std::fs;

// ----------------------------------------
// READING
// ----------------------------------------

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
pub const KING_MOVES: [BitBoard; SQUARES_AMOUNT] = read_bb_cache(&CACHE_KING);
pub const KNIGHT_MOVES: [BitBoard; SQUARES_AMOUNT] = read_bb_cache(&CACHE_KNIGHT);

// Due for removal, only exists until check_decider gets reworked.
pub const PAWN_CAPTURE_MOVES: [[BitBoard; SQUARES_AMOUNT]; SIDES] =
    read_2d_bb_cache(CACHE_CAPTURE_PAWN);

// Various slider caches:
pub const ROOK_PEXT_MASK: [BitBoard; SQUARES_AMOUNT] = read_bb_cache(&CACHE_ROOK_PEXT_MASK);
pub const ROOK_PEXT_INDEX: [usize; SQUARES_AMOUNT] = CACHE_ROOK_PEXT_INDEX;

pub const BISHOP_PEXT_MASK: [BitBoard; SQUARES_AMOUNT] = read_bb_cache(&CACHE_BISHOP_PEXT_MASK);
pub const BISHOP_PEXT_INDEX: [usize; SQUARES_AMOUNT] = CACHE_BISHOP_PEXT_INDEX;

pub static PEXT_TABLE: [BitBoard; PEXT_TABLE_SIZE] = read_bb_cache(&CACHE_PEXT_TABLE);

pub const fn read_bb_cache<const N: usize>(cache: &[u64; N]) -> [BitBoard; N] {
    let mut potential_moves = [BitBoard::new(); N];

    let mut square_index: usize = 0;
    while square_index < N {
        let value = cache[square_index];
        let bb = BitBoard { value };
        potential_moves[square_index] = bb;

        square_index += 1;
    }

    potential_moves
}

pub const fn read_2d_bb_cache(
    cache_2d: [[u64; SQUARES_AMOUNT]; SIDES],
) -> [[BitBoard; SQUARES_AMOUNT]; SIDES] {
    let mut potential_moves = [[BitBoard::new(); SQUARES_AMOUNT]; SIDES];

    let mut side_index: usize = 0;
    while side_index < SIDES {
        let cache = &cache_2d[side_index];

        potential_moves[side_index] = read_bb_cache(cache);

        side_index += 1;
    }

    potential_moves
}

// ----------------------------------------
// WRITING
// ----------------------------------------

const DIR_PATH: &str = "src/backend/compile_time/generated/";
pub fn write_caches() {
    let king_moves = calculate_potential_moves_cache(Piece::King);
    let king_moves = king_moves.map(|b| b.value);
    let knight_moves = calculate_potential_moves_cache(Piece::Knight);
    let knight_moves = knight_moves.map(|b| b.value);

    let capture_pawn_moves = generate_pawn_moves(PawnMoveType::Capture);
    let capture_pawn_moves = capture_pawn_moves.map(|a| a.map(|b| b.value));

    let pext_data = gen_cache_sliders();
    let rook_pext_mask = pext_data.rook_pext_mask;
    let rook_pext_mask = rook_pext_mask.map(|b| b.value);
    let rook_pext_index = pext_data.rook_pext_index;
    let bishop_pext_mask = pext_data.bishop_pext_mask;
    let bishop_pext_mask = bishop_pext_mask.map(|b| b.value);
    let bishop_pext_index = pext_data.bishop_pext_index;
    let pext_table = pext_data.pext_table;
    let pext_table = pext_table.map(|b| b.value);

    let cache_strings = [
        format!("pub const CACHE_KING: [u64; 64] = {:?};", king_moves),
        format!("pub const CACHE_KNIGHT: [u64; 64] = {:?};", knight_moves),
        format!(
            "pub const CACHE_CAPTURE_PAWN: [[u64; 64]; 2] = {:?};",
            capture_pawn_moves
        ),
        format!(
            "pub const CACHE_ROOK_PEXT_MASK: [u64; 64] = {:?};",
            rook_pext_mask
        ),
        format!(
            "pub const CACHE_ROOK_PEXT_INDEX: [usize; 64] = {:?};",
            rook_pext_index
        ),
        format!(
            "pub const CACHE_BISHOP_PEXT_MASK: [u64; 64] = {:?};",
            bishop_pext_mask
        ),
        format!(
            "pub const CACHE_BISHOP_PEXT_INDEX: [usize; 64] = {:?};",
            bishop_pext_index
        ),
        // TODO: const leads to inlining, static does not. Is this better in my case?
        format!(
            "pub static CACHE_PEXT_TABLE: [u64; 107648] = {:?};",
            pext_table
        ),
    ];

    let mut file = String::from("");
    for string in cache_strings {
        file.push_str("#[rustfmt::skip] \n");
        file.push_str(&string);
        file.push_str("\n\n");
    }

    fs::write(format!("{}caches.rs", DIR_PATH), file).unwrap();
}
