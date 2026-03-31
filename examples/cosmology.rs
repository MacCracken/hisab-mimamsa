//! Cosmology — Friedmann equations, expansion history, CMB.

use hisab_mimamsa::cosmology::{expansion, friedmann};

fn main() {
    let params = friedmann::CosmologicalParameters::planck2018();

    // Age of the universe
    let age_s = friedmann::age_of_universe(&params, 1100.0, 10000).unwrap();
    let age_gyr = age_s / (365.25 * 24.0 * 3600.0 * 1e9);
    println!("Age of the universe: {age_gyr:.1} Gyr");

    // Deceleration parameter
    let q0 = friedmann::deceleration_parameter_now(&params).unwrap();
    println!("Deceleration parameter q₀: {q0:.3} (< 0 → accelerating)");

    // CMB temperature
    println!(
        "CMB temperature today: {:.4} K",
        expansion::cmb_temperature(0.0).unwrap()
    );
    println!(
        "CMB at decoupling (z=1100): {:.0} K",
        expansion::cmb_temperature(1100.0).unwrap()
    );

    // Distances at z=1
    let d_c = expansion::comoving_distance(&params, 1.0, 1000).unwrap();
    let d_l = expansion::luminosity_distance(&params, 1.0, 1000).unwrap();
    let gpc = 3.085_677_581e25;
    println!("\nDistances to z=1:");
    println!("  Comoving:   {:.2} Gpc", d_c / gpc);
    println!("  Luminosity: {:.2} Gpc", d_l / gpc);
}
