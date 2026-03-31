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

// ── QFT constants ────────────────────────────────────────────────────────

/// Fine structure constant α (dimensionless). CODATA 2018.
pub const ALPHA: f64 = 7.297_352_569_3e-3;

/// Strong coupling constant α_s at M_Z (dimensionless). PDG 2020.
pub const ALPHA_S_MZ: f64 = 0.1179;

/// Z boson mass (GeV/c²). PDG 2020.
pub const M_Z_GEV: f64 = 91.1876;

/// Electron mass (GeV/c²). CODATA 2018.
pub const M_ELECTRON_GEV: f64 = 0.000_510_998_950;

/// Muon mass (GeV/c²). CODATA 2018.
pub const M_MUON_GEV: f64 = 0.105_658_375_5;

/// Elementary charge (C). CODATA 2018.
pub const E_CHARGE: f64 = 1.602_176_634e-19;

/// Conversion factor: 1 GeV in Joules.
pub const GEV_TO_JOULES: f64 = 1.602_176_634e-10;

/// (ℏc)² in GeV²·fm² — cross-section unit conversion.
pub const HBAR_C_SQ_GEV2_FM2: f64 = 0.389_379_372_1;
