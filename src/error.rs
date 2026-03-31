//! Error types for hisab-mimamsa.

use thiserror::Error;

/// Errors that can occur in theoretical physics computations.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum MimamsaError {
    /// Invalid metric signature or degenerate metric tensor.
    #[error("invalid metric: {0}")]
    InvalidMetric(String),

    /// Coordinate singularity encountered (e.g., r = 0, r = r_s).
    #[error("coordinate singularity at {location}: {detail}")]
    Singularity { location: String, detail: String },

    /// Velocity exceeds speed of light.
    #[error("superluminal velocity: {v} > c")]
    Superluminal { v: f64 },

    /// Negative energy density in classical regime.
    #[error("negative energy density: {rho}")]
    NegativeEnergy { rho: f64 },

    /// Divergent computation (renormalization needed or failed).
    #[error("divergence in {context}: {detail}")]
    Divergence { context: String, detail: String },

    /// Numerical computation failed to converge.
    #[error("convergence failure after {iterations} iterations: {detail}")]
    ConvergenceFailed { iterations: usize, detail: String },

    /// Invalid cosmological parameters.
    #[error("invalid cosmological parameter: {0}")]
    InvalidCosmology(String),

    /// Generic computation error.
    #[error("computation error: {0}")]
    Computation(String),

    /// Non-finite input (NaN or ±Infinity).
    #[error("non-finite input in {context}: {value}")]
    NonFinite { context: &'static str, value: f64 },
}

/// Validate that a single f64 input is finite. Returns `NonFinite` error if not.
#[inline]
pub fn require_finite(value: f64, context: &'static str) -> Result<(), MimamsaError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(MimamsaError::NonFinite { context, value })
    }
}

/// Validate that all f64 inputs are finite.
#[inline]
pub fn require_all_finite(values: &[f64], context: &'static str) -> Result<(), MimamsaError> {
    for &v in values {
        require_finite(v, context)?;
    }
    Ok(())
}

/// Validate that a computed result is finite (catches overflow, 0/0, etc.).
#[inline]
pub fn ensure_finite(value: f64, context: &'static str) -> Result<f64, MimamsaError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(MimamsaError::Computation(format!(
            "{context}: result is {value}"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_finite_nan() {
        assert!(ensure_finite(f64::NAN, "test").is_err());
    }

    #[test]
    fn test_ensure_finite_inf() {
        assert!(ensure_finite(f64::INFINITY, "test").is_err());
    }

    #[test]
    fn test_require_finite_complex_nan_re() {
        let z = hisab::Complex::new(f64::NAN, 1.0);
        assert!(require_finite_complex(&z, "test").is_err());
    }

    #[test]
    fn test_require_finite_complex_nan_im() {
        let z = hisab::Complex::new(1.0, f64::INFINITY);
        assert!(require_finite_complex(&z, "test").is_err());
    }

    #[test]
    fn test_ensure_finite_complex_nan() {
        let z = hisab::Complex::new(f64::NAN, 0.0);
        assert!(ensure_finite_complex(z, "test").is_err());
    }
}

/// Validate that a Complex input has finite real and imaginary parts.
#[inline]
pub fn require_finite_complex(
    z: &hisab::Complex,
    context: &'static str,
) -> Result<(), MimamsaError> {
    if z.re.is_finite() && z.im.is_finite() {
        Ok(())
    } else {
        let bad = if !z.re.is_finite() { z.re } else { z.im };
        Err(MimamsaError::NonFinite {
            context,
            value: bad,
        })
    }
}

/// Validate that a computed Complex result has finite components.
#[inline]
pub fn ensure_finite_complex(
    z: hisab::Complex,
    context: &'static str,
) -> Result<hisab::Complex, MimamsaError> {
    if z.re.is_finite() && z.im.is_finite() {
        Ok(z)
    } else {
        Err(MimamsaError::Computation(format!(
            "{context}: result has non-finite component ({z})"
        )))
    }
}
