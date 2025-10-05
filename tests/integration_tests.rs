use mouse::piece::Piece;
use mouse::piece::PieceColor::White;
use mouse::piece::PieceType::King;
use mouse::{GameState, get_pseudo_legal_moves};

#[test]
fn test_integration() {
    let state = GameState::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let moves = get_pseudo_legal_moves(&state);
    println!(
        "Moves: {:#?}",
        moves.iter().map(|m| m.to_string()).collect::<Vec<String>>()
    );
    assert_eq!(moves.len(), 20);

    let bb = state.bb_manager().get_bitboard(Piece::new(King, White));
    bb.get_all_true_squares();
}
