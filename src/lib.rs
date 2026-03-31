//! Hisab-Mimamsa — Theoretical Physics Engine
//!
//! **Hisab** (Arabic: حساب — calculation) + **Mimamsa** (Sanskrit: मीमांसा — critical inquiry)
//!
//! Computational theoretical physics built on the hisab math foundation.
//! General relativity, quantum field theory, cosmology, and unified field models.
//!
//! # Architecture
//!
//! Four domain modules, each feature-gated:
//!
//! - [`relativity`] — General & special relativity: spacetime metrics, geodesics,
//!   Lorentz transforms, gravitational lensing, black hole thermodynamics
//! - `quantum_field` *(feature `qft`)* — Quantum field theory: field quantization,
//!   propagators, Feynman diagrams, renormalization (extends kana beyond circuits)
//! - `cosmology` *(feature `cosmology`)* — Friedmann equations, dark energy models,
//!   CMB power spectrum, large-scale structure, cosmic expansion history
//! - `unified` *(feature `unified`)* — Bridge between QFT and GR: effective field
//!   theory, holographic principle, information-theoretic bounds, fixed point convergence
//!
//! # Relationship to Other Crates
//!
//! ```text
//! hisab (math foundation)
//!   ├── hisab-mimamsa (this) — theoretical physics
//!   │     ├── relativity — spacetime, geodesics, black holes
//!   │     ├── quantum_field — QFT, propagators, renormalization
//!   │     ├── cosmology — Friedmann, CMB, expansion
//!   │     └── unified — GR+QFT bridge, holographic, fixed point
//!   ├── kana — quantum mechanics (circuits, states, operators)
//!   ├── tanmatra — atomic/subatomic (Standard Model, nuclear, decay)
//!   ├── falak — orbital mechanics (Keplerian, transfers)
//!   ├── tara — stellar astrophysics (classification, evolution)
//!   └── jyotish — astronomical computation (ephemeris, calendar)
//! ```
//!
//! # bhava Bridge (Scale 3-7)
//!
//! The unified module provides bridge functions for bhava's consciousness model:
//! - Scale 3: Planetary field → personality manifestation (via jyotish)
//! - Scale 4: Stellar influence → soul motivation layers (via tara)
//! - Scale 5: Galactic structure → civilizational personality fields
//! - Scale 6: Cosmic expansion → manifestation intensity scalar
//! - Scale 7: Cosmic breath phase → unity/differentiation cycle
//!
//! The fixed point at zero (Unity) emerges from the cosmological model:
//! at heat death / maximum entropy, all fields converge to ground state,
//! all manifestation intensity → 0.0, all bridge outputs → identity element.
//!
//! # Examples
//!
//! Special relativity — muon time dilation at 0.994c:
//!
//! ```
//! use hisab_mimamsa::relativity::lorentz;
//!
//! let gamma = lorentz::lorentz_factor(0.994 * lorentz::C).unwrap();
//! assert!((gamma - 9.14).abs() < 0.1);
//! ```
//!
//! General relativity — Schwarzschild radius of the Sun:
//!
//! ```
//! use hisab_mimamsa::relativity::metric;
//!
//! let rs = metric::schwarzschild_radius(1.989e30).unwrap();
//! assert!((rs - 2953.0).abs() < 5.0);
//! ```
//!
//! Black hole thermodynamics — Hawking temperature:
//!
//! ```
//! use hisab_mimamsa::relativity::black_hole;
//!
//! let t = black_hole::hawking_temperature(1.989e30).unwrap();
//! assert!(t > 1e-9 && t < 1e-6);
//! ```

pub mod constants;
pub mod error;
pub mod relativity;

#[cfg(feature = "qft")]
pub mod quantum_field;

#[cfg(feature = "cosmology")]
pub mod cosmology;

#[cfg(feature = "unified")]
pub mod unified;

#[cfg(feature = "logging")]
pub mod logging;

pub use error::MimamsaError;
