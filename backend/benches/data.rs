use criterion::{criterion_group, criterion_main, Criterion};

fn star_by_name(c: &mut Criterion) {
    let universe = eftb::data::Universe::load().unwrap();
    c.bench_function("star_by_name", |b| {
        b.iter(|| {
            universe.star_by_name(&"E9R-5PC".to_string()).unwrap();
            universe.star_by_name(&"E1J-V83".to_string()).unwrap();
        })
    });
}

fn star_id_to_name(c: &mut Criterion) {
    let universe = eftb::data::Universe::load().unwrap();
    let star = universe.star_by_name(&"E9R-5PC".to_string()).unwrap();
    c.bench_function("star_id_to_name", |b| {
        b.iter(|| {
            universe.star_id_to_name.get(&star.id).unwrap();
        })
    });
}

criterion_group!(benches, star_by_name, star_id_to_name);
criterion_main!(benches);
