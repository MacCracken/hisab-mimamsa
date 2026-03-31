//! Running coupling constants — β-functions, renormalization group flow, asymptotic freedom.
//!
//! Provides one-loop β-functions for QED and QCD, numerical (RK4) and analytic
//! running of coupling constants, and asymptotic freedom detection.
//!
//! All energy scales in GeV (natural units).

use std::f64::consts::PI;

use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::error::{ensure_finite, require_finite, MimamsaError};

/// QED one-loop β-function: β(α) = 2α²/(3π).
///
/// Positive β means the coupling *increases* with energy (QED is not
/// asymptotically free).
#[inline]
pub fn beta_qed_one_loop(alpha: f64) -> Result<f64, MimamsaError> {
    require_finite(alpha, "beta_qed_one_loop")?;
    ensure_finite(2.0 * alpha * alpha / (3.0 * PI), "beta_qed_one_loop")
}

/// QCD one-loop β-function: β(α_s) = -(33 - 2n_f) α_s² / (12π).
///
/// Negative β (for n_f < 17) means the coupling *decreases* with energy —
/// asymptotic freedom.
///
/// # Arguments
/// * `alpha_s` — Strong coupling constant at current scale.
/// * `n_f` — Number of active quark flavors at current scale (0–6 for SM).
#[inline]
pub fn beta_qcd_one_loop(alpha_s: f64, n_f: u8) -> Result<f64, MimamsaError> {
    require_finite(alpha_s, "beta_qcd_one_loop")?;
    let b0 = 33.0 - 2.0 * f64::from(n_f);
    ensure_finite(
        -b0 * alpha_s * alpha_s / (12.0 * PI),
        "beta_qcd_one_loop",
    )
}

/// Returns true if QCD is asymptotically free for the given number of flavors.
///
/// Asymptotic freedom requires 33 − 2n_f > 0, i.e. n_f < 17.
/// The Standard Model has n_f ≤ 6.
#[must_use]
#[inline]
pub fn is_asymptotically_free(n_f: u8) -> bool {
    n_f < 17
}

/// Running QED coupling α(μ) via RK4 integration of dα/d(ln μ) = β(α).
///
/// # Arguments
/// * `alpha_0` — Coupling at reference scale μ₀.
/// * `mu_0_gev` — Reference energy scale (GeV).
/// * `mu_gev` — Target energy scale (GeV).
/// * `n_steps` — Number of RK4 integration steps.
pub fn running_coupling_qed(
    alpha_0: f64,
    mu_0_gev: f64,
    mu_gev: f64,
    n_steps: usize,
) -> Result<f64, MimamsaError> {
    require_finite(alpha_0, "running_coupling_qed")?;
    require_finite(mu_0_gev, "running_coupling_qed")?;
    require_finite(mu_gev, "running_coupling_qed")?;
    if alpha_0 <= 0.0 || mu_0_gev <= 0.0 || mu_gev <= 0.0 {
        return Err(MimamsaError::Computation(
            "running_coupling_qed: alpha, mu_0, mu must be positive".to_string(),
        ));
    }
    if n_steps == 0 {
        return Ok(alpha_0);
    }

    let t0 = mu_0_gev.ln();
    let t_end = mu_gev.ln();

    let result = hisab::num::rk4(
        |_t, y, dy| {
            let a = y[0];
            dy[0] = 2.0 * a * a / (3.0 * PI);
        },
        t0,
        &[alpha_0],
        t_end,
        n_steps,
    )
    .map_err(|e| MimamsaError::Computation(format!("running_coupling_qed: RK4 failed: {e}")))?;

    ensure_finite(result[0], "running_coupling_qed")
}

