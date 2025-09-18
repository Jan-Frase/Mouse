use crate::backend::game_state::GameState;

mod backend;
mod constants;

fn main() {
    let game_state = GameState::new();
    let nodes = perft(4);
}

fn perft(depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;

    nodes
}
