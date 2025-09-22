use crate::backend::movegen::check_decider::is_in_check;
use crate::backend::movegen::move_gen::get_moves;
use crate::backend::state::game_state::GameState;

pub fn perft(game_state: &mut GameState, depth: u8) -> u64 {
    // PERFT: I would love to make this work, but it does not atm.
    // let moves = get_moves(game_state);
    // if depth == 1 {
    //     return moves.len() as u64;
    // }
    if depth == 0 {
        return 1;
    }

    let moves = get_moves(game_state);
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

fn root_debug_perft(game_state: &mut GameState, depth: u8) -> u64 {
    // Total nodes searched.
    let mut nodes = 0;

    // Generate all root moves.
    let moves = get_moves(game_state);
    for chess_move in moves {
        game_state.make_move(chess_move);
        // If we are in check after making the move -> skip.
        if is_in_check(game_state, game_state.active_color().opposite()) {
            game_state.unmake_move(chess_move);
            continue;
        }

        // Recursively calculate nodes for this position.
        let nodes_for_this_position = perft(game_state, depth - 1);
        nodes += nodes_for_this_position;
        // print info for https://github.com/agausmann/perftree
        println!("{} {:?}", chess_move, nodes_for_this_position);

        game_state.unmake_move(chess_move);
    }

    println!();
    println!("{:?}", nodes);
    nodes
}
// --------------------------------------------- //
// TESTING
// --------------------------------------------- //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::perft::perft;

    #[test]
    fn test_perft_01() {
        let mut game_state =
            GameState::new_parse_fen("8/1n2k3/5n1n/2n5/4N3/2N5/1N2KN2/8 w - - 0 1");
        let nodes = perft(&mut game_state, 4);
        assert_eq!(nodes, 472915);

        // This currently takes too long to run.
        // let nodes = perft(&mut game_state, 5);
        // assert_eq!(nodes, 11949411);
    }
}
