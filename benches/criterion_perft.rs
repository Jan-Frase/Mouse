use criterion::{Criterion, criterion_group, criterion_main};
use mouse::GameState;
use mouse::backend::perft::perft;

pub fn criterion_perft(c: &mut Criterion) {
    let mut state =
        GameState::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    c.bench_function("starting pos, depth 2", |b| {
        b.iter(|| perft(&mut state, std::hint::black_box(2)))
    });
}

criterion_group!(benches, criterion_perft);
criterion_main!(benches);
