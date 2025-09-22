use crate::backend::perft::perft;
use backend::state::game_state::GameState;
use std::time::Instant;

mod backend;
mod constants;

fn main() {
    let mut game_state = GameState::new_parse_fen("1n2k3/8/8/8/8/8/8/1N2K3 w - - 0 1");

    // Start timer to calculate nodes per second.
    let now = Instant::now();

    // let nodes = root_debug_perft(&mut game_state, 9);
    let nodes = perft(&mut game_state, 9);

    let elapsed = now.elapsed();
    println!("Nodes searched: {:?}", nodes);
    let nodes_per_second = nodes as f64 / elapsed.as_secs_f64();
    println!("with {:?} nodes per second,", nodes_per_second); // 577620 nps in dev - 8.676.006 nps in release - 25.104.754 nps
    println!("took {:?}.", elapsed);
}
