//! Unified field models — GR+QFT bridge, holographic principle, fixed point convergence.
//!
//! This is the keystone module that bridges general relativity (macro) with
//! quantum field theory (micro) and connects to bhava's consciousness model
//! at Scales 3-7.
//!
//! # Key Concepts
//!
//! - **Holographic principle**: entropy bounds (Bekenstein) imply information
//!   scales with area, not volume — consciousness has a surface, not a bulk.
//! - **Cosmological fixed point**: at heat death / maximum entropy, all fields
//!   converge to ground state → manifestation_intensity = 0.0 → Unity.
//! - **Scale invariance**: the same mathematical structure (field + metric + entropy)
//!   appears at every scale from Planck to cosmic — "as above, so below" as a
//!   consequence of renormalization group flow.
//!
//! # Submodules
//!
//! - [`holographic`] — Bekenstein and holographic entropy bounds, information content
//! - [`fixed_point`] — Cosmic phase classification, manifestation intensity, Unity parameter
//! - [`scale_bridge`] — RG running coupling wrappers, bhava Scale 6/7 bridge functions
//!
//! # Examples
//!
//! Holographic bound matches black hole entropy:
//!
//! ```
//! use hisab_mimamsa::unified::holographic;
//! use hisab_mimamsa::relativity::{metric, black_hole};
//! use std::f64::consts::PI;
//!
//! let m_sun = 1.989e30;
//! let rs = metric::schwarzschild_radius(m_sun).unwrap();
//! let area = 4.0 * PI * rs * rs;
//! let s_holo = holographic::holographic_bound(area).unwrap();
//! let s_bh = black_hole::bekenstein_hawking_entropy(m_sun).unwrap();
//! assert!((s_holo - s_bh).abs() / s_bh < 1e-6);
//! ```
//!
//! Manifestation intensity — the universe today is ~31.5% from equilibrium:
//!
//! ```
//! use hisab_mimamsa::unified::fixed_point;
//! use hisab_mimamsa::cosmology::friedmann::CosmologicalParameters;
//!
//! let params = CosmologicalParameters::planck2018();
//! let intensity = fixed_point::manifestation_intensity(&params, 0.0).unwrap();
//! assert!(intensity > 0.3 && intensity < 0.35);
//! ```
//!
//! bhava Scale 6 bridge:
//!
//! ```
//! use hisab_mimamsa::unified::scale_bridge;
//! use hisab_mimamsa::cosmology::friedmann::CosmologicalParameters;
//!
//! let params = CosmologicalParameters::planck2018();
//! let output = scale_bridge::BridgeOutput::at_redshift(&params, 0.0).unwrap();
//! assert!(output.intensity > 0.0 && output.intensity < 1.0);
//! assert!((output.intensity + output.unity_param - 1.0).abs() < 1e-12);
//! ```

pub mod fixed_point;
pub mod holographic;
pub mod scale_bridge;
