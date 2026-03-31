//! Geodesics — paths through curved spacetime.
//!
//! Geodesics are the "straightest possible" paths in curved geometry.
//! In GR, freely falling objects follow geodesics.

use serde::{Deserialize, Serialize};

use crate::error::{MimamsaError, ensure_finite, require_all_finite};
use tracing::instrument;

/// A point in spacetime with position and four-velocity.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GeodesicPoint {
    /// Coordinate time.
    pub t: f64,
    /// Radial coordinate.
    pub r: f64,
    /// Polar angle (radians).
    pub theta: f64,
    /// Azimuthal angle (radians).
    pub phi: f64,
}

/// Classification of geodesics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum GeodesicType {
    /// Massive particle (timelike).
    Timelike,
    /// Photon (null/lightlike).
    Null,
    /// Tachyonic (spacelike) — theoretical only.
    Spacelike,
}

/// Effective potential for radial motion in Schwarzschild geometry.
/// V_eff(r) = (1 - r_s/r)(1 + L²/(r²c²))
/// For null geodesics: V_eff(r) = (1 - r_s/r) * L²/r²
#[instrument(level = "trace")]
#[inline]
pub fn schwarzschild_effective_potential(
    rs: f64,
    r: f64,
    angular_momentum: f64,
    geodesic_type: GeodesicType,
) -> Result<f64, MimamsaError> {
    require_all_finite(
        &[rs, r, angular_momentum],
        "schwarzschild_effective_potential",
    )?;
    let factor = 1.0 - rs / r;
    ensure_finite(
        match geodesic_type {
            GeodesicType::Timelike => {
                let l2 = angular_momentum * angular_momentum;
                factor * (1.0 + l2 / (r * r))
            }
            GeodesicType::Null => {
                let l2 = angular_momentum * angular_momentum;
                factor * l2 / (r * r)
            }
            GeodesicType::Spacelike => {
                // Theoretical
                let l2 = angular_momentum * angular_momentum;
                factor * (-1.0 + l2 / (r * r))
            }
        },
        "schwarzschild_effective_potential",
    )
}

/// Deflection angle for light passing mass M at impact parameter b.
/// Δφ ≈ 4GM/(bc²) (weak field, first order).
#[instrument(level = "trace")]
#[inline]
pub fn light_deflection_weak_field(
    mass_kg: f64,
    impact_parameter: f64,
) -> Result<f64, MimamsaError> {
    use crate::constants::{C, G};
    require_all_finite(&[mass_kg, impact_parameter], "light_deflection_weak_field")?;
    ensure_finite(
        4.0 * G * mass_kg / (impact_parameter * C * C),
        "light_deflection_weak_field",
    )
}

/// Shapiro time delay for signal passing mass M.
/// Δt ≈ (4GM/c³) * ln(4r₁r₂/b²) where r₁,r₂ are emitter/receiver distances.
#[instrument(level = "trace")]
pub fn shapiro_delay(
    mass_kg: f64,
    r1: f64,
    r2: f64,
    impact_parameter: f64,
) -> Result<f64, MimamsaError> {
    use crate::constants::{C, G};
    require_all_finite(&[mass_kg, r1, r2, impact_parameter], "shapiro_delay")?;
    let c3 = C.powi(3);
    let prefactor = 4.0 * G * mass_kg / c3;
    ensure_finite(
        prefactor * (4.0 * r1 * r2 / (impact_parameter * impact_parameter)).ln(),
        "shapiro_delay",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    const M_SUN: f64 = 1.989e30;
    const AU: f64 = 1.496e11; // meters

    #[test]
    fn test_light_deflection_sun() {
        // Sun deflects starlight by ~1.75 arcseconds
        let r_sun = 6.957e8; // solar radius in meters
        let deflection_rad = light_deflection_weak_field(M_SUN, r_sun).unwrap();
        let deflection_arcsec = deflection_rad * 180.0 / PI * 3600.0;
        assert!((deflection_arcsec - 1.75).abs() < 0.02);
    }

    #[test]
    fn test_effective_potential_at_infinity() {
        // At large r, V_eff → 1 for timelike geodesics
        let rs = 2953.0; // Sun's Schwarzschild radius
        let v = schwarzschild_effective_potential(rs, 1e15, 1.0, GeodesicType::Timelike).unwrap();
        assert!((v - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_shapiro_delay_positive() {
        let delay = shapiro_delay(M_SUN, AU, AU, 6.957e8).unwrap();
        assert!(delay > 0.0);
        // Should be on the order of ~200 microseconds for Sun
        assert!(delay > 1e-5 && delay < 1e-3);
    }

    #[test]
    fn test_effective_potential_null_geodesic() {
        let rs = 2953.0;
        let v = schwarzschild_effective_potential(rs, 1e10, 1e5, GeodesicType::Null).unwrap();
        assert!(v > 0.0);
    }

    #[test]
    fn test_effective_potential_spacelike_geodesic() {
        let rs = 2953.0;
        let v = schwarzschild_effective_potential(rs, 1e10, 1e8, GeodesicType::Spacelike).unwrap();
        assert!(v.is_finite());
    }
}
