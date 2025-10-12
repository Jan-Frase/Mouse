use crate::backend::constants::SQUARES_AMOUNT;
use crate::backend::state::board::bitboard::BitBoard;
use crate::backend::state::square::Square;
use std::fs;

const DIR_PATH: &str = "src/backend/compile_time/generated/";

pub fn gen_cache_king() {
    let cache = calculate_potential_moves_cache();

    let cache_str = format!("pub const CACHE_KING: [u64; 64] = {:?};", cache);
    fs::write(DIR_PATH.to_owned() + "cache_king.rs", cache_str)
        .expect("Unable to write cache_king.rs");
}

fn calculate_potential_moves_cache() -> [u64; SQUARES_AMOUNT] {
    let mut potential_moves = [BitBoard::new(); SQUARES_AMOUNT];

    // iterate over all squares
    let mut square_index: usize = 0;
    while square_index < SQUARES_AMOUNT {
        // generate a square struct from the index
        let square = Square::index_to_square(square_index as i8);

        // and generate the moves for that square
        potential_moves[square_index] = generate_king_moves(square);

        square_index += 1;
    }

    let potential_moves = potential_moves.map(|b| b.value);

    potential_moves
}

fn generate_king_moves(square: Square) -> BitBoard {
    let mut bitboard = BitBoard::new();

    // since this fn is const, we can't use a loop
    // instead we use a while loop
    // to iterate over the surrounding files
    let mut file_offset = -1;
    while file_offset <= 1 {
        // and ranks
        let mut rank_offset = -1;
        while rank_offset <= 1 {
            // skip the current square
            if file_offset == 0 && rank_offset == 0 {
                rank_offset += 1;
                continue;
            }
            // create the relevant square
            let current_square =
                Square::new(square.file() + file_offset, square.rank() + rank_offset);

            // and add it if it's valid
            if current_square.is_valid() {
                bitboard.fill_square(current_square);
            }

            rank_offset += 1;
        }

        file_offset += 1;
    }
    bitboard
}
