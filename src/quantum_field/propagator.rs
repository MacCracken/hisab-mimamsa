//! Free-field propagators in momentum and position space.
//!
//! Provides the Feynman propagator for scalar (Klein-Gordon), fermion (Dirac),
//! and gauge boson fields. All momentum-space propagators are algebraic expressions
//! in `hisab::Complex`.

use hisab::Complex;
use tracing::warn;

use crate::error::{ensure_finite_complex, require_finite, MimamsaError};

use super::FourMomentum;

/// Default Feynman iε regulator (small positive number).
pub const DEFAULT_EPSILON: f64 = 1e-10;

/// Scalar (Klein-Gordon) propagator in momentum space.
///
/// Δ_F(p) = i / (p² - m² + iε)
///
/// # Arguments
/// * `p` — Four-momentum.
/// * `mass_gev` — Particle mass in GeV (natural units).
/// * `epsilon` — Feynman iε regulator (small positive, e.g. [`DEFAULT_EPSILON`]).
#[inline]
pub fn scalar_propagator(
    p: &FourMomentum,
    mass_gev: f64,
    epsilon: f64,
) -> Result<Complex, MimamsaError> {
    require_finite(mass_gev, "scalar_propagator")?;
    require_finite(epsilon, "scalar_propagator")?;
    if epsilon <= 0.0 {
        warn!(epsilon, "scalar_propagator: epsilon must be positive");
        return Err(MimamsaError::Computation(
            "scalar_propagator: epsilon must be positive".to_string(),
        ));
    }
    let p2 = p.invariant_mass_sq()?;
    let denom = Complex::new(p2 - mass_gev * mass_gev, epsilon);
    let i = Complex::new(0.0, 1.0);
    ensure_finite_complex(i / denom, "scalar_propagator")
}

/// Fermion (Dirac) propagator scalar factor in momentum space.
///
/// Returns the scalar denominator factor i / (p² - m² + iε).
/// The full spinor structure S_F(p) = i(p̸ + m) / (p² - m² + iε) requires
/// gamma matrices; this returns the scalar piece. Multiply by (p̸ + m) in
/// spinor space for the complete propagator.
#[inline]
pub fn fermion_propagator_scalar(
    p: &FourMomentum,
    mass_gev: f64,
    epsilon: f64,
) -> Result<Complex, MimamsaError> {
    // Scalar structure is identical to Klein-Gordon
    scalar_propagator(p, mass_gev, epsilon)
}

/// Gauge boson (photon) propagator in Feynman gauge.
///
/// D_F^μν(k) = -i g^μν / (k² + iε)
///
/// Returns the scalar factor -i / (k² + iε); the metric tensor g^μν is implicit.
/// Massless (m = 0).
#[inline]
pub fn gauge_boson_propagator(
    k: &FourMomentum,
    epsilon: f64,
) -> Result<Complex, MimamsaError> {
    require_finite(epsilon, "gauge_boson_propagator")?;
    if epsilon <= 0.0 {
        warn!(epsilon, "gauge_boson_propagator: epsilon must be positive");
        return Err(MimamsaError::Computation(
            "gauge_boson_propagator: epsilon must be positive".to_string(),
        ));
    }
    let k2 = k.invariant_mass_sq()?;
    let denom = Complex::new(k2, epsilon);
    let neg_i = Complex::new(0.0, -1.0);
    ensure_finite_complex(neg_i / denom, "gauge_boson_propagator")
}

