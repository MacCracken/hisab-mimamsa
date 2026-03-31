//! Spacetime metrics — Schwarzschild, Kerr, FLRW, Minkowski.

use serde::{Deserialize, Serialize};

use tracing::warn;

use crate::error::{ensure_finite, require_all_finite, require_finite, MimamsaError};

// Re-export from centralized constants for backward compatibility.
pub use crate::constants::{C, G};

/// Schwarzschild radius: r_s = 2GM/c².
#[inline]
pub fn schwarzschild_radius(mass_kg: f64) -> Result<f64, MimamsaError> {
    require_finite(mass_kg, "schwarzschild_radius")?;
    ensure_finite(2.0 * G * mass_kg / (C * C), "schwarzschild_radius")
}

/// Gravitational time dilation at radius r from mass M.
/// τ/t = √(1 - r_s/r). Returns error at or inside event horizon.
#[inline]
pub fn gravitational_time_dilation(mass_kg: f64, r: f64) -> Result<f64, MimamsaError> {
    require_all_finite(&[mass_kg, r], "gravitational_time_dilation")?;
    let rs = schwarzschild_radius(mass_kg)?;
    if r <= rs {
        warn!(r, rs, "time dilation requested inside event horizon");
        return Err(MimamsaError::Singularity {
            location: format!("r={r:.3e}"),
            detail: format!("inside event horizon (r_s={rs:.3e})"),
        });
    }
    ensure_finite((1.0 - rs / r).sqrt(), "gravitational_time_dilation")
}

/// Gravitational redshift: λ_obs/λ_emit = 1/√(1 - r_s/r).
#[inline]
pub fn gravitational_redshift(mass_kg: f64, r_emit: f64) -> Result<f64, MimamsaError> {
    require_all_finite(&[mass_kg, r_emit], "gravitational_redshift")?;
    ensure_finite(1.0 / gravitational_time_dilation(mass_kg, r_emit)?, "gravitational_redshift")
}

/// ISCO (innermost stable circular orbit) for Schwarzschild: r_isco = 3r_s.
#[inline]
pub fn schwarzschild_isco(mass_kg: f64) -> Result<f64, MimamsaError> {
    require_finite(mass_kg, "schwarzschild_isco")?;
    ensure_finite(3.0 * schwarzschild_radius(mass_kg)?, "schwarzschild_isco")
}

/// Photon sphere radius for Schwarzschild: r_ph = 1.5 r_s.
#[inline]
pub fn photon_sphere_radius(mass_kg: f64) -> Result<f64, MimamsaError> {
    require_finite(mass_kg, "photon_sphere_radius")?;
    ensure_finite(1.5 * schwarzschild_radius(mass_kg)?, "photon_sphere_radius")
}

/// Orbital velocity for circular orbit in Schwarzschild geometry.
/// v = √(GM/r) * 1/√(1 - r_s/r) for r > r_isco.
pub fn schwarzschild_orbital_velocity(mass_kg: f64, r: f64) -> Result<f64, MimamsaError> {
    require_all_finite(&[mass_kg, r], "schwarzschild_orbital_velocity")?;
    let rs = schwarzschild_radius(mass_kg)?;
    if r <= rs {
        warn!(r, rs, "orbital velocity requested inside event horizon");
        return Err(MimamsaError::Singularity {
            location: format!("r={r:.3e}"),
            detail: "inside event horizon".to_string(),
        });
    }
    let v_newton = (G * mass_kg / r).sqrt();
    ensure_finite(v_newton / (1.0 - rs / r).sqrt(), "schwarzschild_orbital_velocity")
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
        let rs = schwarzschild_radius(M_SUN).unwrap();
        assert!((rs - 2953.0).abs() < 5.0);
    }

    #[test]
    fn test_schwarzschild_radius_earth() {
        // Earth: r_s ≈ 8.87 mm
        let rs = schwarzschild_radius(5.972e24).unwrap();
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
        let rs = schwarzschild_radius(M_SUN).unwrap();
        let isco = schwarzschild_isco(M_SUN).unwrap();
        assert!((isco - 3.0 * rs).abs() < 1e-6);
    }

    #[test]
    fn test_photon_sphere() {
        let rs = schwarzschild_radius(M_SUN).unwrap();
        let rph = photon_sphere_radius(M_SUN).unwrap();
        assert!((rph - 1.5 * rs).abs() < 1e-6);
    }
}
