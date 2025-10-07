use mouse::GameState;
use mouse::backend::perft::perft;
use mouse::perft_fixture::{FAST_PERFT, NORMAL_PERFT, PerftFixture};

#[test]
fn test_perft_fast() {
    test_perft(&FAST_PERFT);
}

#[test]
fn test_perft_normal() {
    test_perft(&NORMAL_PERFT);
}

fn test_perft(perft_fixtures: &[PerftFixture]) {
    for perft_fixture in perft_fixtures {
        let fen = perft_fixture.perft_setup().fen();
        let depth = perft_fixture.depth();
        let expected_nodes = perft_fixture.expected_nodes();
        let name = perft_fixture.perft_setup().name().to_owned();

        let mut state = GameState::new_from_fen(fen);
        let nodes = perft(&mut state, depth);

        assert_eq!(nodes, expected_nodes, "Testing {:?}", name);
    }
}
