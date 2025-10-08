use crate::backend::movegen::check_decider::is_in_check;
use crate::backend::movegen::move_gen::get_pseudo_legal_moves;
use crate::backend::state::game::game_state::GameState;

pub fn perft(game_state: &mut GameState, depth: u8) -> u64 {
    // PERFT: I would love to make this work, but it does not atm.
    // let moves = get_moves(game_state);
    // if depth == 1 {
    //     return moves.len() as u64;
    // }
    if depth == 0 {
        return 1;
    }

    let moves = get_pseudo_legal_moves(game_state);
    let mut nodes = 0;
    for chess_move in moves {
        game_state.make_move(chess_move);
        // If we are in check after making the move -> skip.
        if is_in_check(game_state, game_state.active_color().opposite()) {
            game_state.unmake_move(chess_move);
            continue;
        }

        nodes += perft(game_state, depth - 1);
        game_state.unmake_move(chess_move);
    }

    nodes
}
