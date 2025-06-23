use criterion::{criterion_group, criterion_main, Criterion};
use uom::si::f64::*;
use uom::si::length::light_year;

fn path(c: &mut Criterion) {
    let universe = eftb::data::Universe::load().unwrap();
    // ~1200LY across dense space, takes ~0.05s on my laptop
    let start = &universe.star_map[&universe.star_name_to_id["EP7-F5V"]];
    let end = &universe.star_map[&universe.star_name_to_id["I.CKD.J6H"]];
    let jump_distance: Length = Length::new::<light_year>(200.0);
    c.bench_function("calc_path", |b| {
        b.iter(|| {
            eftb::calc_path(
                &universe,
                start,
                end,
                jump_distance,
                eftb::calc::path::PathOptimize::Fuel,
                false,
                Some(10),
            )
        })
    });
}

fn heuristic(c: &mut Criterion) {
    let universe = eftb::data::Universe::load().unwrap();
    let start = &universe.star_map[&universe.star_name_to_id["U75-4J4"]];
    let end = &universe.star_map[&universe.star_name_to_id["AJH-6H5"]];

    c.bench_function(
        format!("heuristic ({} connections)", start.connections.len()).as_str(),
        |b| {
            b.iter(|| {
                start
                    .connections
                    .iter()
                    .map(|conn| eftb::calc::path::heuristic(&universe, conn, end))
                    .collect::<Vec<_>>()
            })
        },
    );
}

fn successors(c: &mut Criterion) {
    let universe = eftb::data::Universe::load().unwrap();
    let start = &universe.star_map[&universe.star_name_to_id["U75-4J4"]];
    let conn = &eftb::data::Connection {
        id: 0,
        conn_type: eftb::data::ConnType::NpcGate,
        distance: Length::new::<light_year>(100.0),
        target: start.id,
    };
    let jump_distance = Length::new::<light_year>(100.0);

    for opt in [
        eftb::calc::path::PathOptimize::Distance,
        eftb::calc::path::PathOptimize::Fuel,
        eftb::calc::path::PathOptimize::Hops,
    ] {
        c.bench_function(format!("successors ({:?})", opt).as_str(), |b| {
            b.iter(|| {
                eftb::calc::path::successors(&universe, conn, jump_distance, opt, false);
            })
        });
    }
}

criterion_group!(benches, path, heuristic, successors);
criterion_main!(benches);
