//! Physical invariant tests — properties that must hold under composition.
//!
//! These verify the physics, not just the plumbing.

use hisab_mimamsa::relativity::{black_hole, lorentz, metric};

#[cfg(feature = "cosmology")]
use hisab_mimamsa::cosmology::{expansion, friedmann};

const M_SUN: f64 = 1.989e30;

// ── Lorentz invariance ──────────────────────────────────────────────────

#[test]
fn boost_then_inverse_preserves_four_vector() {
    let event = lorentz::FourVector::new(5.0, 3.0, 1.0, 0.0).unwrap();
    let v = 0.6 * lorentz::C;

    let boosted = event.boost_x(v).unwrap();
    let restored = boosted.boost_x(-v).unwrap();

    assert!((event.ct - restored.ct).abs() / event.ct.abs() < 1e-10);
    assert!((event.x - restored.x).abs() / event.x.abs() < 1e-10);
    assert!((event.y - restored.y).abs() < 1e-15);
    assert!((event.z - restored.z).abs() < 1e-15);
}

#[test]
fn boost_preserves_interval_at_multiple_velocities() {
    let event = lorentz::FourVector::new(10.0, 4.0, 3.0, 0.0).unwrap();
    let s2_orig = event.invariant_interval();

    for beta in [0.1, 0.3, 0.5, 0.7, 0.9, 0.99] {
        let v = beta * lorentz::C;
        let boosted = event.boost_x(v).unwrap();
        let s2 = boosted.invariant_interval();
        assert!(
            (s2 - s2_orig).abs() / s2_orig.abs() < 1e-10,
            "interval not preserved at beta={beta}: {s2} vs {s2_orig}"
        );
    }
}

#[test]
fn velocity_addition_never_exceeds_c() {
    for u_frac in [0.1, 0.5, 0.9, 0.99, 0.999] {
        for v_frac in [0.1, 0.5, 0.9, 0.99, 0.999] {
            let result = lorentz::velocity_addition(
                u_frac * lorentz::C,
                v_frac * lorentz::C,
            )
            .unwrap();
            assert!(
                result < lorentz::C,
                "v_add({u_frac}c, {v_frac}c) = {result} >= c"
            );
        }
    }
}

#[test]
fn velocity_addition_with_c_returns_c() {
    // Adding c to anything should give c
    let result = lorentz::velocity_addition(lorentz::C, 0.5 * lorentz::C).unwrap();
    assert!((result - lorentz::C).abs() / lorentz::C < 1e-12);
}

// ── Black hole thermodynamics ───────────────────────────────────────────

#[test]
fn temperature_mass_inverse_law() {
    // T ∝ 1/M: doubling mass halves temperature
    for factor in [2.0, 5.0, 10.0, 100.0] {
        let t1 = black_hole::hawking_temperature(M_SUN).unwrap();
        let t2 = black_hole::hawking_temperature(factor * M_SUN).unwrap();
        let ratio = t1 / t2;
        assert!(
            (ratio - factor).abs() / factor < 1e-10,
            "T(M)/T({factor}M) = {ratio}, expected {factor}"
        );
    }
}

#[test]
fn entropy_mass_squared_law() {
    // S ∝ M²: doubling mass quadruples entropy
    for factor in [2.0, 3.0, 10.0] {
        let s1 = black_hole::bekenstein_hawking_entropy(M_SUN).unwrap();
        let s2 = black_hole::bekenstein_hawking_entropy(factor * M_SUN).unwrap();
        let ratio = s2 / s1;
        let expected = factor * factor;
        assert!(
            (ratio - expected).abs() / expected < 1e-6,
            "S({factor}M)/S(M) = {ratio}, expected {expected}"
        );
    }
}

#[test]
fn evaporation_time_mass_cubed_law() {
    // t_evap ∝ M³
    let t1 = black_hole::evaporation_time(M_SUN).unwrap();
    let t2 = black_hole::evaporation_time(2.0 * M_SUN).unwrap();
    assert!((t2 / t1 - 8.0).abs() < 0.01);
}

#[test]
fn entropy_monotonically_increases_with_mass() {
    let masses: Vec<f64> = (1..=10).map(|i| i as f64 * M_SUN).collect();
    let entropies: Vec<f64> = masses
        .iter()
        .map(|&m| black_hole::bekenstein_hawking_entropy(m).unwrap())
        .collect();
    for w in entropies.windows(2) {
        assert!(w[1] > w[0], "entropy not monotonically increasing");
    }
}

// ── GR consistency ──────────────────────────────────────────────────────

