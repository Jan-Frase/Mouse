use crate::backend::movegen::check_decider::is_in_check;
use crate::backend::movegen::move_gen::get_moves;
use crate::backend::piece::{Piece, PieceColor, PieceType};
use backend::state::game_state::GameState;
use std::time::Instant;

mod backend;
mod constants;

fn main() {
    let mut game_state = GameState::new_parse_fen("1n2k3/8/8/8/8/8/8/1N2K3 w - - 0 1");

    println!("{:?}", game_state);

    println!(
        "{}",
        game_state
            .bit_board_manager()
            .get_bitboard(Piece::new(PieceType::Knight, PieceColor::White))
    );
    println!(
        "{}",
        game_state
            .bit_board_manager()
            .get_bitboard(Piece::new(PieceType::Knight, PieceColor::Black))
    );
    println!(
        "{}",
        game_state
            .bit_board_manager()
            .get_bitboard(Piece::new(PieceType::King, PieceColor::White))
    );
    println!(
        "{}",
        game_state
            .bit_board_manager()
            .get_bitboard(Piece::new(PieceType::King, PieceColor::Black))
    );

    // Start timer to calculate nodes per second.
    let now = Instant::now();

    let nodes = root_debug_perft(&mut game_state, 8);
    // let nodes = perft(&mut game_state, 1);

    let elapsed = now.elapsed();
    println!("Nodes searched: {:?}", nodes);
    let nodes_per_second = nodes as f64 / elapsed.as_secs_f64();
    println!("with {:?} nodes per second,", nodes_per_second); // 577620 nps in dev - 8676006 nps in release
    println!("took {:?}.", elapsed);
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

fn perft(game_state: &mut GameState, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    let moves = get_moves(game_state);
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
