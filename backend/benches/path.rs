use criterion::{criterion_group, criterion_main, Criterion};
use eftb::units::Meters;

fn path(c: &mut Criterion) {
    let universe = eftb::data::Universe::load().unwrap();
    // ~1200LY across dense space, takes ~0.05s on my laptop
    let start = universe.star_by_name(&"E9R-5PC".to_string()).unwrap();
    let end = universe.star_by_name(&"E1J-V83".to_string()).unwrap();
    let jump_distance = Meters::from_light_years(200.0);
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
    let start = universe.star_by_name(&"U75-4J4".to_string()).unwrap();
    let end = universe.star_by_name(&"OVD-1SM".to_string()).unwrap();

    c.bench_function(
        format!(
            "heuristic ({} connections)",
            universe.connections[&start.id].len()
        )
        .as_str(),
        |b| {
            b.iter(|| {
                universe.connections[&start.id]
                    .iter()
                    .map(|conn| eftb::calc::path::heuristic(&universe, conn, end))
                    .collect::<Vec<_>>()
            })
        },
    );
}

fn successors(c: &mut Criterion) {
    let universe = eftb::data::Universe::load().unwrap();
    let start = universe.star_by_name(&"U75-4J4".to_string()).unwrap();
    let conn = &eftb::data::Connection {
        id: 0,
        conn_type: eftb::data::ConnType::NpcGate,
        distance: Meters::from_light_years(100.0),
        target: start.id,
    };
    let jump_distance = Meters::from_light_years(100.0);

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

criterion_group! {
    name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(1000);
    targets = path, heuristic, successors
}
//criterion_group!(benches, path, heuristic, successors);
criterion_main!(benches);