#[test]
fn schwarzschild_hierarchy() {
    // For any mass: r_s < r_photon < r_isco
    for mass_factor in [0.1, 1.0, 10.0, 1e6] {
        let m = mass_factor * M_SUN;
        let rs = metric::schwarzschild_radius(m).unwrap();
        let rph = metric::photon_sphere_radius(m).unwrap();
        let risco = metric::schwarzschild_isco(m).unwrap();
        assert!(rs < rph, "r_s >= r_photon at {mass_factor} M_sun");
        assert!(rph < risco, "r_photon >= r_isco at {mass_factor} M_sun");
    }
}

#[test]
fn schwarzschild_radius_scales_linearly() {
    let rs1 = metric::schwarzschild_radius(M_SUN).unwrap();
    let rs2 = metric::schwarzschild_radius(2.0 * M_SUN).unwrap();
    assert!((rs2 / rs1 - 2.0).abs() < 1e-10);
}

#[test]
fn time_dilation_approaches_unity_at_infinity() {
    let factor = metric::gravitational_time_dilation(M_SUN, 1e20).unwrap();
    assert!((factor - 1.0).abs() < 1e-10);
}

// ── Cosmology ───────────────────────────────────────────────────────────

#[cfg(feature = "cosmology")]
mod cosmology_invariants {
    use super::*;

    fn params() -> friedmann::CosmologicalParameters {
        friedmann::CosmologicalParameters::planck2018()
    }

    #[test]
    fn distance_ordering_for_positive_z() {
        // For z > 0: angular_diameter < comoving < luminosity
        let p = params();
        for z in [0.5, 1.0, 2.0, 5.0] {
            let d_a = expansion::angular_diameter_distance(&p, z, 1000).unwrap();
            let d_c = expansion::comoving_distance(&p, z, 1000).unwrap();
            let d_l = expansion::luminosity_distance(&p, z, 1000).unwrap();
            assert!(
                d_a < d_c,
                "d_A >= d_C at z={z}: {d_a} >= {d_c}"
            );
            assert!(
                d_c < d_l,
                "d_C >= d_L at z={z}: {d_c} >= {d_l}"
            );
        }
    }

    #[test]
    fn etherington_reciprocity() {
        // d_L = (1+z)² d_A (Etherington's reciprocity theorem)
        let p = params();
        for z in [0.5, 1.0, 2.0, 3.0] {
            let d_l = expansion::luminosity_distance(&p, z, 1000).unwrap();
            let d_a = expansion::angular_diameter_distance(&p, z, 1000).unwrap();
            let ratio = d_l / d_a;
            let expected = (1.0 + z) * (1.0 + z);
            assert!(
                (ratio - expected).abs() / expected < 1e-6,
                "Etherington violated at z={z}: d_L/d_A = {ratio}, expected {expected}"
            );
        }
    }

    #[test]
    fn hubble_parameter_monotonically_increases_with_z() {
        let p = params();
        let zs = [0.0, 0.5, 1.0, 2.0, 5.0, 10.0, 100.0];
        let hs: Vec<f64> = zs
            .iter()
            .map(|&z| friedmann::hubble_parameter(&p, z).unwrap())
            .collect();
        for w in hs.windows(2) {
            assert!(w[1] > w[0], "H(z) not monotonically increasing");
        }
    }

    #[test]
    fn scale_factor_redshift_roundtrip() {
        for z in [0.0, 0.5, 1.0, 5.0, 100.0, 1100.0] {
            let a = expansion::scale_factor(z).unwrap();
            let z2 = expansion::redshift_from_scale_factor(a).unwrap();
            assert!((z - z2).abs() < 1e-10, "roundtrip failed at z={z}");
        }
    }

    #[test]
    fn lookback_time_bounded_by_age() {
        let p = params();
        let age = friedmann::age_of_universe(&p, 1100.0, 10000).unwrap();
        for z in [0.5, 1.0, 2.0, 5.0] {
            let t = expansion::lookback_time(&p, z, 1000).unwrap();
            assert!(t < age, "lookback time at z={z} exceeds age of universe");
            assert!(t > 0.0, "lookback time at z={z} is not positive");
        }
    }

    #[test]
    fn comoving_distance_monotonically_increases_with_z() {
        let p = params();
        let zs = [0.1, 0.5, 1.0, 2.0, 5.0, 10.0];
        let ds: Vec<f64> = zs
            .iter()
            .map(|&z| expansion::comoving_distance(&p, z, 1000).unwrap())
            .collect();
        for w in ds.windows(2) {
            assert!(w[1] > w[0], "comoving distance not monotonically increasing");
        }
    }
}
