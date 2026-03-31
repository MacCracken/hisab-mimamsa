//! Fundamental physical constants used throughout hisab-mimamsa.
//!
//! All values in SI units. Sources: CODATA 2018, IAU 2015.

/// Speed of light in vacuum (m/s).
pub const C: f64 = 299_792_458.0;

/// Speed of light squared (m²/s²).
pub const C2: f64 = C * C;

/// Gravitational constant G (m³ kg⁻¹ s⁻²).
pub const G: f64 = 6.674_30e-11;

/// Planck's reduced constant ℏ (J·s).
pub const HBAR: f64 = 1.054_571_817e-34;

/// Boltzmann constant k_B (J/K).
pub const K_B: f64 = 1.380_649e-23;
