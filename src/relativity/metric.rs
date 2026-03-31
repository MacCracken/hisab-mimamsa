//! Spacetime metrics — Schwarzschild, Kerr, FLRW, Minkowski.

use serde::{Deserialize, Serialize};

use crate::error::MimamsaError;

/// Gravitational constant G (m³ kg⁻¹ s⁻²).
pub const G: f64 = 6.674_30e-11;

/// Speed of light (m/s).
pub const C: f64 = super::lorentz::C;

/// Schwarzschild radius: r_s = 2GM/c².
#[must_use]
#[inline]
pub fn schwarzschild_radius(mass_kg: f64) -> f64 {
    2.0 * G * mass_kg / (C * C)
}

/// Gravitational time dilation at radius r from mass M.
/// τ/t = √(1 - r_s/r). Returns error at or inside event horizon.
#[inline]
pub fn gravitational_time_dilation(mass_kg: f64, r: f64) -> Result<f64, MimamsaError> {
    let rs = schwarzschild_radius(mass_kg);
    if r <= rs {
        return Err(MimamsaError::Singularity {
            location: format!("r={r:.3e}"),
            detail: format!("inside event horizon (r_s={rs:.3e})"),
        });
    }
    Ok((1.0 - rs / r).sqrt())
}

/// Gravitational redshift: λ_obs/λ_emit = 1/√(1 - r_s/r).
#[inline]
pub fn gravitational_redshift(mass_kg: f64, r_emit: f64) -> Result<f64, MimamsaError> {
    Ok(1.0 / gravitational_time_dilation(mass_kg, r_emit)?)
}

/// ISCO (innermost stable circular orbit) for Schwarzschild: r_isco = 3r_s.
#[must_use]
#[inline]
pub fn schwarzschild_isco(mass_kg: f64) -> f64 {
    3.0 * schwarzschild_radius(mass_kg)
}

/// Photon sphere radius for Schwarzschild: r_ph = 1.5 r_s.
#[must_use]
#[inline]
pub fn photon_sphere_radius(mass_kg: f64) -> f64 {
    1.5 * schwarzschild_radius(mass_kg)
}

/// Orbital velocity for circular orbit in Schwarzschild geometry.
/// v = √(GM/r) * 1/√(1 - r_s/r) for r > r_isco.
pub fn schwarzschild_orbital_velocity(mass_kg: f64, r: f64) -> Result<f64, MimamsaError> {
    let rs = schwarzschild_radius(mass_kg);
    if r <= rs {
        return Err(MimamsaError::Singularity {
            location: format!("r={r:.3e}"),
            detail: "inside event horizon".to_string(),
        });
    }
    let v_newton = (G * mass_kg / r).sqrt();
    Ok(v_newton / (1.0 - rs / r).sqrt())
}

/// Metric signature for classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MetricSignature {
    /// (-+++) Minkowski / mostly plus.
    MostlyPlus,
    /// (+---) Mostly minus (particle physics convention).
    MostlyMinus,
}

/// Known spacetime metrics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SpacetimeMetric {
    /// Flat spacetime.
    Minkowski,
    /// Non-rotating, uncharged black hole.
    Schwarzschild,
    /// Rotating black hole (spin parameter a).
    Kerr,
    /// Expanding universe (Friedmann-Lemaître-Robertson-Walker).
    FLRW,
    /// Charged, non-rotating black hole.
    ReissnerNordstrom,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Solar mass in kg.
    const M_SUN: f64 = 1.989e30;

    #[test]
    fn test_schwarzschild_radius_sun() {
        // Sun: r_s ≈ 2953 m ≈ 2.95 km
        let rs = schwarzschild_radius(M_SUN);
        assert!((rs - 2953.0).abs() < 5.0);
    }

    #[test]
    fn test_schwarzschild_radius_earth() {
        // Earth: r_s ≈ 8.87 mm
        let rs = schwarzschild_radius(5.972e24);
        assert!((rs - 0.00887).abs() < 0.001);
    }

    #[test]
    fn test_time_dilation_earth_surface() {
        // Earth surface: r = 6.371e6 m, negligible dilation
        let factor = gravitational_time_dilation(5.972e24, 6.371e6).unwrap();
        // Should be very close to 1.0 (off by ~7e-10)
        assert!((factor - 1.0).abs() < 1e-8);
    }

    #[test]
    fn test_singularity_inside_horizon() {
        assert!(gravitational_time_dilation(M_SUN, 1000.0).is_err());
    }

    #[test]
    fn test_isco_schwarzschild() {
        // ISCO = 3 * r_s
        let rs = schwarzschild_radius(M_SUN);
        let isco = schwarzschild_isco(M_SUN);
        assert!((isco - 3.0 * rs).abs() < 1e-6);
    }

    #[test]
    fn test_photon_sphere() {
        let rs = schwarzschild_radius(M_SUN);
        let rph = photon_sphere_radius(M_SUN);
        assert!((rph - 1.5 * rs).abs() < 1e-6);
    }
}
