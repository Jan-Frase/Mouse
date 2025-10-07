use criterion::SamplingMode::Flat;
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use mouse::GameState;
use mouse::perft_fixture::FAST_PERFT;

pub fn criterion_perft(c: &mut Criterion) {
    for perft in FAST_PERFT {
        let fen_string = perft.perft_setup().fen();
        let name = perft.perft_setup().name().to_owned()
            + ", depth: "
            + perft.depth().to_string().as_str();
        let depth = perft.depth();
        let expected_nodes = perft.expected_nodes();

        let mut state = GameState::new_from_fen(fen_string);

        let mut group = c.benchmark_group(&*name);
        // group.sample_size(10);
        group.sampling_mode(Flat);
        group.warm_up_time(std::time::Duration::from_millis(1000));
        group.throughput(Throughput::Elements(expected_nodes));
        group.bench_function(&*name, |b| {
            b.iter(|| {
                mouse::backend::perft::perft(
                    std::hint::black_box(&mut state),
                    std::hint::black_box(depth),
                )
            })
        });
        group.finish();
    }
}

criterion_group!(benches, criterion_perft);
criterion_main!(benches);
