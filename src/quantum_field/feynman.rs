//! Feynman diagrams — tree-level amplitudes, Mandelstam variables, cross-sections.
//!
//! Provides types for representing Feynman diagrams at tree level and functions
//! for computing amplitudes, Mandelstam invariants, and differential/total
//! cross-sections for 2→2 scattering.
//!
//! All energies/momenta in GeV (natural units, ℏ = c = 1).

use std::f64::consts::PI;

use hisab::Complex;
use serde::{Deserialize, Serialize};

use tracing::warn;

use crate::error::{MimamsaError, ensure_finite, ensure_finite_complex, require_finite};

use super::FourMomentum;
use super::propagator;

/// Classification of particles in a Feynman diagram.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ParticleType {
    /// Spin-0 scalar.
    Scalar,
    /// Spin-½ fermion.
    Fermion,
    /// Spin-1 photon (massless gauge boson).
    Photon,
}

/// A vertex in a Feynman diagram.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vertex {
    /// Coupling constant at this vertex.
    pub coupling: f64,
    /// Particle types meeting at this vertex.
    pub particles: Vec<ParticleType>,
}

/// An internal line (propagator) connecting two vertices.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalLine {
    /// Type of particle on this line.
    pub particle_type: ParticleType,
    /// Four-momentum flowing through the line.
    pub momentum: FourMomentum,
    /// Mass in GeV (0 for photons).
    pub mass_gev: f64,
}

/// An external line (incoming or outgoing particle).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalLine {
    /// Type of particle.
    pub particle_type: ParticleType,
    /// Four-momentum.
    pub momentum: FourMomentum,
    /// True if incoming, false if outgoing.
    pub incoming: bool,
}

/// A tree-level Feynman diagram (no loops).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeDiagram {
    /// Interaction vertices.
    pub vertices: Vec<Vertex>,
    /// Internal propagator lines.
    pub internal_lines: Vec<InternalLine>,
    /// External particle lines.
    pub external_lines: Vec<ExternalLine>,
}

impl TreeDiagram {
    /// Check four-momentum conservation: Σ p_in = Σ p_out.
    ///
    /// Returns true if conservation holds within numerical tolerance.
    pub fn check_momentum_conservation(&self) -> Result<bool, MimamsaError> {
        let mut sum_e = 0.0;
        let mut sum_px = 0.0;
        let mut sum_py = 0.0;
        let mut sum_pz = 0.0;

        for line in &self.external_lines {
            let sign = if line.incoming { 1.0 } else { -1.0 };
            sum_e += sign * line.momentum.e;
            sum_px += sign * line.momentum.px;
            sum_py += sign * line.momentum.py;
            sum_pz += sign * line.momentum.pz;
        }

        let tol = 1e-8;
        Ok(sum_e.abs() < tol && sum_px.abs() < tol && sum_py.abs() < tol && sum_pz.abs() < tol)
    }
}

/// Evaluate tree-level amplitude for a diagram.
///
/// M = (∏ vertex couplings) × (∏ propagators for internal lines).
///
/// This is a simplified evaluator: it multiplies all vertex coupling factors
/// and all internal propagators (scalar factors). For full spinor/tensor
/// structure, the caller must handle gamma matrices and polarization sums.
pub fn tree_level_amplitude(diagram: &TreeDiagram) -> Result<Complex, MimamsaError> {
    let mut amplitude = Complex::new(1.0, 0.0);

    // Multiply vertex coupling factors
    for vertex in &diagram.vertices {
        require_finite(vertex.coupling, "tree_level_amplitude")?;
        amplitude *= vertex.coupling;
    }

    // Multiply propagators for each internal line
    for line in &diagram.internal_lines {
        let prop = match line.particle_type {
            ParticleType::Scalar | ParticleType::Fermion => propagator::scalar_propagator(
                &line.momentum,
                line.mass_gev,
                propagator::DEFAULT_EPSILON,
            )?,
            ParticleType::Photon => {
                propagator::gauge_boson_propagator(&line.momentum, propagator::DEFAULT_EPSILON)?
            }
        };
        amplitude *= prop;
    }

    ensure_finite_complex(amplitude, "tree_level_amplitude")
}

