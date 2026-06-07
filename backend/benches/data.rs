use criterion::{criterion_group, criterion_main, Criterion};
use eftb::units::Meters;

fn star_by_name(c: &mut Criterion) {
    let universe = eftb::data::Universe::build(Meters::from_light_years(500.0)).unwrap();
    c.bench_function("star_by_name", |b| {
        b.iter(|| {
            universe.star_by_name(&"E9R-5PC".to_string()).unwrap();
            universe.star_by_name(&"E1J-V83".to_string()).unwrap();
        })
    });
}

criterion_group!(benches, star_by_name);
criterion_main!(benches);
