//! Gravitational lensing — Einstein rings, magnification, image distortion.

use tracing::{instrument, warn};

use crate::constants::{C, G};
use crate::error::{MimamsaError, ensure_finite, require_all_finite, require_finite};

/// Einstein ring angular radius (radians).
/// θ_E = √(4GM * D_ls / (c² * D_l * D_s))
/// where D_l = lens distance, D_s = source distance, D_ls = lens-source distance.
#[instrument(level = "trace")]
#[inline]
pub fn einstein_ring_radius(mass_kg: f64, d_lens: f64, d_source: f64) -> Result<f64, MimamsaError> {
    require_all_finite(&[mass_kg, d_lens, d_source], "einstein_ring_radius")?;
    let d_ls = d_source - d_lens;
    if d_lens <= 0.0 || d_source <= 0.0 || d_ls <= 0.0 {
        warn!(
            d_lens,
            d_source, d_ls, "invalid lensing geometry for Einstein ring"
        );
        return Err(MimamsaError::Computation(format!(
            "invalid lensing geometry: d_lens={d_lens:.3e}, d_source={d_source:.3e}"
        )));
    }
    ensure_finite(
        (4.0 * G * mass_kg * d_ls / (C * C * d_lens * d_source)).sqrt(),
        "einstein_ring_radius",
    )
}

/// Point-source magnification for a point lens.
/// μ = (u² + 2) / (u * √(u² + 4))
/// where u = angular separation / θ_E.
#[instrument(level = "trace")]
#[inline]
pub fn point_lens_magnification(u: f64) -> Result<f64, MimamsaError> {
    require_finite(u, "point_lens_magnification")?;
    if u.abs() < 1e-15 {
        return Ok(f64::INFINITY); // perfect alignment → infinite magnification (point source)
    }
    let u2 = u * u;
    ensure_finite(
        (u2 + 2.0) / (u * (u2 + 4.0).sqrt()),
        "point_lens_magnification",
    )
}

/// Critical surface density for lensing (kg/m²).
/// Σ_cr = c²D_s / (4πGD_lD_ls)
#[instrument(level = "trace")]
#[inline]
pub fn critical_surface_density(d_lens: f64, d_source: f64) -> Result<f64, MimamsaError> {
    require_all_finite(&[d_lens, d_source], "critical_surface_density")?;
    let d_ls = d_source - d_lens;
    if d_lens <= 0.0 || d_source <= 0.0 || d_ls <= 0.0 {
        warn!(
            d_lens,
            d_source, d_ls, "invalid lensing geometry for critical density"
        );
        return Err(MimamsaError::Computation(format!(
            "invalid lensing geometry: d_lens={d_lens:.3e}, d_source={d_source:.3e}"
        )));
    }
    ensure_finite(
        C * C * d_source / (4.0 * std::f64::consts::PI * G * d_lens * d_ls),
        "critical_surface_density",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const M_SUN: f64 = 1.989e30;
    const KPC: f64 = 3.086e19; // kiloparsec in meters

    #[test]
    fn test_einstein_ring_positive() {
        let theta = einstein_ring_radius(M_SUN, 10.0 * KPC, 20.0 * KPC).unwrap();
        assert!(theta > 0.0);
    }

    #[test]
    fn test_einstein_ring_invalid_geometry() {
        // Source behind lens → error
        assert!(einstein_ring_radius(M_SUN, 20.0 * KPC, 10.0 * KPC).is_err());
        // Negative distance → error
        assert!(einstein_ring_radius(M_SUN, -1.0, 20.0 * KPC).is_err());
    }

    #[test]
    fn test_magnification_far_from_lens() {
        // Far from lens (u >> 1): magnification → 1
        let mu = point_lens_magnification(100.0).unwrap();
        assert!((mu - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_magnification_near_einstein_ring() {
        // At u = 1 (on Einstein ring): μ = 3/√5 ≈ 1.342
        let mu = point_lens_magnification(1.0).unwrap();
        assert!((mu - 3.0 / 5.0_f64.sqrt()).abs() < 0.001);
    }

    #[test]
    fn test_critical_density_positive() {
        let sigma = critical_surface_density(10.0 * KPC, 20.0 * KPC).unwrap();
        assert!(sigma > 0.0);
    }

    #[test]
    fn test_critical_density_invalid_geometry() {
        assert!(critical_surface_density(20.0 * KPC, 10.0 * KPC).is_err());
    }
}
