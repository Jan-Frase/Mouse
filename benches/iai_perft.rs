use iai::black_box;
use mouse::State;
use mouse::backend::perft::perft;
use perft_fixtures::perft_fixtures::{NORMAL_PERFT, PerftFixture};

pub fn iai_perft(perft_fixture: &PerftFixture) {
    let fen_string = perft_fixture.perft_setup.fen;
    let _ = perft_fixture.perft_setup.name.to_owned()
        + ", depth: "
        + perft_fixture.depth.to_string().as_str();
    let depth = perft_fixture.depth;
    let _ = perft_fixture.expected_nodes;

    let mut state = State::new_from_fen(fen_string);

    perft(black_box(&mut state), black_box(depth));
}

pub fn starting_perft() {
    iai_perft(&NORMAL_PERFT[0]);
}

iai::main!(starting_perft);
