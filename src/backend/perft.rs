use crate::backend::movegen::check_decider::is_in_check;
use crate::backend::movegen::moove::Moove;
use crate::backend::movegen::move_gen::get_pseudo_legal_moves;
use crate::backend::state::game::game_state::GameState;
use std::env::Args;

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
    let mut game_state = GameState::new_parse_fen(fen);

    for mooves in input {
        // Code golfing
        mooves
            .split_whitespace()
            .map(|moove| Moove::new_from_uci_notation(moove))
            .for_each(|moove| {
                game_state.make_move(moove);
            });
    }

    root_debug_perft(&mut game_state, depth as u8);
}

fn root_debug_perft(game_state: &mut GameState, depth: u8) -> u64 {
    // Total nodes searched.
    let mut nodes = 0;

    // Generate all root moves.
    let mut moves = get_pseudo_legal_moves(game_state);
    // Sort them in the same way as perftree does
    moves.sort();
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

// let mut game_state = GameState::new_parse_fen("1n2k3/8/8/8/8/8/8/1N2K3 w - - 0 1");
//
//  Start timer to calculate nodes per second.
// let now = Instant::now();
//
// let nodes = root_debug_perft(&mut game_state, 9);
// let nodes = perft(&mut game_state, 9);
//
// let elapsed = now.elapsed();
// println!("Nodes searched: {:?}", nodes);
// let nodes_per_second = nodes as f64 / elapsed.as_secs_f64();
// println!("with {:?} nodes per second,", nodes_per_second); // 577620 nps in dev - 8.676.006 nps in release - 25.104.754 nps
// println!("took {:?}.", elapsed);
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

    #[test]
    fn test_perft_02() {
        let mut game_state =
            GameState::new_parse_fen("4k3/pppppppp/8/8/8/8/PPPPPPPP/4K3 w - - 0 1");
        let nodes = perft(&mut game_state, 4);
        assert_eq!(nodes, 98766);
    }

    #[test]
    fn test_perft_03() {
        let mut game_state = GameState::new_parse_fen("4k3/ppp5/7p/8/8/8/PPP5/4K3 w - - 0 1");

        let nodes = perft(&mut game_state, 4);
        assert_eq!(nodes, 17684);
    }

    #[test]
    fn test_perft_04() {
        let mut game_state = GameState::new_parse_fen("4k3/ppp5/7p/8/8/8/PPP5/4K3 w - - 0 1");

        let nodes = perft(&mut game_state, 5);
        assert_eq!(nodes, 197056);
    }

    #[test]
    fn test_perft_05() {
        let mut game_state = GameState::new_parse_fen("7k/3p4/8/2P5/8/8/8/7K b - - 0 1");

        let nodes = root_debug_perft(&mut game_state, 4);
        assert_eq!(nodes, 896);

        let nodes = root_debug_perft(&mut game_state, 5);
        assert_eq!(nodes, 6583);
    }

    #[test]
    fn test_perft_06() {
        let mut game_state = GameState::new_parse_fen("7k/8/8/8/8/2K5/2P5/8 w - - 0 1");

        let nodes = root_debug_perft(&mut game_state, 1);
        assert_eq!(nodes, 7);
    }

    #[test]
    fn test_perft_07() {
        let mut game_state = GameState::new_parse_fen("8/3P1k2/8/8/8/8/8/7K b - - 0 1");

        let nodes = root_debug_perft(&mut game_state, 1);
        assert_eq!(nodes, 7);

        let nodes = root_debug_perft(&mut game_state, 2);
        assert_eq!(nodes, 49);

        // Missing slider logic atm
        // let nodes = root_debug_perft(&mut game_state, 3);
        // assert_eq!(nodes, 289);
    }

    #[test]
    fn test_perft_08() {
        let mut game_state = GameState::new_parse_fen("8/1ppP1k2/1n6/3P2P1/8/8/8/7K b - - 0 1");

        let nodes = root_debug_perft(&mut game_state, 2);
        assert_eq!(nodes, 117);
    }

    #[test]
    fn test_perft_09() {
        let mut game_state = GameState::new_parse_fen("7k/P7/8/8/8/8/8/7K w - - 0 1");

        let nodes = root_debug_perft(&mut game_state, 1);
        assert_eq!(nodes, 7);
    }
}
