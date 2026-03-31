//! Unified field — holographic entropy, manifestation intensity, bhava bridge.

use hisab_mimamsa::cosmology::friedmann::CosmologicalParameters;
use hisab_mimamsa::unified::{fixed_point, holographic, scale_bridge};

fn main() {
    let params = CosmologicalParameters::planck2018();

    // Black hole information content
    let m_sun = 1.989e30;
    let bits = holographic::black_hole_information_bits(m_sun).unwrap();
    println!("Solar mass BH information: {bits:.2e} bits");

    // Cosmological horizon entropy
    let h0_si = params.h0 / 3.085_677_581e19;
    let s_horizon = holographic::cosmological_horizon_entropy(h0_si).unwrap();
    println!("Cosmological horizon entropy: {s_horizon:.2e} J/K");

    // Cosmic phase at various epochs
    println!("\nCosmic phases:");
    for &(z, label) in &[(10000.0, "z=10000"), (10.0, "z=10"), (0.0, "now")] {
        let phase = fixed_point::cosmic_phase(&params, z).unwrap();
        let intensity = fixed_point::manifestation_intensity(&params, z).unwrap();
        println!("  {label:>8}: {phase:?}, intensity = {intensity:.4}");
    }

    // bhava bridge output
    println!("\nbhava Scale 6/7 bridge (now):");
    let bridge = scale_bridge::BridgeOutput::at_redshift(&params, 0.0).unwrap();
    println!("  Intensity:        {:.4}", bridge.intensity);
    println!("  Unity parameter:  {:.4}", bridge.unity_param);
    println!("  Phase:            {:.4}", bridge.phase);
    println!("  Convergence rate: {:.4e}", bridge.convergence_rate);
}