/// Running QCD coupling α_s(μ) via RK4 integration.
///
/// # Arguments
/// * `alpha_s_0` — Strong coupling at reference scale.
/// * `mu_0_gev` — Reference energy scale (GeV).
/// * `mu_gev` — Target energy scale (GeV).
/// * `n_f` — Number of active quark flavors.
/// * `n_steps` — Number of RK4 integration steps.
pub fn running_coupling_qcd(
    alpha_s_0: f64,
    mu_0_gev: f64,
    mu_gev: f64,
    n_f: u8,
    n_steps: usize,
) -> Result<f64, MimamsaError> {
    require_finite(alpha_s_0, "running_coupling_qcd")?;
    require_finite(mu_0_gev, "running_coupling_qcd")?;
    require_finite(mu_gev, "running_coupling_qcd")?;
    if alpha_s_0 <= 0.0 || mu_0_gev <= 0.0 || mu_gev <= 0.0 {
        return Err(MimamsaError::Computation(
            "running_coupling_qcd: alpha_s, mu_0, mu must be positive".to_string(),
        ));
    }
    if n_steps == 0 {
        return Ok(alpha_s_0);
    }

    let b0 = 33.0 - 2.0 * f64::from(n_f);
    let t0 = mu_0_gev.ln();
    let t_end = mu_gev.ln();

    let result = hisab::num::rk4(
        |_t, y, dy| {
            let a = y[0];
            dy[0] = -b0 * a * a / (12.0 * PI);
        },
        t0,
        &[alpha_s_0],
        t_end,
        n_steps,
    )
    .map_err(|e| MimamsaError::Computation(format!("running_coupling_qcd: RK4 failed: {e}")))?;

    ensure_finite(result[0], "running_coupling_qcd")
}

/// One-loop analytic QED running: α(μ) = α₀ / (1 - 2α₀/(3π) · ln(μ/μ₀)).
///
/// Exact at one-loop; faster than numerical integration.
#[inline]
pub fn running_coupling_qed_analytic(
    alpha_0: f64,
    mu_0_gev: f64,
    mu_gev: f64,
) -> Result<f64, MimamsaError> {
    require_finite(alpha_0, "running_coupling_qed_analytic")?;
    require_finite(mu_0_gev, "running_coupling_qed_analytic")?;
    require_finite(mu_gev, "running_coupling_qed_analytic")?;
    if alpha_0 <= 0.0 || mu_0_gev <= 0.0 || mu_gev <= 0.0 {
        return Err(MimamsaError::Computation(
            "running_coupling_qed_analytic: alpha, mu_0, mu must be positive".to_string(),
        ));
    }
    let log_ratio = (mu_gev / mu_0_gev).ln();
    let denom = 1.0 - 2.0 * alpha_0 / (3.0 * PI) * log_ratio;
    if denom <= 0.0 {
        warn!(alpha_0, mu_0_gev, mu_gev, denom, "QED Landau pole encountered");
        return Err(MimamsaError::Divergence {
            context: "running_coupling_qed_analytic".to_string(),
            detail: "Landau pole encountered".to_string(),
        });
    }
    ensure_finite(alpha_0 / denom, "running_coupling_qed_analytic")
}

/// One-loop analytic QCD running: α_s(μ) = α_s₀ / (1 + b₀α_s₀·ln(μ/μ₀)/(2π)).
///
/// where b₀ = (33 - 2n_f)/6.
#[inline]
pub fn running_coupling_qcd_analytic(
    alpha_s_0: f64,
    mu_0_gev: f64,
    mu_gev: f64,
    n_f: u8,
) -> Result<f64, MimamsaError> {
    require_finite(alpha_s_0, "running_coupling_qcd_analytic")?;
    require_finite(mu_0_gev, "running_coupling_qcd_analytic")?;
    require_finite(mu_gev, "running_coupling_qcd_analytic")?;
    if alpha_s_0 <= 0.0 || mu_0_gev <= 0.0 || mu_gev <= 0.0 {
        return Err(MimamsaError::Computation(
            "running_coupling_qcd_analytic: alpha_s, mu_0, mu must be positive".to_string(),
        ));
    }
    let b0 = (33.0 - 2.0 * f64::from(n_f)) / 6.0;
    let log_ratio = (mu_gev / mu_0_gev).ln();
    let denom = 1.0 + b0 * alpha_s_0 * log_ratio / (2.0 * PI);
    if denom <= 0.0 {
        warn!(alpha_s_0, mu_0_gev, mu_gev, n_f, denom, "QCD infrared Landau pole encountered");
        return Err(MimamsaError::Divergence {
            context: "running_coupling_qcd_analytic".to_string(),
            detail: "infrared Landau pole encountered".to_string(),
        });
    }
    ensure_finite(alpha_s_0 / denom, "running_coupling_qcd_analytic")
}

