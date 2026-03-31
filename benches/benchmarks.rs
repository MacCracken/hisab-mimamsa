use criterion::{Criterion, black_box, criterion_group, criterion_main};

use hisab_mimamsa::relativity::black_hole;
use hisab_mimamsa::relativity::geodesic;
use hisab_mimamsa::relativity::lorentz;
use hisab_mimamsa::relativity::metric;

const M_SUN: f64 = 1.989e30;

fn bench_lorentz_factor(c: &mut Criterion) {
    c.bench_function("lorentz_factor", |b| {
        b.iter(|| lorentz::lorentz_factor(black_box(0.866 * lorentz::C)))
    });
}

fn bench_schwarzschild_radius(c: &mut Criterion) {
    c.bench_function("schwarzschild_radius", |b| {
        b.iter(|| metric::schwarzschild_radius(black_box(M_SUN)))
    });
}

fn bench_hawking_temperature(c: &mut Criterion) {
    c.bench_function("hawking_temperature", |b| {
        b.iter(|| black_hole::hawking_temperature(black_box(M_SUN)))
    });
}

fn bench_light_deflection(c: &mut Criterion) {
    c.bench_function("light_deflection_weak_field", |b| {
        b.iter(|| geodesic::light_deflection_weak_field(black_box(M_SUN), black_box(6.957e8)))
    });
}

fn bench_four_vector_boost(c: &mut Criterion) {
    let event = lorentz::FourVector::new(3.0 * lorentz::C, 2.0 * lorentz::C, 0.0, 0.0);
    c.bench_function("four_vector_boost_x", |b| {
        b.iter(|| black_box(event).boost_x(black_box(0.5 * lorentz::C)))
    });
}

criterion_group!(
    benches,
    bench_lorentz_factor,
    bench_schwarzschild_radius,
    bench_hawking_temperature,
    bench_light_deflection,
    bench_four_vector_boost,
);
criterion_main!(benches);
