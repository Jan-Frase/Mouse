use crate::backend::compile_time::gen_caches::write_caches;
use crate::backend::movegen::check_decider::is_in_check;
use crate::backend::movegen::moove::Moove;
use crate::backend::movegen::move_gen::get_pseudo_legal_moves;
use crate::backend::perft::perft;
use crate::backend::state::game::state::State;
use std::env::Args;

mod backend;

fn main() {
    write_caches();
    // let args = env::args();
    // run_perftree_debug(args);
}

// --------------------------------------------- //
// PERFTREE DEBUGGING
// https://github.com/agausmann/perftree
// --------------------------------------------- //

pub fn run_perftree_debug(mut input: Args) {
    // Remove the first useless input.
    input.next();

    let depth = input.next().unwrap();
    let depth = depth.parse::<i32>().unwrap();

    let fen = &input.next().unwrap();
    let mut game_state = State::new_from_fen(fen);

    for mooves in input {
        // Code golfing
        mooves
            .split_whitespace()
            .map(Moove::new_from_uci_notation)
            .for_each(|moove| {
                game_state = game_state.make_move(moove);
            });
    }

    root_debug_perft(&mut game_state, depth as u8);
}

pub fn root_debug_perft(root_state: &mut State, depth: u8) -> u64 {
    // Total nodes searched.
    let mut nodes = 0;

    // Generate all root moves.
    let mut moves = get_pseudo_legal_moves(root_state);
    // Sort them in the same way as perftree does
    moves.sort();
    for chess_move in moves {
        let state = root_state.make_move(chess_move);
        // If we are in check after making the move -> skip.
        if is_in_check(&state, state.active_color().opposite()) {
            // game_state.unmake_move(chess_move);
            continue;
        }

        // Recursively calculate nodes for this position.
        let nodes_for_this_position = perft(&state, depth - 1);
        nodes += nodes_for_this_position;
        // print info for https://github.com/agausmann/perftree
        println!("{} {:?}", chess_move, nodes_for_this_position);

        // game_state.unmake_move(chess_move);
    }

    println!();
    println!("{:?}", nodes);
    nodes
}
