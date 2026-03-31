//! Special relativity — Lorentz transformations, four-vectors, invariants.

use serde::{Deserialize, Serialize};

use crate::error::MimamsaError;

/// Speed of light in vacuum (m/s).
pub const C: f64 = 299_792_458.0;

/// Speed of light squared.
pub const C2: f64 = C * C;

/// Lorentz factor γ = 1/√(1 - v²/c²).
///
/// Returns error if v ≥ c (superluminal).
#[inline]
pub fn lorentz_factor(v: f64) -> Result<f64, MimamsaError> {
    let beta = v / C;
    let beta2 = beta * beta;
    if beta2 >= 1.0 {
        return Err(MimamsaError::Superluminal { v });
    }
    Ok(1.0 / (1.0 - beta2).sqrt())
}

/// Velocity parameter β = v/c.
#[must_use]
#[inline]
pub fn beta(v: f64) -> f64 {
    v / C
}

/// Time dilation: Δt' = γΔt.
#[inline]
pub fn time_dilation(proper_time: f64, v: f64) -> Result<f64, MimamsaError> {
    Ok(proper_time * lorentz_factor(v)?)
}

/// Length contraction: L' = L/γ.
#[inline]
pub fn length_contraction(proper_length: f64, v: f64) -> Result<f64, MimamsaError> {
    Ok(proper_length / lorentz_factor(v)?)
}

/// Relativistic kinetic energy: E_k = (γ - 1)mc².
#[inline]
pub fn kinetic_energy(mass_kg: f64, v: f64) -> Result<f64, MimamsaError> {
    let gamma = lorentz_factor(v)?;
    Ok((gamma - 1.0) * mass_kg * C2)
}

/// Total relativistic energy: E = γmc².
#[inline]
pub fn total_energy(mass_kg: f64, v: f64) -> Result<f64, MimamsaError> {
    Ok(lorentz_factor(v)? * mass_kg * C2)
}

/// Rest energy: E₀ = mc².
#[must_use]
#[inline]
pub fn rest_energy(mass_kg: f64) -> f64 {
    mass_kg * C2
}

/// Relativistic momentum: p = γmv.
#[inline]
pub fn relativistic_momentum(mass_kg: f64, v: f64) -> Result<f64, MimamsaError> {
    Ok(lorentz_factor(v)? * mass_kg * v)
}

/// Relativistic velocity addition: u' = (u + v) / (1 + uv/c²).
#[must_use]
#[inline]
pub fn velocity_addition(u: f64, v: f64) -> f64 {
    (u + v) / (1.0 + u * v / C2)
}

/// Relativistic Doppler factor for radial motion.
/// f_obs = f_src * √((1 - β) / (1 + β)) for recession (v > 0).
#[inline]
pub fn doppler_factor(v: f64) -> Result<f64, MimamsaError> {
    let b = beta(v);
    if b.abs() >= 1.0 {
        return Err(MimamsaError::Superluminal { v });
    }
    Ok(((1.0 - b) / (1.0 + b)).sqrt())
}

/// Four-vector in Minkowski space (ct, x, y, z).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FourVector {
    pub ct: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl FourVector {
    /// Create a new four-vector.
    #[must_use]
    pub fn new(ct: f64, x: f64, y: f64, z: f64) -> Self {
        Self { ct, x, y, z }
    }

    /// Minkowski inner product: s² = -(ct)² + x² + y² + z² (signature -+++)
    #[must_use]
    #[inline]
    pub fn invariant_interval(&self) -> f64 {
        -self.ct * self.ct + self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Classify the interval: timelike (< 0), lightlike (= 0), spacelike (> 0).
    #[must_use]
    pub fn interval_type(&self) -> IntervalType {
        let s2 = self.invariant_interval();
        if s2 < -1e-12 {
            IntervalType::Timelike
        } else if s2 > 1e-12 {
            IntervalType::Spacelike
        } else {
            IntervalType::Lightlike
        }
    }

    /// Lorentz boost along x-axis.
    pub fn boost_x(&self, v: f64) -> Result<Self, MimamsaError> {
        let gamma = lorentz_factor(v)?;
        let b = beta(v);
        Ok(Self {
            ct: gamma * (self.ct - b * self.x),
            x: gamma * (self.x - b * self.ct),
            y: self.y,
            z: self.z,
        })
    }
}

/// Classification of spacetime intervals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum IntervalType {
    /// Causally connected — inside the light cone.
    Timelike,
    /// On the light cone — null separation.
    Lightlike,
    /// Causally disconnected — outside the light cone.
    Spacelike,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lorentz_factor_zero() {
        let gamma = lorentz_factor(0.0).unwrap();
        assert!((gamma - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_lorentz_factor_high_v() {
        // 0.866c → γ ≈ 2.0
        let v = 0.866 * C;
        let gamma = lorentz_factor(v).unwrap();
        assert!((gamma - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_superluminal_rejected() {
        assert!(lorentz_factor(C * 1.01).is_err());
    }

    #[test]
    fn test_rest_energy_electron() {
        // Electron mass: 9.109e-31 kg → E₀ ≈ 8.187e-14 J ≈ 0.511 MeV
        let e0 = rest_energy(9.109e-31);
        let mev = e0 / 1.602e-13;
        assert!((mev - 0.511).abs() < 0.001);
    }

    #[test]
    fn test_velocity_addition_subluminal() {
        // Two objects each at 0.9c → combined < c
        let u = velocity_addition(0.9 * C, 0.9 * C);
        assert!(u < C);
        // Should be ~0.9945c
        assert!((u / C - 0.9945).abs() < 0.001);
    }

    #[test]
    fn test_four_vector_lightlike() {
        // Photon: travels 1 light-second in 1 second
        let photon = FourVector::new(C, C, 0.0, 0.0);
        assert_eq!(photon.interval_type(), IntervalType::Lightlike);
    }

    #[test]
    fn test_four_vector_timelike() {
        // Massive particle: at rest
        let rest = FourVector::new(C, 0.0, 0.0, 0.0);
        assert_eq!(rest.interval_type(), IntervalType::Timelike);
    }

    #[test]
    fn test_boost_preserves_interval() {
        // Use natural units (ct, x in meters) for clean interval check
        let event = FourVector::new(3.0, 2.0, 0.0, 0.0);
        let boosted = event.boost_x(0.5 * C).unwrap();
        let s2_orig = event.invariant_interval();
        let s2_boosted = boosted.invariant_interval();
        assert!((s2_orig - s2_boosted).abs() / s2_orig.abs() < 1e-10);
    }

    #[test]
    fn test_time_dilation_muon() {
        // Muon at 0.994c: γ ≈ 9.14, proper lifetime 2.2μs → observed ~20μs
        let v = 0.994 * C;
        let observed = time_dilation(2.2e-6, v).unwrap();
        assert!((observed / 2.2e-6 - 9.14).abs() < 0.1);
    }

    #[test]
    fn test_doppler_blueshift() {
        // Approaching: v < 0 → factor > 1 (blueshift)
        let f = doppler_factor(-0.5 * C).unwrap();
        assert!(f > 1.0);
    }

    #[test]
    fn test_doppler_redshift() {
        // Receding: v > 0 → factor < 1 (redshift)
        let f = doppler_factor(0.5 * C).unwrap();
        assert!(f < 1.0);
    }
}
