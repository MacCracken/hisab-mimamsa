//! Scale bridge — multi-scale structure, RG wrappers, bhava bridge functions.
//!
//! Connects renormalization group flow (scale-dependent couplings) with the
//! cosmological fixed-point analysis to provide bridge functions for bhava's
//! consciousness model at Scales 6 and 7.
//!
//! All energy scales in GeV (natural units).

use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::constants::{ALPHA, ALPHA_S_MZ, M_Z_GEV};
use crate::cosmology::friedmann::CosmologicalParameters;
use crate::error::{ensure_finite, require_finite, MimamsaError};
use crate::quantum_field::coupling::{
    running_coupling_qcd_analytic, running_coupling_qed_analytic,
};

use super::fixed_point::manifestation_intensity;

/// Output bundle for a bhava bridge computation at a given redshift.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BridgeOutput {
    /// Manifestation intensity ∈ [0, 1] (Scale 6).
    pub intensity: f64,
    /// Cosmic breath phase ∈ [0, 1] (Scale 7).
    pub phase: f64,
    /// Unity parameter = 1 − intensity.
    pub unity_param: f64,
    /// Rate of convergence toward fixed point (|dI/dz|, approximate).
    pub convergence_rate: f64,
}

impl BridgeOutput {
    /// Compute the full bridge output at redshift z.
    ///
    /// Convergence rate is estimated by finite difference with Δz = 0.01.
    pub fn at_redshift(
        params: &CosmologicalParameters,
        z: f64,
    ) -> Result<Self, MimamsaError> {
        let intensity = bridge_scale_6(params, z)?;
        let phase = bridge_scale_7(params, z)?;
        let unity_param = 1.0 - intensity;

        let dz = 0.01;
        let intensity_plus = bridge_scale_6(params, z + dz)?;
        let convergence_rate =
            ensure_finite(((intensity - intensity_plus) / dz).abs(), "BridgeOutput::convergence_rate")?;

        Ok(Self {
            intensity,
            phase,
            unity_param,
            convergence_rate,
        })
    }
}

/// QED running coupling at energy scale μ (GeV).
///
/// Convenience wrapper: runs α from [`ALPHA`] at [`M_Z_GEV`] to μ using
/// the one-loop analytic formula.
#[inline]
pub fn scale_coupling_qed(mu_gev: f64) -> Result<f64, MimamsaError> {
    require_finite(mu_gev, "scale_coupling_qed")?;
    if mu_gev <= 0.0 {
        warn!(mu_gev, "scale_coupling_qed: scale must be positive");
        return Err(MimamsaError::Computation(
            "scale_coupling_qed: scale must be positive".to_string(),
        ));
    }
    running_coupling_qed_analytic(ALPHA, M_Z_GEV, mu_gev)
}

/// QCD running coupling at energy scale μ (GeV) with n_f active flavors.
///
/// Convenience wrapper: runs α_s from [`ALPHA_S_MZ`] at [`M_Z_GEV`] to μ.
#[inline]
pub fn scale_coupling_qcd(mu_gev: f64, n_f: u8) -> Result<f64, MimamsaError> {
    require_finite(mu_gev, "scale_coupling_qcd")?;
    if mu_gev <= 0.0 {
        warn!(mu_gev, "scale_coupling_qcd: scale must be positive");
        return Err(MimamsaError::Computation(
            "scale_coupling_qcd: scale must be positive".to_string(),
        ));
    }
    running_coupling_qcd_analytic(ALPHA_S_MZ, M_Z_GEV, mu_gev, n_f)
}

/// bhava Scale 6 bridge: cosmic expansion → manifestation intensity.
///
/// Returns the manifestation intensity at redshift z ∈ [0, 1], serving as
/// the scalar input to bhava's Scale 6 personality field computations.
///
/// Maps the cosmos's thermodynamic distance from equilibrium into a scalar
/// that modulates all manifestation at that epoch.
pub fn bridge_scale_6(
    params: &CosmologicalParameters,
    z: f64,
) -> Result<f64, MimamsaError> {
    manifestation_intensity(params, z)
}

/// bhava Scale 7 bridge: cosmic breath phase (unity/differentiation cycle).
///
/// Returns a phase ∈ [0, 1]:
/// - 0.0 = pure unity (heat death / maximum entropy)
/// - 1.0 = maximum differentiation (Big Bang / minimum entropy)
///
/// For ΛCDM (monotonic expansion) this equals Scale 6. In future cyclic
/// cosmology extensions, Scale 7 would incorporate the oscillation phase
/// of successive cosmic cycles.
pub fn bridge_scale_7(
    params: &CosmologicalParameters,
    z: f64,
) -> Result<f64, MimamsaError> {
    // For ΛCDM, phase = intensity. Separated for future cyclic extensions.
    manifestation_intensity(params, z)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn planck() -> CosmologicalParameters {
        CosmologicalParameters::planck2018()
    }

    #[test]
    fn test_scale_coupling_qed_at_mz() {
        let a = scale_coupling_qed(M_Z_GEV).unwrap();
        assert!((a - ALPHA).abs() / ALPHA < 1e-6);
    }

    #[test]
    fn test_scale_coupling_qed_increases() {
        let a_200 = scale_coupling_qed(200.0).unwrap();
        assert!(a_200 > ALPHA, "QED coupling should increase with energy");
    }

    #[test]
    fn test_scale_coupling_qcd_at_mz() {
        let a = scale_coupling_qcd(M_Z_GEV, 6).unwrap();
        assert!((a - ALPHA_S_MZ).abs() / ALPHA_S_MZ < 1e-6);
    }

    #[test]
    fn test_scale_coupling_qcd_decreases() {
        let a_1000 = scale_coupling_qcd(1000.0, 6).unwrap();
        assert!(
            a_1000 < ALPHA_S_MZ,
            "QCD coupling should decrease with energy"
        );
    }

    #[test]
    fn test_bridge_scale_6_bounds() {
        let p = planck();
        for z in [0.0, 0.5, 1.0, 5.0, 100.0] {
            let i = bridge_scale_6(&p, z).unwrap();
            assert!(
                (0.0..=1.0).contains(&i),
                "bridge_scale_6({z}) = {i} out of [0,1]"
            );
        }
    }

    #[test]
    fn test_bridge_scale_7_bounds() {
        let p = planck();
        for z in [0.0, 0.5, 1.0, 5.0, 100.0] {
            let phase = bridge_scale_7(&p, z).unwrap();
            assert!(
                (0.0..=1.0).contains(&phase),
                "bridge_scale_7({z}) = {phase} out of [0,1]"
            );
        }
    }

    #[test]
    fn test_bridge_output_consistency() {
        let out = BridgeOutput::at_redshift(&planck(), 0.0).unwrap();
        assert!((out.intensity + out.unity_param - 1.0).abs() < 1e-12);
        assert!(out.convergence_rate >= 0.0);
    }

    #[test]
    fn test_bridge_output_serde() {
        let out = BridgeOutput::at_redshift(&planck(), 0.5).unwrap();
        let json = serde_json::to_string(&out).unwrap();
        let _back: BridgeOutput = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_negative_scale_rejected() {
        assert!(scale_coupling_qed(-1.0).is_err());
        assert!(scale_coupling_qcd(-1.0, 6).is_err());
    }
}
