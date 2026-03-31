//! Adversarial input tests — every public function hit with hostile values.
//!
//! NaN, ±Inf, ±0.0, negative masses, extreme magnitudes, n=0 integration.
//! A function must either return a sensible Result::Err or a finite, non-NaN value.
//! Panics and silent NaN propagation are both failures.

use hisab_mimamsa::relativity::{black_hole, geodesic, lensing, lorentz, metric};

#[cfg(feature = "cosmology")]
use hisab_mimamsa::cosmology::{expansion, friedmann};

const HOSTILE: [f64; 10] = [
    f64::NAN,
    f64::INFINITY,
    f64::NEG_INFINITY,
    0.0,
    -0.0,
    f64::MIN,
    f64::MAX,
    f64::EPSILON,
    -1.0,
    1e-300,
];

// ── helpers ──────────────────────────────────────────────────────────────

/// A Result-returning function must not panic. If Ok, value must be finite.
fn assert_result_sound(r: Result<f64, hisab_mimamsa::MimamsaError>, ctx: &str) {
    match r {
        Ok(v) => assert!(v.is_finite(), "{ctx}: returned non-finite Ok({v})"),
        Err(_) => {} // error is always acceptable
    }
}

// ── lorentz ──────────────────────────────────────────────────────────────

#[test]
fn fuzz_lorentz_factor() {
    for &v in &HOSTILE {
        let r = lorentz::lorentz_factor(v);
        assert_result_sound(r, &format!("lorentz_factor({v})"));
    }
}

#[test]
fn fuzz_time_dilation() {
    for &t in &HOSTILE {
        for &v in &HOSTILE {
            let r = lorentz::time_dilation(t, v);
            assert_result_sound(r, &format!("time_dilation({t}, {v})"));
        }
    }
}

#[test]
fn fuzz_length_contraction() {
    for &l in &HOSTILE {
        for &v in &HOSTILE {
            let r = lorentz::length_contraction(l, v);
            assert_result_sound(r, &format!("length_contraction({l}, {v})"));
        }
    }
}

#[test]
fn fuzz_kinetic_energy() {
    for &m in &HOSTILE {
        for &v in &HOSTILE {
            let r = lorentz::kinetic_energy(m, v);
            assert_result_sound(r, &format!("kinetic_energy({m}, {v})"));
        }
    }
}

#[test]
fn fuzz_total_energy() {
    for &m in &HOSTILE {
        for &v in &HOSTILE {
            let r = lorentz::total_energy(m, v);
            assert_result_sound(r, &format!("total_energy({m}, {v})"));
        }
    }
}

#[test]
fn fuzz_relativistic_momentum() {
    for &m in &HOSTILE {
        for &v in &HOSTILE {
            let r = lorentz::relativistic_momentum(m, v);
            assert_result_sound(r, &format!("relativistic_momentum({m}, {v})"));
        }
    }
}

#[test]
fn fuzz_velocity_addition() {
    for &u in &HOSTILE {
        for &v in &HOSTILE {
            let r = lorentz::velocity_addition(u, v);
            assert_result_sound(r, &format!("velocity_addition({u}, {v})"));
        }
    }
}

#[test]
fn fuzz_doppler_factor() {
    for &v in &HOSTILE {
        let r = lorentz::doppler_factor(v);
        assert_result_sound(r, &format!("doppler_factor({v})"));
    }
}

#[test]
fn fuzz_four_vector_boost() {
    let fv = lorentz::FourVector::new(1.0, 1.0, 0.0, 0.0).unwrap();
    for &v in &HOSTILE {
        let r = fv.boost_x(v);
        assert_result_sound(r.map(|b| b.invariant_interval()), &format!("boost_x({v})"));
    }
    // Also fuzz the four-vector components themselves
    for &x in &HOSTILE {
        let r = lorentz::FourVector::new(x, x, x, x);
        assert_result_sound(
            r.map(|fv2| fv2.invariant_interval()),
            &format!("FourVector::new/invariant_interval with {x}"),
        );
    }
}

#[test]
fn fuzz_beta() {
    for &v in &HOSTILE {
        let b = lorentz::beta(v);
        assert_result_sound(b, &format!("beta({v})"));
    }
}

#[test]
fn fuzz_rest_energy() {
    for &m in &HOSTILE {
        let e = lorentz::rest_energy(m);
        assert_result_sound(e, &format!("rest_energy({m})"));
    }
}

// ── metric ───────────────────────────────────────────────────────────────

#[test]
fn fuzz_schwarzschild_radius() {
    for &m in &HOSTILE {
        let r = metric::schwarzschild_radius(m);
        assert_result_sound(r, &format!("schwarzschild_radius({m})"));
    }
}

#[test]
fn fuzz_gravitational_time_dilation() {
    for &m in &HOSTILE {
        for &r in &HOSTILE {
            let res = metric::gravitational_time_dilation(m, r);
            assert_result_sound(res, &format!("gravitational_time_dilation({m}, {r})"));
        }
    }
}

