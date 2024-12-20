use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use uom::si::f64::*;
use uom::si::length::light_year;

fn criterion_benchmark(c: &mut Criterion) {
    let star_map = eftb::data::get_star_map().unwrap();
    let start = star_map.get(&30017903).unwrap(); // E.G1G.6GD
    let end = star_map.get(&30008580).unwrap(); // Nod

    let jump_distance: Length = Length::new::<light_year>(120.0);
    c.bench_function("path 120", |b| {
        b.iter(|| {
            eftb::calc_path(
                &star_map,
                start,
                end,
                black_box(jump_distance),
                eftb::PathOptimize::Fuel,
            )
        })
    });

    let jump_distance: Length = Length::new::<light_year>(500.0);
    c.bench_function("path 500", |b| {
        b.iter(|| {
            eftb::calc_path(
                &star_map,
                start,
                end,
                black_box(jump_distance),
                eftb::PathOptimize::Fuel,
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
