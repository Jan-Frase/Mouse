use crate::backend::movegen::check_decider::is_in_check;
use crate::backend::movegen::move_gen::get_pseudo_legal_moves;
use crate::backend::state::game::state::State;

pub fn perft(state: &State, depth: u8) -> u64 {
    // PERFT: I would love to make this work, but it does not atm.
    // let moves = get_moves(game_state);
    // if depth == 1 {
    //     return moves.len() as u64;
    // }
    if depth == 0 {
        return 1;
    }

    let moves = get_pseudo_legal_moves(state);
    let mut nodes = 0;
    for chess_move in moves {
        let next_state = state.make_move(chess_move);
        // If we are in check after making the move -> skip.
        if is_in_check(&next_state, next_state.active_color().opposite()) {
            continue;
        }

        nodes += perft(&next_state, depth - 1);
    }

    nodes
}
