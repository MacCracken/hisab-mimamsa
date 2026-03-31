//! Fixed point convergence — cosmic phase classification, manifestation intensity, Unity.
//!
//! Tracks the universe's thermodynamic distance from heat death (the fixed-point
//! attractor). As entropy approaches its maximum, all fields converge to ground
//! state and `manifestation_intensity` → 0.0 → Unity.

use serde::{Deserialize, Serialize};
use tracing::{instrument, warn};

use crate::cosmology::friedmann::CosmologicalParameters;
use crate::error::{MimamsaError, ensure_finite, require_finite};

/// Which energy component dominates the expansion at a given epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CosmicPhase {
    /// Radiation dominated (early universe, z > ~3400).
    RadiationDominated,
    /// Matter dominated (intermediate, ~0.3 < z < ~3400).
    MatterDominated,
    /// Dark energy dominated (late universe, z < ~0.3).
    DarkEnergyDominated,
}

/// Snapshot of the fixed-point convergence state at a given redshift.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FixedPointState {
    /// Manifestation intensity ∈ [0, 1]. Higher = further from heat death.
    pub intensity: f64,
    /// Unity parameter = 1.0 - intensity. Approaches 1 at heat death.
    pub unity_param: f64,
    /// Current cosmic phase.
    pub phase: CosmicPhase,
    /// Fraction of maximum entropy reached ∈ [0, 1].
    pub entropy_ratio: f64,
}

impl FixedPointState {
    /// Compute the full fixed-point state at redshift z.
    #[instrument(level = "debug", skip(params), ret)]
    pub fn at_redshift(params: &CosmologicalParameters, z: f64) -> Result<Self, MimamsaError> {
        let phase = cosmic_phase(params, z)?;
        let e_ratio = entropy_ratio(params, z)?;
        let intensity = manifestation_intensity(params, z)?;
        let unity = unity_parameter(intensity)?;
        Ok(Self {
            intensity,
            unity_param: unity,
            phase,
            entropy_ratio: e_ratio,
        })
    }
}

/// Determine which energy component dominates expansion at redshift z.
///
/// Compares Ω_r(1+z)⁴, Ω_m(1+z)³, and Ω_Λ to find the largest
/// contributor to H²(z).
#[instrument(level = "trace", skip(params))]
pub fn cosmic_phase(params: &CosmologicalParameters, z: f64) -> Result<CosmicPhase, MimamsaError> {
    require_finite(z, "cosmic_phase")?;
    if z < -1.0 {
        warn!(z, "cosmic_phase: z < -1 is unphysical");
        return Err(MimamsaError::InvalidCosmology(
            "z < -1 is unphysical".to_string(),
        ));
    }
    let z1 = 1.0 + z;
    let rho_r = params.omega_r * z1.powi(4);
    let rho_m = params.omega_m * z1.powi(3);
    let rho_lambda = params.omega_lambda;

    if rho_r >= rho_m && rho_r >= rho_lambda {
        Ok(CosmicPhase::RadiationDominated)
    } else if rho_m >= rho_lambda {
        Ok(CosmicPhase::MatterDominated)
    } else {
        Ok(CosmicPhase::DarkEnergyDominated)
    }
}

/// Fraction of maximum (horizon) entropy reached at redshift z.
///
/// S(z)/S_max = Ω_Λ / (Ω_r(1+z)⁴ + Ω_m(1+z)³ + Ω_k(1+z)² + Ω_Λ)
///
/// Derived from S_horizon ∝ 1/H² and the fact that at the de Sitter
/// attractor (t → ∞), H → H₀√Ω_Λ. The constant prefactors cancel
/// in the ratio.
///
/// Returns a value in [0, 1] that increases monotonically toward the future.
#[instrument(level = "trace", skip(params))]
pub fn entropy_ratio(params: &CosmologicalParameters, z: f64) -> Result<f64, MimamsaError> {
    require_finite(z, "entropy_ratio")?;
    if z < -1.0 {
        warn!(z, "entropy_ratio: z < -1 is unphysical");
        return Err(MimamsaError::InvalidCosmology(
            "entropy_ratio: z < -1 is unphysical".to_string(),
        ));
    }
    let z1 = 1.0 + z;
    let z2 = z1 * z1;
    let z3 = z2 * z1;
    let z4 = z3 * z1;

    let denom =
        params.omega_r * z4 + params.omega_m * z3 + params.omega_k * z2 + params.omega_lambda;

    if denom <= 0.0 {
        warn!(denom, "entropy_ratio: density sum non-positive");
        return Err(MimamsaError::Computation(
            "entropy_ratio: density sum non-positive".to_string(),
        ));
    }

    let ratio = params.omega_lambda / denom;
    // Clamp to [0, 1] for numerical safety
    let clamped = ratio.clamp(0.0, 1.0);
    ensure_finite(clamped, "entropy_ratio")
}