/// Mandelstam variable s = (p₁ + p₂)².
#[inline]
pub fn mandelstam_s(p1: &FourMomentum, p2: &FourMomentum) -> Result<f64, MimamsaError> {
    let total = p1.add(p2)?;
    total.invariant_mass_sq()
}

/// Mandelstam variable t = (p₁ - p₃)².
#[inline]
pub fn mandelstam_t(p1: &FourMomentum, p3: &FourMomentum) -> Result<f64, MimamsaError> {
    let diff = p1.sub(p3)?;
    diff.invariant_mass_sq()
}

/// Mandelstam variable u = (p₁ - p₄)².
#[inline]
pub fn mandelstam_u(p1: &FourMomentum, p4: &FourMomentum) -> Result<f64, MimamsaError> {
    let diff = p1.sub(p4)?;
    diff.invariant_mass_sq()
}

/// Verify the Mandelstam identity: s + t + u = Σ mᵢ².
///
/// Returns true if the identity holds within numerical tolerance.
pub fn verify_mandelstam_identity(
    s: f64,
    t: f64,
    u: f64,
    masses_gev: &[f64],
) -> Result<bool, MimamsaError> {
    require_finite(s, "verify_mandelstam_identity")?;
    require_finite(t, "verify_mandelstam_identity")?;
    require_finite(u, "verify_mandelstam_identity")?;
    let sum_m2: f64 = masses_gev.iter().map(|m| m * m).sum();
    let lhs = s + t + u;
    Ok((lhs - sum_m2).abs() < 1e-6 * (sum_m2.abs() + 1.0))
}

/// Differential cross-section for 2→2 scattering.
///
/// dσ/dΩ = |M|² / (64π²s) (natural units, GeV⁻²).
///
/// # Arguments
/// * `amplitude_sq` — |M|² (squared matrix element).
/// * `s_mandelstam` — Center-of-mass energy squared (GeV²).
#[inline]
pub fn differential_cross_section_2to2(
    amplitude_sq: f64,
    s_mandelstam: f64,
) -> Result<f64, MimamsaError> {
    require_finite(amplitude_sq, "differential_cross_section_2to2")?;
    require_finite(s_mandelstam, "differential_cross_section_2to2")?;
    if amplitude_sq < 0.0 {
        warn!(amplitude_sq, "|M|² must be non-negative");
        return Err(MimamsaError::Computation(
            "differential_cross_section_2to2: |M|² must be non-negative".to_string(),
        ));
    }
    if s_mandelstam <= 0.0 {
        return Err(MimamsaError::Computation(
            "differential_cross_section_2to2: s must be positive".to_string(),
        ));
    }
    ensure_finite(
        amplitude_sq / (64.0 * PI * PI * s_mandelstam),
        "differential_cross_section_2to2",
    )
}

