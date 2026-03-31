//! Quantum field theory — propagators, running couplings, Casimir effect.

use hisab_mimamsa::constants::{ALPHA, ALPHA_S_MZ, M_Z_GEV};
use hisab_mimamsa::quantum_field::{FourMomentum, coupling, propagator, vacuum};

fn main() {
    // Scalar propagator: on-shell vs off-shell
    let m_e = 0.511e-3; // electron mass in GeV
    let p_on = FourMomentum::new(m_e, 0.0, 0.0, 0.0).unwrap();
    let p_off = FourMomentum::new(100.0, 0.0, 0.0, 0.0).unwrap();
    let d_on = propagator::scalar_propagator(&p_on, m_e, propagator::DEFAULT_EPSILON).unwrap();
    let d_off = propagator::scalar_propagator(&p_off, m_e, propagator::DEFAULT_EPSILON).unwrap();
    println!("Scalar propagator (electron):");
    println!("  On-shell  |Δ| = {:.2e}", d_on.abs());
    println!("  Off-shell |Δ| = {:.2e}", d_off.abs());

    // Running couplings
    println!("\nRunning coupling constants:");
    println!("  α(M_Z)   = {ALPHA:.6}");
    println!("  α_s(M_Z) = {ALPHA_S_MZ:.4}");
    for &mu in &[200.0, 500.0, 1000.0] {
        let a_qed = coupling::running_coupling_qed_analytic(ALPHA, M_Z_GEV, mu).unwrap();
        let a_qcd = coupling::running_coupling_qcd_analytic(ALPHA_S_MZ, M_Z_GEV, mu, 6).unwrap();
        println!("  α({mu:>6} GeV) = {a_qed:.6}  α_s = {a_qcd:.4}");
    }

    // Casimir effect
    let d_um = 1e-6;
    let f = vacuum::casimir_force_per_area(d_um).unwrap();
    println!("\nCasimir force at {d_um:.0e} m separation: {f:.4e} N/m²");
}
