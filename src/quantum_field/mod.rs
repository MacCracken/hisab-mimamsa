//! Quantum Field Theory — propagators, vacuum energy, running couplings, Feynman diagrams.
//!
//! Computational QFT built on the hisab math foundation. Provides momentum-space
//! propagators, vacuum energy calculations, renormalization group running of coupling
//! constants, and tree-level Feynman diagram evaluation.
//!
//! # Units
//!
//! This module uses **natural units** (ℏ = c = 1) with energies in GeV internally.
//! The vacuum submodule also provides SI-unit functions (Casimir effect) using
//! constants from [`crate::constants`]. All function docs state which unit system
//! is used.
//!
//! # Submodules
//!
//! - [`propagator`] — Free-field propagators (Klein-Gordon, Dirac, gauge boson)
//! - [`vacuum`] — Zero-point energy, Casimir effect, regularized vacuum energy
//! - [`coupling`] — Running coupling constants, β-functions, asymptotic freedom
//! - [`feynman`] — Feynman diagram types, tree-level amplitudes, cross-sections

pub mod coupling;
pub mod feynman;
pub mod propagator;
pub mod vacuum;

use serde::{Deserialize, Serialize};

use crate::error::{ensure_finite, require_all_finite, MimamsaError};

/// Classification of quantum fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum FieldType {
    /// Spin-0 scalar field (Klein-Gordon).
    Scalar,
    /// Spin-½ fermion field (Dirac).
    Fermion,
    /// Spin-1 gauge boson field.
    GaugeBoson,
}

/// Gauge choice for gauge boson propagators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum GaugeChoice {
    /// Feynman gauge (ξ = 1): simplest propagator form.
    Feynman,
}

/// Four-momentum vector (E, p_x, p_y, p_z) in natural units (GeV).
///
/// Uses mostly-minus signature: p² = E² - |**p**|².
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FourMomentum {
    /// Energy component (GeV).
    pub e: f64,
    /// x-momentum (GeV).
    pub px: f64,
    /// y-momentum (GeV).
    pub py: f64,
    /// z-momentum (GeV).
    pub pz: f64,
}

/// Maximum component magnitude to prevent overflow in quadratic operations.
const MAX_MOMENTUM_COMPONENT: f64 = 1.34e154;

impl FourMomentum {
    /// Create a new four-momentum with input validation.
    pub fn new(e: f64, px: f64, py: f64, pz: f64) -> Result<Self, MimamsaError> {
        require_all_finite(&[e, px, py, pz], "FourMomentum::new")?;
        for &c in &[e, px, py, pz] {
            if c.abs() > MAX_MOMENTUM_COMPONENT {
                return Err(MimamsaError::Computation(
                    "FourMomentum component magnitude too large".to_string(),
                ));
            }
        }
        Ok(Self { e, px, py, pz })
    }

    /// Lorentz-invariant mass squared: p² = E² - |**p**|² (mostly-minus).
    #[inline]
    pub fn invariant_mass_sq(&self) -> Result<f64, MimamsaError> {
        ensure_finite(
            self.e * self.e - self.px * self.px - self.py * self.py - self.pz * self.pz,
            "FourMomentum::invariant_mass_sq",
        )
    }

    /// Spatial momentum magnitude |**p**| = √(px² + py² + pz²).
    #[inline]
    pub fn spatial_magnitude(&self) -> Result<f64, MimamsaError> {
        ensure_finite(
            (self.px * self.px + self.py * self.py + self.pz * self.pz).sqrt(),
            "FourMomentum::spatial_magnitude",
        )
    }

    /// Add two four-momenta.
    #[inline]
    pub fn add(&self, other: &Self) -> Result<Self, MimamsaError> {
        Self::new(
            self.e + other.e,
            self.px + other.px,
            self.py + other.py,
            self.pz + other.pz,
        )
    }

    /// Subtract two four-momenta.
    #[inline]
    pub fn sub(&self, other: &Self) -> Result<Self, MimamsaError> {
        Self::new(
            self.e - other.e,
            self.px - other.px,
            self.py - other.py,
            self.pz - other.pz,
        )
    }

    /// Minkowski inner product: p · q = E_p E_q - **p** · **q** (mostly-minus).
    #[inline]
    pub fn dot(&self, other: &Self) -> Result<f64, MimamsaError> {
        ensure_finite(
            self.e * other.e - self.px * other.px - self.py * other.py - self.pz * other.pz,
            "FourMomentum::dot",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_momentum_on_shell_massless() {
        // Photon: E = |p|, so p² = 0
        let p = FourMomentum::new(10.0, 10.0, 0.0, 0.0).unwrap();
        let m2 = p.invariant_mass_sq().unwrap();
        assert!(m2.abs() < 1e-10);
    }

    #[test]
    fn test_four_momentum_on_shell_massive() {
        // Electron at rest: E = m, p = 0 → p² = m²
        let m = crate::constants::M_ELECTRON_GEV;
        let p = FourMomentum::new(m, 0.0, 0.0, 0.0).unwrap();
        let m2 = p.invariant_mass_sq().unwrap();
        assert!((m2 - m * m).abs() < 1e-20);
    }

    #[test]
    fn test_four_momentum_addition() {
        let p1 = FourMomentum::new(5.0, 3.0, 0.0, 0.0).unwrap();
        let p2 = FourMomentum::new(5.0, -3.0, 0.0, 0.0).unwrap();
        let total = p1.add(&p2).unwrap();
        assert!((total.e - 10.0).abs() < 1e-12);
        assert!(total.px.abs() < 1e-12);
    }

    #[test]
    fn test_four_momentum_dot_product() {
        let p = FourMomentum::new(5.0, 3.0, 0.0, 4.0).unwrap();
        // p·p = E² - |p|² = 25 - 9 - 16 = 0
        let pp = p.dot(&p).unwrap();
        assert!(pp.abs() < 1e-10);
    }

    #[test]
    fn test_four_momentum_overflow_rejected() {
        assert!(FourMomentum::new(f64::MAX, 0.0, 0.0, 0.0).is_err());
    }

    #[test]
    fn test_four_momentum_nan_rejected() {
        assert!(FourMomentum::new(f64::NAN, 0.0, 0.0, 0.0).is_err());
    }

    #[test]
    fn test_field_type_serde() {
        let ft = FieldType::Scalar;
        let json = serde_json::to_string(&ft).unwrap();
        let back: FieldType = serde_json::from_str(&json).unwrap();
        assert_eq!(ft, back);
    }
}
