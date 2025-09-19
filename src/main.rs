use crate::backend::move_gen::check_decider::is_in_check;
use crate::backend::move_gen::move_gen::get_moves;
use crate::backend::piece::{Piece, PieceColor, PieceType};
use backend::state::game_state::GameState;
use std::time::Instant;

mod backend;
mod constants;

fn main() {
    let mut game_state = GameState::new();

    let white_king_bitboard = game_state
        .bit_board_manager_mut()
        .get_bitboard_mut(Piece::new(PieceType::King, PieceColor::White));
    white_king_bitboard.fill_square(backend::square::Square::new(4, 0));

    let black_king_bitboard = game_state
        .bit_board_manager_mut()
        .get_bitboard_mut(Piece::new(PieceType::King, PieceColor::Black));
    black_king_bitboard.fill_square(backend::square::Square::new(4, 7));

    // root_debug_perft(&mut game_state, 9);

    // Start timer to calculate nodes per second.
    let now = Instant::now();
    let nodes = perft(&mut game_state, 10);
    let elapsed = now.elapsed();
    println!("{:?}", nodes);
    let nodes_per_second = nodes as f64 / elapsed.as_secs_f64();
    println!("{:?}", nodes_per_second); // 577620 nps in dev - 8676006 nps in release
    println!("{:?}", elapsed);
}

fn root_debug_perft(game_state: &mut GameState, depth: u8) {
    // Total nodes searched.
    let mut nodes = 0;

    // Generate all root moves.
    let moves = get_moves(&game_state);
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
}

fn perft(game_state: &mut GameState, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    let moves = get_moves(&game_state);
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