/// Manifestation intensity at redshift z.
///
/// A scalar in [0, 1] representing how far the universe is from heat death.
/// Defined as 1.0 − entropy_ratio: high when the universe is young and
/// far from equilibrium, trending to 0 at heat death.
///
/// For bhava Scale 6: the cosmic expansion contribution to manifestation.
#[instrument(level = "trace", skip(params))]
pub fn manifestation_intensity(
    params: &CosmologicalParameters,
    z: f64,
) -> Result<f64, MimamsaError> {
    let ratio = entropy_ratio(params, z)?;
    ensure_finite(1.0 - ratio, "manifestation_intensity")
}

/// Unity parameter: 1.0 − intensity.
///
/// Approaches 1.0 at heat death (maximum entropy, ground state convergence).
/// This is the fixed-point attractor value.
#[instrument(level = "trace")]
#[inline]
pub fn unity_parameter(intensity: f64) -> Result<f64, MimamsaError> {
    require_finite(intensity, "unity_parameter")?;
    if !(0.0..=1.0).contains(&intensity) {
        warn!(intensity, "unity_parameter: intensity out of [0, 1]");
        return Err(MimamsaError::Computation(
            "unity_parameter: intensity must be in [0, 1]".to_string(),
        ));
    }
    ensure_finite(1.0 - intensity, "unity_parameter")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn planck() -> CosmologicalParameters {
        CosmologicalParameters::planck2018()
    }

    #[test]
    fn test_cosmic_phase_radiation_early() {
        assert_eq!(
            cosmic_phase(&planck(), 10000.0).unwrap(),
            CosmicPhase::RadiationDominated
        );
    }

    #[test]
    fn test_cosmic_phase_matter_intermediate() {
        assert_eq!(
            cosmic_phase(&planck(), 10.0).unwrap(),
            CosmicPhase::MatterDominated
        );
    }

    #[test]
    fn test_cosmic_phase_dark_energy_now() {
        assert_eq!(
            cosmic_phase(&planck(), 0.0).unwrap(),
            CosmicPhase::DarkEnergyDominated
        );
    }

    #[test]
    fn test_entropy_ratio_z0() {
        // At z=0, ratio ≈ omega_lambda ≈ 0.685
        let ratio = entropy_ratio(&planck(), 0.0).unwrap();
        assert!(
            (ratio - planck().omega_lambda).abs() < 0.01,
            "entropy_ratio(z=0) = {ratio}"
        );
    }

    #[test]
    fn test_entropy_ratio_monotonic_decreasing_with_z() {
        let p = planck();
        let r0 = entropy_ratio(&p, 0.0).unwrap();
        let r1 = entropy_ratio(&p, 1.0).unwrap();
        let r10 = entropy_ratio(&p, 10.0).unwrap();
        assert!(r0 > r1, "ratio should decrease with z: r0={r0}, r1={r1}");
        assert!(r1 > r10, "ratio should decrease with z: r1={r1}, r10={r10}");
    }

    #[test]
    fn test_entropy_ratio_bounds() {
        let p = planck();
        for z in [0.0, 0.5, 1.0, 5.0, 100.0, 1100.0] {
            let r = entropy_ratio(&p, z).unwrap();
            assert!(
                (0.0..=1.0).contains(&r),
                "entropy_ratio({z}) = {r} out of [0,1]"
            );
        }
    }

    #[test]
    fn test_manifestation_intensity_z0() {
        let p = planck();
        let intensity = manifestation_intensity(&p, 0.0).unwrap();
        // ≈ 1 - omega_lambda ≈ 0.315
        assert!(
            (intensity - (1.0 - p.omega_lambda)).abs() < 0.01,
            "intensity(z=0) = {intensity}"
        );
    }

    #[test]
    fn test_manifestation_intensity_increases_with_z() {
        let p = planck();
        let i0 = manifestation_intensity(&p, 0.0).unwrap();
        let i1 = manifestation_intensity(&p, 1.0).unwrap();
        let i10 = manifestation_intensity(&p, 10.0).unwrap();
        assert!(i1 > i0);
        assert!(i10 > i1);
    }

    #[test]
    fn test_unity_parameter_complement() {
        let p = planck();
        let intensity = manifestation_intensity(&p, 0.0).unwrap();
        let unity = unity_parameter(intensity).unwrap();
        assert!((intensity + unity - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_unity_parameter_rejects_out_of_range() {
        assert!(unity_parameter(1.5).is_err());
        assert!(unity_parameter(-0.1).is_err());
    }

    #[test]
    fn test_fixed_point_state_at_z0() {
        let state = FixedPointState::at_redshift(&planck(), 0.0).unwrap();
        assert_eq!(state.phase, CosmicPhase::DarkEnergyDominated);
        assert!((state.intensity + state.unity_param - 1.0).abs() < 1e-12);
        assert!((state.entropy_ratio + state.intensity - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_unphysical_z_rejected() {
        assert!(cosmic_phase(&planck(), -2.0).is_err());
        assert!(entropy_ratio(&planck(), -2.0).is_err());
    }
}
