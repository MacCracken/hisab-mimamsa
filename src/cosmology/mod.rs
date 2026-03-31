//! Cosmology — Friedmann equations, expansion history, CMB, dark energy.
//!
//! Models the large-scale evolution of the universe. Provides the physics
//! for bhava Scale 6-7 (cosmic expansion → manifestation intensity).

pub mod expansion;
pub mod friedmann;

pub use expansion::*;
pub use friedmann::*;
