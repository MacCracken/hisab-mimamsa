use hisab_mimamsa::relativity::black_hole::*;
use hisab_mimamsa::relativity::geodesic::*;
use hisab_mimamsa::relativity::lorentz::*;
use hisab_mimamsa::relativity::metric::*;

const M_SUN: f64 = 1.989e30;

#[test]
fn test_sr_and_gr_consistency() {
    // At large r, gravitational time dilation → 1 (flat space → SR regime)
    let factor = gravitational_time_dilation(M_SUN, 1e15).unwrap();
    assert!((factor - 1.0).abs() < 1e-10);

    // At v=0, Lorentz factor → 1
    let gamma = lorentz_factor(0.0).unwrap();
    assert!((gamma - 1.0).abs() < 1e-15);
}

#[test]
fn test_schwarzschild_properties_consistent() {
    let rs = schwarzschild_radius(M_SUN).unwrap();
    let rph = photon_sphere_radius(M_SUN).unwrap();
    let risco = schwarzschild_isco(M_SUN).unwrap();

    // r_photon < r_isco (photon sphere inside ISCO)
    assert!(rph < risco);
    // Both outside event horizon
    assert!(rph > rs);
    assert!(risco > rs);
}

#[test]
fn test_black_hole_thermodynamics_laws() {
    // Second law: entropy increases with mass
    let s1 = bekenstein_hawking_entropy(M_SUN).unwrap();
    let s2 = bekenstein_hawking_entropy(2.0 * M_SUN).unwrap();
    assert!(s2 > s1);

    // Third law: temperature → 0 as mass → ∞
    let t1 = hawking_temperature(M_SUN).unwrap();
    let t2 = hawking_temperature(1e6 * M_SUN).unwrap();
    assert!(t2 < t1);
}

#[test]
fn test_eddington_deflection_doubles_weak_field() {
    // Doubling the mass doubles the deflection (weak field, linear)
    let r = 6.957e8;
    let d1 = light_deflection_weak_field(M_SUN, r).unwrap();
    let d2 = light_deflection_weak_field(2.0 * M_SUN, r).unwrap();
    assert!((d2 / d1 - 2.0).abs() < 1e-10);
}
