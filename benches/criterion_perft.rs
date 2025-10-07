use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use mouse::GameState;
use mouse::backend::perft::perft;
use mouse::perft_fixture::FAST_PERFT;

pub fn criterion_perft(c: &mut Criterion) {
    for perft_set_up in FAST_PERFT {
        let fen_string = perft_set_up.perft_setup().fen();
        let name = perft_set_up.perft_setup().name().to_owned()
            + ", depth: "
            + perft_set_up.depth().to_string().as_str();
        let depth = perft_set_up.depth();
        let expected_nodes = perft_set_up.expected_nodes();

        let mut state = GameState::new_from_fen(fen_string);

        run_criterion_perft(c, name, depth, expected_nodes, &mut state);
    }
}

fn run_criterion_perft(
    c: &mut Criterion,
    name: String,
    depth: u8,
    expected_nodes: u64,
    mut state: &mut GameState,
) {
    let mut group = c.benchmark_group(&*name);
    // group.sample_size(10);
    // group.sampling_mode(Flat);
    // group.warm_up_time(std::time::Duration::from_millis(1000));
    group.throughput(Throughput::Elements(expected_nodes));
    group.bench_function(&*name, |b| {
        b.iter(|| {
            perft(
                std::hint::black_box(&mut state),
                std::hint::black_box(depth),
            )
        })
    });
    group.finish();
}

criterion_group!(benches, criterion_perft);
criterion_main!(benches);
