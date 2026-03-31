//! Cosmic expansion — scale factor, distances, horizons.

use super::friedmann::{CosmologicalParameters, MPC_IN_KM, hubble_parameter};

/// Comoving distance to redshift z (meters).
/// d_C = c ∫₀ᶻ dz'/H(z')
#[must_use]
pub fn comoving_distance(params: &CosmologicalParameters, z: f64, n: usize) -> f64 {
    let c = crate::constants::C;
    let dz = z / n as f64;
    let mut integral = 0.0;

    for i in 0..n {
        let z0 = i as f64 * dz;
        let z1 = (i + 1) as f64 * dz;
        let f0 = 1.0 / hubble_parameter(params, z0);
        let f1 = 1.0 / hubble_parameter(params, z1);
        integral += 0.5 * (f0 + f1) * dz;
    }

    c * integral
}

/// Luminosity distance: d_L = (1+z) * d_C.
#[must_use]
pub fn luminosity_distance(params: &CosmologicalParameters, z: f64, n: usize) -> f64 {
    (1.0 + z) * comoving_distance(params, z, n)
}

/// Angular diameter distance: d_A = d_C / (1+z).
#[must_use]
pub fn angular_diameter_distance(params: &CosmologicalParameters, z: f64, n: usize) -> f64 {
    comoving_distance(params, z, n) / (1.0 + z)
}

/// Lookback time to redshift z (seconds).
/// t_lb = ∫₀ᶻ dz'/((1+z')H(z'))
#[must_use]
pub fn lookback_time(params: &CosmologicalParameters, z: f64, n: usize) -> f64 {
    let dz = z / n as f64;
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

/// Hubble distance: d_H = c/H₀.
#[must_use]
#[inline]
pub fn hubble_distance(params: &CosmologicalParameters) -> f64 {
    let c = crate::constants::C;
    let h0_si = params.h0 / MPC_IN_KM;
    c / h0_si
}

/// Scale factor a(z) = 1/(1+z).
#[must_use]
#[inline]
pub fn scale_factor(z: f64) -> f64 {
    1.0 / (1.0 + z)
}

/// Redshift from scale factor: z = 1/a - 1.
#[must_use]
#[inline]
pub fn redshift_from_scale_factor(a: f64) -> f64 {
    1.0 / a - 1.0
}

/// CMB temperature at redshift z: T(z) = T₀(1+z).
/// T₀ = 2.7255 K (COBE/FIRAS).
#[must_use]
#[inline]
pub fn cmb_temperature(z: f64) -> f64 {
    2.7255 * (1.0 + z)
}

#[cfg(test)]
mod tests {
    use super::*;

    const GYR_S: f64 = 365.25 * 24.0 * 3600.0 * 1e9;
    const MPC_M: f64 = 3.085_677_581e22;
    /// 1 Gpc in meters (1 Gpc = 1e3 Mpc).
    const GPC_M: f64 = 1e3 * MPC_M;

    #[test]
    fn test_scale_factor_now() {
        assert!((scale_factor(0.0) - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_scale_factor_cmb() {
        // CMB at z ≈ 1100: a ≈ 9.1e-4
        let a = scale_factor(1100.0);
        assert!(a > 8e-4 && a < 1e-3);
    }

    #[test]
    fn test_cmb_temperature_now() {
        assert!((cmb_temperature(0.0) - 2.7255).abs() < 1e-4);
    }

    #[test]
    fn test_cmb_temperature_decoupling() {
        // At z ≈ 1100: T ≈ 3000 K
        let t = cmb_temperature(1100.0);
        assert!(t > 2900.0 && t < 3100.0);
    }

    #[test]
    fn test_comoving_distance_z1() {
        let params = CosmologicalParameters::planck2018();
        let d = comoving_distance(&params, 1.0, 1000);
        let d_gpc = d / (GPC_M);
        // z=1: ~3.3 Gpc comoving
        assert!(d_gpc > 3.0 && d_gpc < 3.6);
    }

    #[test]
    fn test_angular_diameter_distance_turnover() {
        // Angular diameter distance has a maximum around z ≈ 1.6
        let params = CosmologicalParameters::planck2018();
        let d1 = angular_diameter_distance(&params, 1.0, 1000);
        let d2 = angular_diameter_distance(&params, 1.6, 1000);
        let d3 = angular_diameter_distance(&params, 3.0, 1000);
        // d2 near maximum: exceeds both d1 (pre-peak) and d3 (post-peak)
        assert!(d2 > d1);
        assert!(d2 > d3);
    }

    #[test]
    fn test_hubble_distance() {
        let params = CosmologicalParameters::planck2018();
        let dh = hubble_distance(&params);
        let dh_gpc = dh / (GPC_M);
        // d_H ≈ 4.4 Gpc
        assert!(dh_gpc > 4.0 && dh_gpc < 5.0);
    }

    #[test]
    fn test_lookback_time_z1() {
        let params = CosmologicalParameters::planck2018();
        let t = lookback_time(&params, 1.0, 1000);
        let t_gyr = t / GYR_S;
        // z=1: lookback ≈ 7.9 Gyr
        assert!(t_gyr > 7.0 && t_gyr < 8.5);
    }

    #[test]
    fn test_roundtrip_scale_factor_redshift() {
        let z = 2.5;
        let a = scale_factor(z);
        let z2 = redshift_from_scale_factor(a);
        assert!((z - z2).abs() < 1e-12);
    }
}
