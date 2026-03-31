//! General & Special Relativity
//!
//! Spacetime metrics, geodesics, Lorentz transformations, gravitational lensing,
//! black hole thermodynamics, and Penrose diagrams.

pub mod black_hole;
pub mod geodesic;
pub mod lensing;
pub mod lorentz;
pub mod metric;

// Re-export submodules directly — consumers use relativity::lorentz::foo
// No glob re-exports to avoid ambiguity (C is defined in both lorentz and metric).
