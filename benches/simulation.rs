use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use thanatos::conway;
use thanatos::types::CellConfiguration;

fn bench_simulation(c: &mut Criterion) {
    let soup = CellConfiguration::cook_soup(42, 50, 50, 0.5);
    let input = CellConfiguration::from_soup(soup);

    c.bench_function("conway", |b| {
        b.iter(|| {
            conway::step(black_box(&input));
        })
    });

    c.bench_function("mfrac", |b| {
        b.iter(|| {
            thanatos::mfrac::run_pipeline(black_box(&input));
        })
    });
}

criterion_group!(benches, bench_simulation);
criterion_main!(benches);
