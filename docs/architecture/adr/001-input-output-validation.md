# ADR-001: Input/Output Validation on All Public Functions

**Status**: Accepted  
**Date**: 2026-03-30

## Context

Physics computations with floating-point arithmetic can silently produce NaN or Infinity from valid-looking inputs (e.g., `rest_energy(f64::MAX)` overflows to `-Inf`). Downstream consumers (bhava, soorat) would receive garbage data without any error signal.

## Decision

Every public function validates inputs **and** outputs:

1. **Input**: `require_finite(value, "fn_name")` rejects NaN/Inf before computation
2. **Output**: `ensure_finite(result, "fn_name")` catches overflow, 0/0, and other arithmetic pathologies after computation
3. **Complex**: `require_finite_complex` / `ensure_finite_complex` for QFT propagators

Functions that accept `f64` and can fail return `Result<T, MimamsaError>`. No function silently returns NaN.

## Consequences

- Every public function incurs a small overhead for finiteness checks (~2ns per call)
- Adversarial fuzzing with `[NaN, Inf, -Inf, 0.0, f64::MIN, f64::MAX, ...]` passes for all 88 public functions
- Downstream consumers can trust that `Ok(value)` is always finite and physically meaningful
- The `NonFinite` error variant provides the context string for debugging
