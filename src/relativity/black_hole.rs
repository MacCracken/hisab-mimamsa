//! Black hole thermodynamics — Hawking radiation, entropy, temperature.

use serde::{Deserialize, Serialize};

use super::metric::schwarzschild_radius;

// Re-export from centralized constants for backward compatibility.
pub use crate::constants::{HBAR, K_B};
use crate::constants::{C, G};
use crate::error::{ensure_finite, require_finite, MimamsaError};

/// Hawking temperature of a Schwarzschild black hole.
/// T_H = ℏc³ / (8πGMk_B)
#[inline]
pub fn hawking_temperature(mass_kg: f64) -> Result<f64, MimamsaError> {
    require_finite(mass_kg, "hawking_temperature")?;
    ensure_finite(HBAR * C.powi(3) / (8.0 * std::f64::consts::PI * G * mass_kg * K_B), "hawking_temperature")
}

/// Bekenstein-Hawking entropy: S = k_B * A / (4 * l_P²)
/// where A = 4πr_s² and l_P = √(ℏG/c³).
pub fn bekenstein_hawking_entropy(mass_kg: f64) -> Result<f64, MimamsaError> {
    require_finite(mass_kg, "bekenstein_hawking_entropy")?;
    let rs = schwarzschild_radius(mass_kg)?;
    let area = 4.0 * std::f64::consts::PI * rs * rs;
    let lp2 = HBAR * G / C.powi(3); // Planck length squared
    ensure_finite(K_B * area / (4.0 * lp2), "bekenstein_hawking_entropy")
}

/// Hawking evaporation time: t ≈ 5120πG²M³/(ℏc⁴).
#[inline]
pub fn evaporation_time(mass_kg: f64) -> Result<f64, MimamsaError> {
    require_finite(mass_kg, "evaporation_time")?;
    ensure_finite(5120.0 * std::f64::consts::PI * G * G * mass_kg.powi(3) / (HBAR * C.powi(4)), "evaporation_time")
}

/// Schwarzschild black hole surface gravity: κ = c⁴/(4GM).
#[inline]
pub fn surface_gravity(mass_kg: f64) -> Result<f64, MimamsaError> {
    require_finite(mass_kg, "surface_gravity")?;
    ensure_finite(C.powi(4) / (4.0 * G * mass_kg), "surface_gravity")
}

/// Black hole properties bundle.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BlackHoleProperties {
    pub mass_kg: f64,
    pub radius_m: f64,
    pub temperature_k: f64,
    pub entropy_j_per_k: f64,
    pub evaporation_time_s: f64,
    pub surface_gravity_m_s2: f64,
}

impl BlackHoleProperties {
    /// Compute all properties from mass.
    pub fn from_mass(mass_kg: f64) -> Result<Self, MimamsaError> {
        require_finite(mass_kg, "BlackHoleProperties::from_mass")?;
        Ok(Self {
            mass_kg,
            radius_m: schwarzschild_radius(mass_kg)?,
            temperature_k: hawking_temperature(mass_kg)?,
            entropy_j_per_k: bekenstein_hawking_entropy(mass_kg)?,
            evaporation_time_s: evaporation_time(mass_kg)?,
            surface_gravity_m_s2: surface_gravity(mass_kg)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const M_SUN: f64 = 1.989e30;

    #[test]
    fn test_hawking_temperature_solar_mass() {
        // Solar mass BH: T ≈ 6.17e-8 K (extremely cold)
        let t = hawking_temperature(M_SUN).unwrap();
        assert!(t > 1e-9 && t < 1e-6);
    }

    #[test]
    fn test_hawking_temperature_inversely_proportional() {
        // Smaller BH → hotter
        let t1 = hawking_temperature(M_SUN).unwrap();
        let t2 = hawking_temperature(M_SUN * 0.5).unwrap();
        assert!(t2 > t1);
        assert!((t2 / t1 - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_evaporation_time_solar_mass() {
        // Solar mass BH: evaporation time >> age of universe (~4.3e17 s)
        let t = evaporation_time(M_SUN).unwrap();
        assert!(t > 1e50); // ~2e67 years
    }

    #[test]
    fn test_entropy_increases_with_mass() {
        let s1 = bekenstein_hawking_entropy(M_SUN).unwrap();
        let s2 = bekenstein_hawking_entropy(2.0 * M_SUN).unwrap();
        // S ∝ M² (since A ∝ r_s² ∝ M²)
        assert!((s2 / s1 - 4.0).abs() < 0.1);
    }

    #[test]
    fn test_properties_bundle() {
        let props = BlackHoleProperties::from_mass(M_SUN).unwrap();
        assert!(props.radius_m > 2900.0 && props.radius_m < 3000.0);
        assert!(props.temperature_k > 0.0);
        assert!(props.entropy_j_per_k > 0.0);
    }
}
