use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

use hisab_mimamsa::relativity::black_hole;
use hisab_mimamsa::relativity::geodesic;
use hisab_mimamsa::relativity::lorentz;
use hisab_mimamsa::relativity::metric;

const M_SUN: f64 = 1.989e30;

// ── Relativity ───────────────────────────────────────────────────────────

fn bench_lorentz_factor(c: &mut Criterion) {
    c.bench_function("lorentz_factor", |b| {
        b.iter(|| lorentz::lorentz_factor(black_box(0.866 * lorentz::C)))
    });
}

fn bench_schwarzschild_radius(c: &mut Criterion) {
    c.bench_function("schwarzschild_radius", |b| {
        b.iter(|| metric::schwarzschild_radius(black_box(M_SUN)).unwrap())
    });
}

fn bench_hawking_temperature(c: &mut Criterion) {
    c.bench_function("hawking_temperature", |b| {
        b.iter(|| black_hole::hawking_temperature(black_box(M_SUN)).unwrap())
    });
}

fn bench_light_deflection(c: &mut Criterion) {
    c.bench_function("light_deflection_weak_field", |b| {
        b.iter(|| {
            geodesic::light_deflection_weak_field(black_box(M_SUN), black_box(6.957e8)).unwrap()
        })
    });
}

fn bench_four_vector_boost(c: &mut Criterion) {
    let event = lorentz::FourVector::new(3.0 * lorentz::C, 2.0 * lorentz::C, 0.0, 0.0).unwrap();
    c.bench_function("four_vector_boost_x", |b| {
        b.iter(|| black_box(event).boost_x(black_box(0.5 * lorentz::C)))
    });
}

// ── QFT ──────────────────────────────────────────────────────────────────

#[cfg(feature = "qft")]
mod qft_benches {
    use super::*;
    use hisab_mimamsa::constants::{ALPHA, ALPHA_S_MZ, M_Z_GEV};
    use hisab_mimamsa::quantum_field::{FourMomentum, coupling, propagator, vacuum};

    pub fn bench_scalar_propagator(c: &mut Criterion) {
        let p = FourMomentum::new(100.0, 50.0, 30.0, 10.0).unwrap();
        c.bench_function("scalar_propagator", |b| {
            b.iter(|| {
                propagator::scalar_propagator(
                    black_box(&p),
                    black_box(0.511e-3),
                    propagator::DEFAULT_EPSILON,
                )
                .unwrap()
            })
        });
    }

    pub fn bench_casimir_force(c: &mut Criterion) {
        c.bench_function("casimir_force_per_area", |b| {
            b.iter(|| vacuum::casimir_force_per_area(black_box(1e-6)).unwrap())
        });
    }

    pub fn bench_running_coupling_qed(c: &mut Criterion) {
        c.bench_function("running_coupling_qed_analytic", |b| {
            b.iter(|| {
                coupling::running_coupling_qed_analytic(
                    black_box(ALPHA),
                    black_box(M_Z_GEV),
                    black_box(1000.0),
                )
                .unwrap()
            })
        });
    }

    pub fn bench_running_coupling_qcd(c: &mut Criterion) {
        c.bench_function("running_coupling_qcd_analytic", |b| {
            b.iter(|| {
                coupling::running_coupling_qcd_analytic(
                    black_box(ALPHA_S_MZ),
                    black_box(M_Z_GEV),
                    black_box(1000.0),
                    black_box(6),
                )
                .unwrap()
            })
        });
    }
}

// ── Unified ──────────────────────────────────────────────────────────────

#[cfg(feature = "unified")]
mod unified_benches {
    use super::*;
    use hisab_mimamsa::cosmology::friedmann::CosmologicalParameters;
    use hisab_mimamsa::unified::{fixed_point, holographic, scale_bridge};

    pub fn bench_holographic_bound(c: &mut Criterion) {
        let rs = metric::schwarzschild_radius(M_SUN).unwrap();
        let area = 4.0 * std::f64::consts::PI * rs * rs;
        c.bench_function("holographic_bound", |b| {
            b.iter(|| holographic::holographic_bound(black_box(area)).unwrap())
        });
    }

    pub fn bench_entropy_ratio(c: &mut Criterion) {
        let params = CosmologicalParameters::planck2018();
        c.bench_function("entropy_ratio", |b| {
            b.iter(|| fixed_point::entropy_ratio(black_box(&params), black_box(0.0)).unwrap())
        });
    }

    pub fn bench_manifestation_intensity(c: &mut Criterion) {
        let params = CosmologicalParameters::planck2018();
        c.bench_function("manifestation_intensity", |b| {
            b.iter(|| {
                fixed_point::manifestation_intensity(black_box(&params), black_box(0.0)).unwrap()
            })
        });
    }

    pub fn bench_bridge_output(c: &mut Criterion) {
        let params = CosmologicalParameters::planck2018();
        c.bench_function("BridgeOutput::at_redshift", |b| {
            b.iter(|| {
                scale_bridge::BridgeOutput::at_redshift(black_box(&params), black_box(0.0)).unwrap()
            })
        });
    }
}

// ── Groups ───────────────────────────────────────────────────────────────

criterion_group!(
    relativity_benches,
    bench_lorentz_factor,
    bench_schwarzschild_radius,
    bench_hawking_temperature,
    bench_light_deflection,
    bench_four_vector_boost,
);

#[cfg(feature = "qft")]
criterion_group!(
    qft_bench_group,
    qft_benches::bench_scalar_propagator,
    qft_benches::bench_casimir_force,
    qft_benches::bench_running_coupling_qed,
    qft_benches::bench_running_coupling_qcd,
);

#[cfg(feature = "unified")]
criterion_group!(
    unified_bench_group,
    unified_benches::bench_holographic_bound,
    unified_benches::bench_entropy_ratio,
    unified_benches::bench_manifestation_intensity,
    unified_benches::bench_bridge_output,
);

#[cfg(all(feature = "qft", feature = "unified"))]
criterion_main!(relativity_benches, qft_bench_group, unified_bench_group);

#[cfg(all(feature = "qft", not(feature = "unified")))]
criterion_main!(relativity_benches, qft_bench_group);

#[cfg(not(feature = "qft"))]
criterion_main!(relativity_benches);
