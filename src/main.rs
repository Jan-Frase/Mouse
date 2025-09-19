use crate::backend::game_state::GameState;
use crate::backend::move_gen::get_moves;
use crate::backend::piece::{Piece, PieceColor, PieceType};

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

    let nodes = perft(&mut game_state, 2);
    println!("Nodes: {}", nodes);
}

fn perft(game_state: &mut GameState, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    let moves = get_moves(&game_state);
    for chess_move in moves {
        game_state.make_move(chess_move);
        // TODO: check if the positions is legal else unmake

        nodes += perft(game_state, depth - 1);
        game_state.unmake_move(chess_move);
    }

    nodes
}
