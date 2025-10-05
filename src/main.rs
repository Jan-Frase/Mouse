use crate::backend::perft::{perft, run_perftree_debug};
use crate::backend::state::game::game_state::GameState;
use std::env;
use std::time::Instant;

mod backend;
mod constants;

fn main() {
    let args = env::args();
    run_perftree_debug(args);
    // run_nps_perft();
}

fn run_nps_perft() {
    let mut game_state =
        GameState::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    // Start timer to calculate nodes per second.
    let now = Instant::now();

    // let nodes = root_debug_perft(&mut game_state, 9);
    let nodes = perft(&mut game_state, 6);

    let elapsed = now.elapsed();
    println!("Nodes searched: {:?}", nodes);
    let nodes_per_second = nodes as f64 / elapsed.as_secs_f64();
    println!("with {:?} nodes per second,", nodes_per_second); // 577620 nps in dev - 8.676.006 nps in release - 25.104.754 nps
    println!("took {:?}.", elapsed);
}
