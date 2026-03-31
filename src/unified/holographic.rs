//! Holographic principle — entropy bounds, information-theoretic limits.
//!
//! Links general relativity (black hole thermodynamics) to information theory
//! via the Bekenstein and holographic bounds. All quantities in SI units.

use std::f64::consts::PI;

use tracing::warn;

use crate::constants::{C, G, HBAR, K_B};
use crate::error::{ensure_finite, require_finite, MimamsaError};
use crate::relativity::black_hole::bekenstein_hawking_entropy;

/// Maximum entropy for a system of given radius and energy (Bekenstein bound).
///
/// S_max = 2π k_B R E / (ℏc)
///
/// # Arguments
/// * `radius_m` — Bounding sphere radius (m, must be > 0).
/// * `energy_j` — Total energy of the system (J, must be > 0).
#[inline]
pub fn bekenstein_bound(radius_m: f64, energy_j: f64) -> Result<f64, MimamsaError> {
    require_finite(radius_m, "bekenstein_bound")?;
    require_finite(energy_j, "bekenstein_bound")?;
    if radius_m <= 0.0 {
        warn!(radius_m, "bekenstein_bound: radius must be positive");
        return Err(MimamsaError::Computation(
            "bekenstein_bound: radius must be positive".to_string(),
        ));
    }
    if energy_j <= 0.0 {
        warn!(energy_j, "bekenstein_bound: energy must be positive");
        return Err(MimamsaError::Computation(
            "bekenstein_bound: energy must be positive".to_string(),
        ));
    }
    ensure_finite(
        2.0 * PI * K_B * radius_m * energy_j / (HBAR * C),
        "bekenstein_bound",
    )
}

/// Maximum entropy for a region bounded by surface area A (holographic bound).
///
/// S_max = k_B A / (4 l_P²) where l_P² = ℏG/c³
///
/// # Arguments
/// * `area_m2` — Bounding surface area (m², must be > 0).
#[inline]
pub fn holographic_bound(area_m2: f64) -> Result<f64, MimamsaError> {
    require_finite(area_m2, "holographic_bound")?;
    if area_m2 <= 0.0 {
        return Err(MimamsaError::Computation(
            "holographic_bound: area must be positive".to_string(),
        ));
    }
    let lp2 = HBAR * G / C.powi(3);
    ensure_finite(K_B * area_m2 / (4.0 * lp2), "holographic_bound")
}

/// Convert entropy (J/K) to information content (bits).
///
/// I = S / (k_B ln 2)
#[inline]
pub fn information_content_bits(entropy_j_per_k: f64) -> Result<f64, MimamsaError> {
    require_finite(entropy_j_per_k, "information_content_bits")?;
    if entropy_j_per_k < 0.0 {
        return Err(MimamsaError::Computation(
            "information_content_bits: entropy must be non-negative".to_string(),
        ));
    }
    ensure_finite(
        entropy_j_per_k / (K_B * 2.0_f64.ln()),
        "information_content_bits",
    )
}

/// Information content of a Schwarzschild black hole (bits).
///
/// Calls [`bekenstein_hawking_entropy`] then converts to bits.
pub fn black_hole_information_bits(mass_kg: f64) -> Result<f64, MimamsaError> {
    require_finite(mass_kg, "black_hole_information_bits")?;
    if mass_kg <= 0.0 {
        return Err(MimamsaError::Computation(
            "black_hole_information_bits: mass must be positive".to_string(),
        ));
    }
    let entropy = bekenstein_hawking_entropy(mass_kg)?;
    information_content_bits(entropy)
}

/// Entropy of the cosmological (de Sitter) horizon.
///
/// S = π k_B c⁵ / (G ℏ H²)
///
/// This is the Gibbons-Hawking entropy of the cosmological event horizon
/// for a de Sitter universe with Hubble parameter H.
///
/// # Arguments
/// * `hubble_param_si` — Hubble parameter in s⁻¹ (must be > 0).
#[inline]
pub fn cosmological_horizon_entropy(hubble_param_si: f64) -> Result<f64, MimamsaError> {
    require_finite(hubble_param_si, "cosmological_horizon_entropy")?;
    if hubble_param_si <= 0.0 {
        warn!(
            hubble_param_si,
            "cosmological_horizon_entropy: H must be positive"
        );
        return Err(MimamsaError::Computation(
            "cosmological_horizon_entropy: H must be positive".to_string(),
        ));
    }
    let h2 = hubble_param_si * hubble_param_si;
    ensure_finite(
        PI * K_B * C.powi(5) / (G * HBAR * h2),
        "cosmological_horizon_entropy",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::relativity::metric::schwarzschild_radius;

    const M_SUN: f64 = 1.989e30;

    #[test]
    fn test_bekenstein_bound_positive() {
        let s = bekenstein_bound(1.0, 1.0).unwrap();
        assert!(s > 0.0);
    }

    #[test]
    fn test_holographic_bound_matches_bh_entropy() {
        // For a BH, holographic_bound(A_horizon) = bekenstein_hawking_entropy(M)
        let rs = schwarzschild_radius(M_SUN).unwrap();
        let area = 4.0 * PI * rs * rs;
        let s_holo = holographic_bound(area).unwrap();
        let s_bh = bekenstein_hawking_entropy(M_SUN).unwrap();
        let rel_diff = (s_holo - s_bh).abs() / s_bh;
        assert!(rel_diff < 1e-6, "holographic vs BH entropy: {rel_diff}");
    }

    #[test]
    fn test_information_1_bit() {
        // 1 bit = k_B ln(2)
        let one_bit_entropy = K_B * 2.0_f64.ln();
        let bits = information_content_bits(one_bit_entropy).unwrap();
        assert!((bits - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_black_hole_information_positive() {
        let bits = black_hole_information_bits(M_SUN).unwrap();
        assert!(bits > 1e70, "solar mass BH info: {bits:.2e} bits");
    }

    #[test]
    fn test_cosmological_horizon_entropy_positive() {
        // H0 ~ 2.18e-18 s^{-1}
        let h0_si = 67.4 / 3.085_677_581e19;
        let s = cosmological_horizon_entropy(h0_si).unwrap();
        assert!(s > 1e90, "cosmic horizon entropy: {s:.2e}");
    }

    #[test]
    fn test_negative_inputs_rejected() {
        assert!(bekenstein_bound(-1.0, 1.0).is_err());
        assert!(bekenstein_bound(1.0, -1.0).is_err());
        assert!(holographic_bound(-1.0).is_err());
        assert!(information_content_bits(-1.0).is_err());
        assert!(black_hole_information_bits(-1.0).is_err());
        assert!(cosmological_horizon_entropy(-1.0).is_err());
    }
}
