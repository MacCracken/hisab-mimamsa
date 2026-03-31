//! Friedmann equations — the dynamical equations of an expanding universe.

use serde::{Deserialize, Serialize};

/// Hubble constant H₀ (km/s/Mpc → s⁻¹ for internal use).
pub const H0_KM_S_MPC: f64 = 67.4; // Planck 2018
pub const MPC_IN_KM: f64 = 3.085_677_581e19;
/// H₀ in s⁻¹.
pub const H0: f64 = H0_KM_S_MPC / MPC_IN_KM;

/// ΛCDM cosmological parameters (Planck 2018 best fit).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CosmologicalParameters {
    /// Hubble constant (km/s/Mpc).
    pub h0: f64,
    /// Matter density parameter (baryonic + dark).
    pub omega_m: f64,
    /// Radiation density parameter.
    pub omega_r: f64,
    /// Dark energy density parameter (cosmological constant).
    pub omega_lambda: f64,
    /// Curvature density parameter (1 - Ω_m - Ω_r - Ω_Λ).
    pub omega_k: f64,
}

impl CosmologicalParameters {
    /// Planck 2018 ΛCDM best fit.
    #[must_use]
    pub fn planck2018() -> Self {
        let omega_m = 0.315;
        let omega_r = 9.15e-5;
        // Derive Ω_Λ so density parameters sum exactly to 1 (flat universe).
        let omega_lambda = 1.0 - omega_m - omega_r;
        Self {
            h0: 67.4,
            omega_m,
            omega_r,
            omega_lambda,
            omega_k: 0.0,
        }
    }

    /// Verify density parameters sum to 1 (flat) or compute curvature.
    #[must_use]
    pub fn is_flat(&self) -> bool {
        (self.omega_m + self.omega_r + self.omega_lambda + self.omega_k - 1.0).abs() < 1e-6
    }
}

impl Default for CosmologicalParameters {
    fn default() -> Self {
        Self::planck2018()
    }
}

/// Hubble parameter H(z) as a function of redshift z.
/// H(z) = H₀ √(Ω_r(1+z)⁴ + Ω_m(1+z)³ + Ω_k(1+z)² + Ω_Λ)
#[must_use]
#[inline]
pub fn hubble_parameter(params: &CosmologicalParameters, z: f64) -> f64 {
    let z1 = 1.0 + z;
    let z2 = z1 * z1;
    let z3 = z2 * z1;
    let z4 = z3 * z1;

    let h0_si = params.h0 / MPC_IN_KM;
    h0_si
        * (params.omega_r * z4 + params.omega_m * z3 + params.omega_k * z2 + params.omega_lambda)
            .sqrt()
}

/// Critical density: ρ_c = 3H²/(8πG).
#[must_use]
#[inline]
pub fn critical_density(h: f64) -> f64 {
    3.0 * h * h / (8.0 * std::f64::consts::PI * crate::constants::G)
}

/// Deceleration parameter q(z) = -1 - Ḣ/H².
/// At z=0: q₀ = Ω_m/2 + Ω_r - Ω_Λ.
#[must_use]
#[inline]
pub fn deceleration_parameter_now(params: &CosmologicalParameters) -> f64 {
    params.omega_m / 2.0 + params.omega_r - params.omega_lambda
}

/// Age of universe via numerical integration of 1/((1+z)H(z)).
/// Uses simple trapezoidal rule with n steps from z=0 to z_max.
#[must_use]
pub fn age_of_universe(params: &CosmologicalParameters, z_max: f64, n: usize) -> f64 {
    let dz = z_max / n as f64;
    let mut integral = 0.0;

    for i in 0..n {
        let z0 = i as f64 * dz;
        let z1 = (i + 1) as f64 * dz;
        let f0 = 1.0 / ((1.0 + z0) * hubble_parameter(params, z0));
        let f1 = 1.0 / ((1.0 + z1) * hubble_parameter(params, z1));
        integral += 0.5 * (f0 + f1) * dz;
    }

    integral
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planck_is_flat() {
        assert!(CosmologicalParameters::planck2018().is_flat());
    }

    #[test]
    fn test_hubble_at_z0() {
        let params = CosmologicalParameters::planck2018();
        let h = hubble_parameter(&params, 0.0);
        let h0_si = params.h0 / MPC_IN_KM;
        // At z=0, H(0) should ≈ H₀ (within numerical precision of density sum)
        assert!((h - h0_si).abs() / h0_si < 0.01);
    }

    #[test]
    fn test_hubble_increases_with_z() {
        let params = CosmologicalParameters::planck2018();
        let h0 = hubble_parameter(&params, 0.0);
        let h1 = hubble_parameter(&params, 1.0);
        let h10 = hubble_parameter(&params, 10.0);
        assert!(h1 > h0);
        assert!(h10 > h1);
    }

    #[test]
    fn test_deceleration_parameter() {
        let params = CosmologicalParameters::planck2018();
        let q = deceleration_parameter_now(&params);
        // q₀ < 0 means accelerating expansion (dark energy dominated)
        assert!(q < 0.0);
    }

    #[test]
    fn test_age_of_universe() {
        let params = CosmologicalParameters::planck2018();
        let age_s = age_of_universe(&params, 1100.0, 10000);
        let age_gyr = age_s / (365.25 * 24.0 * 3600.0 * 1e9);
        // Should be ~13.8 Gyr
        assert!(age_gyr > 13.0 && age_gyr < 14.5);
    }

    #[test]
    fn test_critical_density_today() {
        let h0_si = H0_KM_S_MPC / MPC_IN_KM;
        let rho_c = critical_density(h0_si);
        // ρ_c ≈ 9.47e-27 kg/m³
        assert!(rho_c > 8e-27 && rho_c < 1.1e-26);
    }
}
