//! Vacuum energy — zero-point fluctuations, Casimir effect, regularization.
//!
//! Provides both SI-unit calculations (Casimir effect between plates) and
//! natural-unit calculations (regularized vacuum energy density).

use std::f64::consts::PI;

use tracing::{instrument, warn};

use crate::constants::{C, HBAR};
use crate::error::{MimamsaError, ensure_finite, require_finite};

/// Zero-point energy of a single field mode with angular frequency ω.
///
/// E₀ = ½ℏω (SI units, Joules).
#[inline]
pub fn zero_point_energy(omega: f64) -> Result<f64, MimamsaError> {
    require_finite(omega, "zero_point_energy")?;
    if omega < 0.0 {
        warn!(omega, "zero_point_energy: omega must be non-negative");
        return Err(MimamsaError::Computation(
            "zero_point_energy: omega must be non-negative".to_string(),
        ));
    }
    ensure_finite(0.5 * HBAR * omega, "zero_point_energy")
}

/// Sum of zero-point energies for a scalar field in a 1D box of size L.
///
/// E = Σ_{n=1}^{n_max} ½ℏω_n where ω_n = cπn/L (SI units, Joules).
///
/// The sum diverges as n_max → ∞; n_max serves as an ultraviolet cutoff.
#[instrument(level = "trace")]
pub fn zero_point_energy_sum(box_size_m: f64, n_max: usize) -> Result<f64, MimamsaError> {
    require_finite(box_size_m, "zero_point_energy_sum")?;
    if box_size_m <= 0.0 {
        warn!(
            box_size_m,
            "zero_point_energy_sum: box_size must be positive"
        );
        return Err(MimamsaError::Computation(
            "zero_point_energy_sum: box_size must be positive".to_string(),
        ));
    }
    if n_max == 0 {
        return Ok(0.0);
    }
    let mut total = 0.0;
    let prefactor = 0.5 * HBAR * C * PI / box_size_m;
    for n in 1..=n_max {
        total += prefactor * n as f64;
    }
    ensure_finite(total, "zero_point_energy_sum")
}

/// Casimir force per unit area between parallel conducting plates.
///
/// F/A = -π²ℏc / (240 d⁴) (SI units, N/m²).
///
/// The force is attractive (negative).
#[instrument(level = "trace")]
#[inline]
pub fn casimir_force_per_area(plate_separation_m: f64) -> Result<f64, MimamsaError> {
    require_finite(plate_separation_m, "casimir_force_per_area")?;
    if plate_separation_m <= 0.0 {
        warn!(
            plate_separation_m,
            "casimir_force_per_area: separation must be positive"
        );
        return Err(MimamsaError::Computation(
            "casimir_force_per_area: plate separation must be positive".to_string(),
        ));
    }
    let d4 = plate_separation_m.powi(4);
    ensure_finite(-PI * PI * HBAR * C / (240.0 * d4), "casimir_force_per_area")
}

/// Casimir energy per unit area between parallel conducting plates.
///
/// E/A = -π²ℏc / (720 d³) (SI units, J/m²).
#[instrument(level = "trace")]
#[inline]
pub fn casimir_energy_per_area(plate_separation_m: f64) -> Result<f64, MimamsaError> {
    require_finite(plate_separation_m, "casimir_energy_per_area")?;
    if plate_separation_m <= 0.0 {
        warn!(
            plate_separation_m,
            "casimir_energy_per_area: separation must be positive"
        );
        return Err(MimamsaError::Computation(
            "casimir_energy_per_area: plate separation must be positive".to_string(),
        ));
    }
    let d3 = plate_separation_m.powi(3);
    ensure_finite(
        -PI * PI * HBAR * C / (720.0 * d3),
        "casimir_energy_per_area",
    )
}

/// Regularized vacuum energy density with hard momentum cutoff Λ.
///
/// ρ_vac = Λ⁴ / (16π²) (natural units, GeV⁴).
#[instrument(level = "trace")]
#[inline]
pub fn regularized_vacuum_energy_density(cutoff_gev: f64) -> Result<f64, MimamsaError> {
    require_finite(cutoff_gev, "regularized_vacuum_energy_density")?;
    if cutoff_gev <= 0.0 {
        warn!(
            cutoff_gev,
            "regularized_vacuum_energy_density: cutoff must be positive"
        );
        return Err(MimamsaError::Computation(
            "regularized_vacuum_energy_density: cutoff must be positive".to_string(),
        ));
    }
    let l4 = cutoff_gev.powi(4);
    ensure_finite(l4 / (16.0 * PI * PI), "regularized_vacuum_energy_density")
}

