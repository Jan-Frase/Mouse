use mouse::{GameState, get_pseudo_legal_moves};

#[test]
fn test_starting_pos() {
    let state = GameState::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let moves = get_pseudo_legal_moves(&state);
    println!(
        "Moves: {:#?}",
        moves.iter().map(|m| m.to_string()).collect::<Vec<String>>()
    );
    assert_eq!(moves.len(), 20);
}

#[test]
fn test_starting_pos_perft() {
    let state = GameState::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
}
