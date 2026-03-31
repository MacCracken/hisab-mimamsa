//! Special and general relativity — time dilation, black holes, lensing.

use hisab_mimamsa::relativity::{black_hole, lorentz, metric};

fn main() {
    // Muon time dilation at 0.994c
    let gamma = lorentz::lorentz_factor(0.994 * lorentz::C).unwrap();
    let observed = lorentz::time_dilation(2.2e-6, 0.994 * lorentz::C).unwrap();
    println!("Muon at 0.994c: γ = {gamma:.2}, observed lifetime = {observed:.2e} s");

    // Schwarzschild black hole (solar mass)
    let m_sun = 1.989e30;
    let props = black_hole::BlackHoleProperties::from_mass(m_sun).unwrap();
    println!("\nSolar mass black hole:");
    println!("  Schwarzschild radius: {:.0} m", props.radius_m);
    println!("  Hawking temperature:  {:.2e} K", props.temperature_k);
    println!("  Evaporation time:     {:.2e} s", props.evaporation_time_s);

    // Gravitational time dilation at Earth's surface
    let factor = metric::gravitational_time_dilation(5.972e24, 6.371e6).unwrap();
    println!(
        "\nGravitational time dilation at Earth surface: {:.10}",
        factor
    );
}