#[test]
fn fuzz_gravitational_redshift() {
    for &m in &HOSTILE {
        for &r in &HOSTILE {
            let res = metric::gravitational_redshift(m, r);
            assert_result_sound(res, &format!("gravitational_redshift({m}, {r})"));
        }
    }
}

#[test]
fn fuzz_schwarzschild_orbital_velocity() {
    for &m in &HOSTILE {
        for &r in &HOSTILE {
            let res = metric::schwarzschild_orbital_velocity(m, r);
            assert_result_sound(res, &format!("schwarzschild_orbital_velocity({m}, {r})"));
        }
    }
}

// ── geodesic ─────────────────────────────────────────────────────────────

#[test]
fn fuzz_light_deflection() {
    for &m in &HOSTILE {
        for &b in &HOSTILE {
            let d = geodesic::light_deflection_weak_field(m, b);
            assert_result_sound(d, &format!("light_deflection_weak_field({m}, {b})"));
        }
    }
}

#[test]
fn fuzz_shapiro_delay() {
    for &m in &HOSTILE {
        let d = geodesic::shapiro_delay(m, 1e11, 1e11, 7e8);
        assert_result_sound(d, &format!("shapiro_delay({m}, ...)"));
    }
    // Also fuzz impact parameter (log of zero/negative risk)
    for &b in &HOSTILE {
        let d = geodesic::shapiro_delay(1.989e30, 1e11, 1e11, b);
        assert_result_sound(d, &format!("shapiro_delay(..., b={b})"));
    }
    // Fuzz r1, r2 (also in the log)
    for &r in &HOSTILE {
        let d = geodesic::shapiro_delay(1.989e30, r, 1e11, 7e8);
        assert_result_sound(d, &format!("shapiro_delay(..., r1={r})"));
    }
}

#[test]
fn fuzz_effective_potential() {
    for &r in &HOSTILE {
        for &l in &HOSTILE {
            let v = geodesic::schwarzschild_effective_potential(
                2953.0,
                r,
                l,
                geodesic::GeodesicType::Timelike,
            );
            assert_result_sound(v, &format!("effective_potential(r={r}, L={l})"));
        }
    }
}

// ── black_hole ───────────────────────────────────────────────────────────

#[test]
fn fuzz_hawking_temperature() {
    for &m in &HOSTILE {
        let t = black_hole::hawking_temperature(m);
        assert_result_sound(t, &format!("hawking_temperature({m})"));
    }
}

#[test]
fn fuzz_bekenstein_hawking_entropy() {
    for &m in &HOSTILE {
        let s = black_hole::bekenstein_hawking_entropy(m);
        assert_result_sound(s, &format!("bekenstein_hawking_entropy({m})"));
    }
}

#[test]
fn fuzz_evaporation_time() {
    for &m in &HOSTILE {
        let t = black_hole::evaporation_time(m);
        assert_result_sound(t, &format!("evaporation_time({m})"));
    }
}

#[test]
fn fuzz_surface_gravity() {
    for &m in &HOSTILE {
        let g = black_hole::surface_gravity(m);
        assert_result_sound(g, &format!("surface_gravity({m})"));
    }
}

#[test]
fn fuzz_black_hole_properties() {
    for &m in &HOSTILE {
        let r = black_hole::BlackHoleProperties::from_mass(m);
        match r {
            Ok(p) => {
                assert!(p.radius_m.is_finite(), "BlackHoleProperties({m}).radius_m non-finite");
                assert!(p.temperature_k.is_finite(), "BlackHoleProperties({m}).temperature_k non-finite");
                assert!(p.entropy_j_per_k.is_finite(), "BlackHoleProperties({m}).entropy_j_per_k non-finite");
                assert!(p.evaporation_time_s.is_finite(), "BlackHoleProperties({m}).evaporation_time_s non-finite");
                assert!(p.surface_gravity_m_s2.is_finite(), "BlackHoleProperties({m}).surface_gravity_m_s2 non-finite");
            }
            Err(_) => {} // error is always acceptable
        }
    }
}

// ── lensing ──────────────────────────────────────────────────────────────

#[test]
fn fuzz_einstein_ring() {
    for &m in &HOSTILE {
        let r = lensing::einstein_ring_radius(m, 1e22, 2e22);
        assert_result_sound(r, &format!("einstein_ring_radius(m={m})"));
    }
    for &d in &HOSTILE {
        let r = lensing::einstein_ring_radius(1.989e30, d, 2e22);
        assert_result_sound(r, &format!("einstein_ring_radius(d_lens={d})"));
        let r = lensing::einstein_ring_radius(1.989e30, 1e22, d);
        assert_result_sound(r, &format!("einstein_ring_radius(d_source={d})"));
    }
}