/// Result bundle for coupling constant analysis at a given scale.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CouplingAnalysis {
    /// Coupling constant at the target scale.
    pub alpha_at_scale: f64,
    /// β-function value at the target scale.
    pub beta_value: f64,
    /// Energy scale (GeV).
    pub scale_gev: f64,
    /// Whether the theory is asymptotically free.
    pub is_asymptotically_free: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{ALPHA, ALPHA_S_MZ, M_Z_GEV};

    #[test]
    fn test_beta_qed_positive() {
        let b = beta_qed_one_loop(ALPHA).unwrap();
        assert!(b > 0.0, "QED beta should be positive");
    }

    #[test]
    fn test_beta_qcd_negative() {
        // n_f = 6 (all SM quarks): β < 0
        let b = beta_qcd_one_loop(ALPHA_S_MZ, 6).unwrap();
        assert!(b < 0.0, "QCD beta should be negative for n_f=6");
    }

    #[test]
    fn test_asymptotic_freedom() {
        assert!(is_asymptotically_free(6)); // SM
        assert!(is_asymptotically_free(16)); // barely
        assert!(!is_asymptotically_free(17)); // lost
    }

    #[test]
    fn test_qed_coupling_increases() {
        let a_low = running_coupling_qed_analytic(ALPHA, M_Z_GEV, 200.0).unwrap();
        assert!(a_low > ALPHA, "QED coupling should increase with energy");
    }

    #[test]
    fn test_qcd_coupling_decreases() {
        let a_high = running_coupling_qcd_analytic(ALPHA_S_MZ, M_Z_GEV, 1000.0, 6).unwrap();
        assert!(
            a_high < ALPHA_S_MZ,
            "QCD coupling should decrease with energy"
        );
    }

    #[test]
    fn test_analytic_vs_numerical_qed() {
        let a_analytic = running_coupling_qed_analytic(ALPHA, M_Z_GEV, 200.0).unwrap();
        let a_numerical = running_coupling_qed(ALPHA, M_Z_GEV, 200.0, 10000).unwrap();
        let rel_diff = (a_analytic - a_numerical).abs() / a_analytic;
        assert!(
            rel_diff < 0.01,
            "analytic vs numerical QED: {rel_diff:.4e}"
        );
    }

    #[test]
    fn test_analytic_vs_numerical_qcd() {
        let a_analytic =
            running_coupling_qcd_analytic(ALPHA_S_MZ, M_Z_GEV, 1000.0, 6).unwrap();
        let a_numerical =
            running_coupling_qcd(ALPHA_S_MZ, M_Z_GEV, 1000.0, 6, 10000).unwrap();
        let rel_diff = (a_analytic - a_numerical).abs() / a_analytic;
        assert!(
            rel_diff < 0.01,
            "analytic vs numerical QCD: {rel_diff:.4e}"
        );
    }

    #[test]
    fn test_no_evolution_at_same_scale() {
        let a = running_coupling_qed(ALPHA, M_Z_GEV, M_Z_GEV, 1000).unwrap();
        assert!((a - ALPHA).abs() / ALPHA < 1e-6);
    }

    #[test]
    fn test_zero_steps_returns_initial() {
        let a = running_coupling_qed(ALPHA, M_Z_GEV, 200.0, 0).unwrap();
        assert!((a - ALPHA).abs() < 1e-15);
    }

    #[test]
    fn test_negative_alpha_rejected() {
        assert!(running_coupling_qed(-1.0, M_Z_GEV, 200.0, 100).is_err());
    }

    #[test]
    fn test_coupling_analysis_serde() {
        let ca = CouplingAnalysis {
            alpha_at_scale: ALPHA,
            beta_value: 0.001,
            scale_gev: M_Z_GEV,
            is_asymptotically_free: false,
        };
        let json = serde_json::to_string(&ca).unwrap();
        let _back: CouplingAnalysis = serde_json::from_str(&json).unwrap();
    }
}
