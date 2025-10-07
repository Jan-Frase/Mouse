pub mod backend;

pub use backend::movegen::moove::Moove;
pub use backend::movegen::move_gen::get_pseudo_legal_moves;
pub use backend::state::board::bitboard;
pub use backend::state::game::game_state::GameState;
pub use backend::state::piece;
pub use backend::state::square;

// Everything below is for testing and benchmarking purposes
#[cfg(any(test, feature = "bench"))]
pub mod perft_fixture;

#[cfg(all(not(test), not(feature = "bench")))]
mod perft_fixture;
