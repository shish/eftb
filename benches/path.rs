use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use eftb::data::Star;
use std::collections::HashMap;
use std::hint::black_box;
use uom::si::f64::*;
use uom::si::length::light_year;

fn criterion_benchmark(c: &mut Criterion) {
    let universe = eftb::data::Universe::load().unwrap();

    //   | E    | N    | I    | L    |
    // --+------+------+------+------+
    // E |      |
    // N | 1500 |
    // D | 3000 |
    // I | 4300 |
    // L | 6100 |

    let stars: HashMap<String, Star> = [
        ("E.G1G.6GD", 30017903),
        ("Nod", 30008580),
        ("D.271.E9Q", 30020473),
        ("I2S.G19", 30020604),
        ("L.YZG.1RS", 30023494),
    ]
    .iter()
    .map(|(name, id)| (name.to_string(), universe.star_map.get(id).unwrap().clone()))
    .collect();

    for optimize in [
        eftb::calc::path::PathOptimize::Distance,
        eftb::calc::path::PathOptimize::Fuel,
        eftb::calc::path::PathOptimize::Hops,
    ] {
        let start = stars.get("E.G1G.6GD").unwrap();
        let end = stars.get("Nod").unwrap();
        let mut group = c.benchmark_group(format!("path --optimize {:?}", optimize).as_str());
        group.warm_up_time(std::time::Duration::from_secs(1));
        group.measurement_time(std::time::Duration::from_secs(1));
        group.sample_size(10);
        for jd in 0..6 {
            let jump_distance: Length = Length::new::<light_year>((jd * 100) as f64);
            group.bench_with_input(
                BenchmarkId::from_parameter(jd * 100),
                &jump_distance,
                |b, jump_distance| {
                    b.iter(|| {
                        eftb::calc_path(
                            &universe,
                            start,
                            end,
                            black_box(*jump_distance),
                            optimize,
                            false,
                            Some(10),
                        )
                    })
                },
            );
        }
        group.finish();
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