/// Vacuum energy density with dimensional regularization for a scalar field.
///
/// ρ_vac = m⁴/(64π²) · [ln(m²/μ²) - 3/2] (natural units, GeV⁴).
///
/// # Arguments
/// * `mass_gev` — Scalar field mass (GeV).
/// * `mu_gev` — Renormalization scale (GeV).
#[instrument(level = "trace")]
pub fn vacuum_energy_density_dimreg(mass_gev: f64, mu_gev: f64) -> Result<f64, MimamsaError> {
    require_finite(mass_gev, "vacuum_energy_density_dimreg")?;
    require_finite(mu_gev, "vacuum_energy_density_dimreg")?;
    if mass_gev <= 0.0 || mu_gev <= 0.0 {
        warn!(
            mass_gev,
            mu_gev, "vacuum_energy_density_dimreg: mass and scale must be positive"
        );
        return Err(MimamsaError::Computation(
            "vacuum_energy_density_dimreg: mass and scale must be positive".to_string(),
        ));
    }
    let m2 = mass_gev * mass_gev;
    let mu2 = mu_gev * mu_gev;
    let m4 = m2 * m2;
    ensure_finite(
        m4 / (64.0 * PI * PI) * ((m2 / mu2).ln() - 1.5),
        "vacuum_energy_density_dimreg",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_point_energy_positive() {
        let e = zero_point_energy(1e15).unwrap();
        assert!(e > 0.0);
    }

    #[test]
    fn test_zero_point_energy_zero_omega() {
        let e = zero_point_energy(0.0).unwrap();
        assert!((e - 0.0).abs() < 1e-50);
    }

    #[test]
    fn test_zero_point_energy_negative_rejected() {
        assert!(zero_point_energy(-1.0).is_err());
    }

    #[test]
    fn test_casimir_force_attractive() {
        let f = casimir_force_per_area(1e-6).unwrap();
        assert!(f < 0.0, "Casimir force should be attractive (negative)");
    }

    #[test]
    fn test_casimir_force_1um() {
        // At 1 μm separation: F/A ≈ -1.3e-3 N/m²
        let f = casimir_force_per_area(1e-6).unwrap();
        assert!(f < -1e-4 && f > -1e-2, "Casimir force at 1μm: {f}");
    }

    #[test]
    fn test_casimir_energy_negative() {
        let e = casimir_energy_per_area(1e-6).unwrap();
        assert!(e < 0.0, "Casimir energy should be negative (bound state)");
    }

    #[test]
    fn test_casimir_force_scales_as_d4() {
        let f1 = casimir_force_per_area(1e-6).unwrap();
        let f2 = casimir_force_per_area(2e-6).unwrap();
        // F ∝ 1/d⁴ → halving separation → 16× force
        let ratio = f1 / f2;
        assert!((ratio - 16.0).abs() < 0.01);
    }

    #[test]
    fn test_regularized_vacuum_scales_as_lambda4() {
        let r1 = regularized_vacuum_energy_density(100.0).unwrap();
        let r2 = regularized_vacuum_energy_density(200.0).unwrap();
        let ratio = r2 / r1;
        assert!((ratio - 16.0).abs() < 0.01);
    }

    #[test]
    fn test_zero_separation_rejected() {
        assert!(casimir_force_per_area(0.0).is_err());
        assert!(casimir_force_per_area(-1.0).is_err());
    }

    #[test]
    fn test_vacuum_dimreg() {
        let rho = vacuum_energy_density_dimreg(1.0, 1.0).unwrap();
        // ln(1) = 0, so ρ = m⁴/(64π²) * (-3/2)
        assert!(rho < 0.0);
    }

    #[test]
    fn test_zero_point_sum() {
        let e = zero_point_energy_sum(1.0, 10).unwrap();
        assert!(e > 0.0);
    }

    #[test]
    fn test_zero_point_sum_zero_n() {
        let e = zero_point_energy_sum(1.0, 0).unwrap();
        assert!((e - 0.0).abs() < 1e-50);
    }
}