/// Total cross-section for 2→2 scattering with massless final state and isotropic |M|².
///
/// σ = |M|² / (16πs) (natural units, GeV⁻²).
#[inline]
pub fn total_cross_section_2to2_massless(
    amplitude_sq: f64,
    s_mandelstam: f64,
) -> Result<f64, MimamsaError> {
    require_finite(amplitude_sq, "total_cross_section_2to2_massless")?;
    require_finite(s_mandelstam, "total_cross_section_2to2_massless")?;
    if amplitude_sq < 0.0 {
        warn!(amplitude_sq, "|M|² must be non-negative");
        return Err(MimamsaError::Computation(
            "total_cross_section_2to2_massless: |M|² must be non-negative".to_string(),
        ));
    }
    if s_mandelstam <= 0.0 {
        return Err(MimamsaError::Computation(
            "total_cross_section_2to2_massless: s must be positive".to_string(),
        ));
    }
    ensure_finite(
        amplitude_sq / (16.0 * PI * s_mandelstam),
        "total_cross_section_2to2_massless",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_2to2_momenta() -> (FourMomentum, FourMomentum, FourMomentum, FourMomentum) {
        // e+e- → μ+μ- at √s = 10 GeV, massless approximation
        let e = 5.0;
        let p1 = FourMomentum::new(e, 0.0, 0.0, e).unwrap(); // e- along z
        let p2 = FourMomentum::new(e, 0.0, 0.0, -e).unwrap(); // e+ along -z
        let p3 = FourMomentum::new(e, e, 0.0, 0.0).unwrap(); // μ- along x
        let p4 = FourMomentum::new(e, -e, 0.0, 0.0).unwrap(); // μ+ along -x
        (p1, p2, p3, p4)
    }

    #[test]
    fn test_mandelstam_s() {
        let (p1, p2, _, _) = make_2to2_momenta();
        let s = mandelstam_s(&p1, &p2).unwrap();
        // s = (2E)² = 100
        assert!((s - 100.0).abs() < 1e-8);
    }

    #[test]
    fn test_mandelstam_identity_massless() {
        let (p1, p2, p3, p4) = make_2to2_momenta();
        let s = mandelstam_s(&p1, &p2).unwrap();
        let t = mandelstam_t(&p1, &p3).unwrap();
        let u = mandelstam_u(&p1, &p4).unwrap();
        // For massless particles: s + t + u = 0
        let ok = verify_mandelstam_identity(s, t, u, &[0.0, 0.0, 0.0, 0.0]).unwrap();
        assert!(ok, "s+t+u = {}, expected 0", s + t + u);
    }

    #[test]
    fn test_momentum_conservation() {
        let (p1, p2, p3, p4) = make_2to2_momenta();
        let diagram = TreeDiagram {
            vertices: vec![],
            internal_lines: vec![],
            external_lines: vec![
                ExternalLine {
                    particle_type: ParticleType::Fermion,
                    momentum: p1,
                    incoming: true,
                },
                ExternalLine {
                    particle_type: ParticleType::Fermion,
                    momentum: p2,
                    incoming: true,
                },
                ExternalLine {
                    particle_type: ParticleType::Fermion,
                    momentum: p3,
                    incoming: false,
                },
                ExternalLine {
                    particle_type: ParticleType::Fermion,
                    momentum: p4,
                    incoming: false,
                },
            ],
        };
        assert!(diagram.check_momentum_conservation().unwrap());
    }

    #[test]
    fn test_tree_level_amplitude_single_propagator() {
        // Single internal photon line at k² = 50
        let k = FourMomentum::new(10.0, 5.0, 3.0, 0.0).unwrap();
        let diagram = TreeDiagram {
            vertices: vec![
                Vertex {
                    coupling: 0.3,
                    particles: vec![
                        ParticleType::Fermion,
                        ParticleType::Fermion,
                        ParticleType::Photon,
                    ],
                },
                Vertex {
                    coupling: 0.3,
                    particles: vec![
                        ParticleType::Fermion,
                        ParticleType::Fermion,
                        ParticleType::Photon,
                    ],
                },
            ],
            internal_lines: vec![InternalLine {
                particle_type: ParticleType::Photon,
                momentum: k,
                mass_gev: 0.0,
            }],
            external_lines: vec![],
        };
        let amp = tree_level_amplitude(&diagram).unwrap();
        assert!(amp.abs() > 0.0);
    }

    #[test]
    fn test_cross_section_positive() {
        let sigma = total_cross_section_2to2_massless(1.0, 100.0).unwrap();
        assert!(sigma > 0.0);
    }

    #[test]
    fn test_cross_section_negative_s_rejected() {
        assert!(total_cross_section_2to2_massless(1.0, -1.0).is_err());
    }

    #[test]
    fn test_cross_section_negative_amplitude_sq_rejected() {
        assert!(total_cross_section_2to2_massless(-1.0, 100.0).is_err());
        assert!(differential_cross_section_2to2(-0.5, 100.0).is_err());
    }

    #[test]
    fn test_differential_vs_total() {
        // For isotropic |M|², total = differential × 4π
        let m2 = 1.0;
        let s = 100.0;
        let diff = differential_cross_section_2to2(m2, s).unwrap();
        let total = total_cross_section_2to2_massless(m2, s).unwrap();
        let ratio = total / diff;
        // total / diff = 64π²/(16π) = 4π
        assert!((ratio - 4.0 * PI).abs() < 1e-8);
    }
}
