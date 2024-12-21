use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;
use uom::si::f64::*;
use uom::si::length::light_year;

fn criterion_benchmark(c: &mut Criterion) {
    let star_map = eftb::data::get_star_map().unwrap();
    let start = star_map.get(&30020604).unwrap(); // I2S-G19
    let end = star_map.get(&30023494).unwrap(); // L.YZG.1RS

    for opt in [
        eftb::calc::path::PathOptimize::Distance,
        eftb::calc::path::PathOptimize::Fuel,
        eftb::calc::path::PathOptimize::Hops,
    ] {
        let mut group = c.benchmark_group(format!("path --optimize {:?}", opt).as_str());
        group.warm_up_time(std::time::Duration::from_secs(1));
        group.measurement_time(std::time::Duration::from_secs(1));
        group.sample_size(10);
        for jd in 0..3 {
            let jump_distance: Length = Length::new::<light_year>((jd * 50) as f64);
            group.bench_with_input(
                BenchmarkId::from_parameter(jd * 50),
                &jump_distance,
                |b, jump_distance| {
                    b.iter(|| {
                        eftb::calc_path(
                            &star_map,
                            start,
                            end,
                            black_box(*jump_distance),
                            eftb::calc::path::PathOptimize::Fuel,
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