#[test]
fn fuzz_point_lens_magnification() {
    for &u in &HOSTILE {
        let mu = lensing::point_lens_magnification(u);
        match mu {
            // u ≈ 0 → infinite magnification is physically correct (point source)
            Ok(v) if v.is_infinite() && u.abs() < 1e-15 => {}
            other => assert_result_sound(other, &format!("point_lens_magnification({u})")),
        }
    }
}

#[test]
fn fuzz_critical_surface_density() {
    for &d in &HOSTILE {
        let r = lensing::critical_surface_density(d, 2e22);
        assert_result_sound(r, &format!("critical_surface_density(d_lens={d})"));
        let r = lensing::critical_surface_density(1e22, d);
        assert_result_sound(r, &format!("critical_surface_density(d_source={d})"));
    }
}

// ── cosmology ────────────────────────────────────────────────────────────

#[cfg(feature = "cosmology")]
mod cosmology_fuzz {
    use super::*;

    fn params() -> friedmann::CosmologicalParameters {
        friedmann::CosmologicalParameters::planck2018()
    }

    #[test]
    fn fuzz_hubble_parameter() {
        for &z in &HOSTILE {
            let h = friedmann::hubble_parameter(&params(), z);
            assert_result_sound(h, &format!("hubble_parameter(z={z})"));
        }
    }

    #[test]
    fn fuzz_critical_density() {
        for &h in &HOSTILE {
            let rho = friedmann::critical_density(h);
            assert_result_sound(rho, &format!("critical_density(h={h})"));
        }
    }

    #[test]
    fn fuzz_deceleration_parameter() {
        let q = friedmann::deceleration_parameter_now(&params());
        assert_result_sound(q, "deceleration_parameter_now");
    }

    #[test]
    fn fuzz_age_of_universe() {
        // n=0 must not panic or infinite-loop
        let a = friedmann::age_of_universe(&params(), 1100.0, 0);
        assert_result_sound(a, "age_of_universe(n=0)");

        // n=1
        let a = friedmann::age_of_universe(&params(), 1100.0, 1);
        assert_result_sound(a, "age_of_universe(n=1)");

        // negative z_max
        let a = friedmann::age_of_universe(&params(), -10.0, 100);
        assert_result_sound(a, "age_of_universe(z_max=-10)");

        // z_max = 0
        let a = friedmann::age_of_universe(&params(), 0.0, 100);
        match a {
            Ok(v) => assert!((v - 0.0).abs() < f64::EPSILON, "age_of_universe(z_max=0) should be 0"),
            Err(_) => {} // error is acceptable
        }

        // hostile z_max
        for &z in &HOSTILE {
            let a = friedmann::age_of_universe(&params(), z, 10);
            assert_result_sound(a, &format!("age_of_universe(z_max={z}, n=10)"));
        }
    }

    #[test]
    fn fuzz_comoving_distance() {
        for &z in &HOSTILE {
            let d = expansion::comoving_distance(&params(), z, 10);
            assert_result_sound(d, &format!("comoving_distance(z={z})"));
        }
        // n=0
        let d = expansion::comoving_distance(&params(), 1.0, 0);
        assert_result_sound(d, "comoving_distance(n=0)");
    }

    #[test]
    fn fuzz_luminosity_distance() {
        for &z in &HOSTILE {
            let d = expansion::luminosity_distance(&params(), z, 10);
            assert_result_sound(d, &format!("luminosity_distance(z={z})"));
        }
    }

    #[test]
    fn fuzz_angular_diameter_distance() {
        for &z in &HOSTILE {
            let d = expansion::angular_diameter_distance(&params(), z, 10);
            assert_result_sound(d, &format!("angular_diameter_distance(z={z})"));
        }
    }

    #[test]
    fn fuzz_lookback_time() {
        for &z in &HOSTILE {
            let t = expansion::lookback_time(&params(), z, 10);
            assert_result_sound(t, &format!("lookback_time(z={z})"));
        }
        // n=0
        let t = expansion::lookback_time(&params(), 1.0, 0);
        assert_result_sound(t, "lookback_time(n=0)");
    }

    #[test]
    fn fuzz_hubble_distance() {
        // hostile h0 values
        for &h0 in &HOSTILE {
            let mut p = params();
            p.h0 = h0;
            let d = expansion::hubble_distance(&p);
            assert_result_sound(d, &format!("hubble_distance(h0={h0})"));
        }
    }

    #[test]
    fn fuzz_scale_factor() {
        for &z in &HOSTILE {
            let a = expansion::scale_factor(z);
            assert_result_sound(a, &format!("scale_factor(z={z})"));
        }
    }

    #[test]
    fn fuzz_redshift_from_scale_factor() {
        for &a in &HOSTILE {
            let z = expansion::redshift_from_scale_factor(a);
            assert_result_sound(z, &format!("redshift_from_scale_factor(a={a})"));
        }
    }

    #[test]
    fn fuzz_cmb_temperature() {
        for &z in &HOSTILE {
            let t = expansion::cmb_temperature(z);
            assert_result_sound(t, &format!("cmb_temperature(z={z})"));
        }
    }
}