/// Scalar propagator in position space via numerical FFT.
///
/// Computes the 1D Fourier transform of the momentum-space propagator
/// along the energy axis (at zero spatial separation):
///
/// Δ_F(t) = ∫ dE/(2π) · e^{-iEt} · i/(E² - m² + iε)
///
/// # Arguments
/// * `mass_gev` — Particle mass in GeV.
/// * `time_separation` — Temporal separation in natural units (1/GeV).
/// * `n_points` — FFT resolution (must be a power of 2).
/// * `e_max` — Energy cutoff for the integration grid (GeV).
pub fn scalar_propagator_position_space(
    mass_gev: f64,
    time_separation: f64,
    n_points: usize,
    e_max: f64,
) -> Result<Complex, MimamsaError> {
    require_finite(mass_gev, "scalar_propagator_position_space")?;
    require_finite(time_separation, "scalar_propagator_position_space")?;
    require_finite(e_max, "scalar_propagator_position_space")?;

    if n_points == 0 || !n_points.is_power_of_two() {
        return Err(MimamsaError::Computation(
            "scalar_propagator_position_space: n_points must be a positive power of 2".to_string(),
        ));
    }
    if e_max <= 0.0 {
        return Err(MimamsaError::Computation(
            "scalar_propagator_position_space: e_max must be positive".to_string(),
        ));
    }

    let de = 2.0 * e_max / n_points as f64;
    let n = n_points;

    // Build the propagator on a momentum grid and multiply by phase e^{-iEt}
    let mut data: Vec<Complex> = Vec::with_capacity(n);
    for k in 0..n {
        let e = -e_max + (k as f64 + 0.5) * de;
        let denom = Complex::new(e * e - mass_gev * mass_gev, DEFAULT_EPSILON);
        let prop = Complex::new(0.0, 1.0) / denom;
        // Phase factor for the specific time separation
        let phase = Complex::from_polar(1.0, -e * time_separation);
        data.push(prop * phase * de);
    }

    // FFT gives us the Fourier integral
    hisab::num::fft(&mut data).map_err(|e| {
        MimamsaError::Computation(format!("scalar_propagator_position_space: FFT failed: {e}"))
    })?;

    // The result is in the DC bin (index 0), normalized by 1/(2π)
    let result = data[0] * (1.0 / (2.0 * std::f64::consts::PI));
    ensure_finite_complex(result, "scalar_propagator_position_space")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::M_ELECTRON_GEV;

    #[test]
    fn test_scalar_propagator_on_shell_large() {
        // On-shell: p² ≈ m² → denominator ≈ iε → large magnitude
        let m = M_ELECTRON_GEV;
        let p = FourMomentum::new(m, 0.0, 0.0, 0.0).unwrap();
        let prop = scalar_propagator(&p, m, DEFAULT_EPSILON).unwrap();
        // |Δ| ≈ 1/ε = 1e10
        assert!(prop.abs() > 1e8);
    }

    #[test]
    fn test_scalar_propagator_off_shell_small() {
        // Far off-shell: p² >> m² → small propagator
        let m = M_ELECTRON_GEV;
        let p = FourMomentum::new(100.0, 0.0, 0.0, 0.0).unwrap();
        let prop = scalar_propagator(&p, m, DEFAULT_EPSILON).unwrap();
        // |Δ| ≈ 1/p² ≈ 1e-4
        assert!(prop.abs() < 0.01);
    }

    #[test]
    fn test_gauge_boson_propagator_massless() {
        // Photon propagator: -i/(k² + iε)
        let k = FourMomentum::new(10.0, 5.0, 0.0, 0.0).unwrap();
        let prop = gauge_boson_propagator(&k, DEFAULT_EPSILON).unwrap();
        // k² = 100 - 25 = 75, |D| ≈ 1/75
        assert!((prop.abs() - 1.0 / 75.0).abs() < 1e-4);
    }

    #[test]
    fn test_propagator_symmetry() {
        // Δ(p) = Δ(-p) for scalar field
        let m = 1.0;
        let p = FourMomentum::new(5.0, 3.0, 1.0, 2.0).unwrap();
        let neg_p = FourMomentum::new(-5.0, -3.0, -1.0, -2.0).unwrap();
        let d1 = scalar_propagator(&p, m, DEFAULT_EPSILON).unwrap();
        let d2 = scalar_propagator(&neg_p, m, DEFAULT_EPSILON).unwrap();
        assert!((d1.re - d2.re).abs() < 1e-12);
        assert!((d1.im - d2.im).abs() < 1e-12);
    }

    #[test]
    fn test_propagator_nan_rejected() {
        let p = FourMomentum::new(1.0, 0.0, 0.0, 0.0).unwrap();
        assert!(scalar_propagator(&p, f64::NAN, DEFAULT_EPSILON).is_err());
    }

    #[test]
    fn test_epsilon_zero_rejected() {
        let p = FourMomentum::new(1.0, 0.0, 0.0, 0.0).unwrap();
        assert!(scalar_propagator(&p, 0.5, 0.0).is_err());
        assert!(scalar_propagator(&p, 0.5, -1e-10).is_err());
    }

    #[test]
    fn test_gauge_epsilon_zero_rejected() {
        let k = FourMomentum::new(10.0, 5.0, 0.0, 0.0).unwrap();
        assert!(gauge_boson_propagator(&k, 0.0).is_err());
        assert!(gauge_boson_propagator(&k, -1.0).is_err());
    }

    #[test]
    fn test_position_space_propagator_runs() {
        let result = scalar_propagator_position_space(1.0, 0.1, 256, 50.0);
        assert!(result.is_ok());
        let val = result.unwrap();
        assert!(val.re.is_finite());
        assert!(val.im.is_finite());
    }

    #[test]
    fn test_position_space_bad_n_rejected() {
        assert!(scalar_propagator_position_space(1.0, 0.1, 100, 50.0).is_err()); // not power of 2
        assert!(scalar_propagator_position_space(1.0, 0.1, 0, 50.0).is_err());
    }
}
