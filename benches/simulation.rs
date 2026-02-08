use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use thanatos::conway;
use thanatos::types::cell_configuration::CellConfiguration;

fn bench_simulation(c: &mut Criterion) {
    let seed_cells = CellConfiguration::random_configuration(42, 50, 50, 0.5);
    let input = CellConfiguration::with_seed_configuration(seed_cells);

    c.bench_function("conway", |b| {
        b.iter(|| {
            conway::step(black_box(&input));
        })
    });

    c.bench_function("mfroc", |b| {
        b.iter(|| {
            thanatos::mfroc::process_mfroc(black_box(&input));
        })
    });
}

criterion_group!(benches, bench_simulation);
criterion_main!(benches);
