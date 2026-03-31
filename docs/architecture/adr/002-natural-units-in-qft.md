# ADR-002: Natural Units (GeV) in Quantum Field Theory Module

**Status**: Accepted  
**Date**: 2026-03-31

## Context

The relativity and cosmology modules use SI units (meters, kilograms, seconds, Joules) following CODATA conventions. QFT conventionally uses natural units where ℏ = c = 1, with energy measured in GeV.

Mixing unit systems within the same crate creates conversion errors and cognitive overhead.

## Decision

- **Relativity + cosmology**: SI units (consistent with their existing implementation)
- **Quantum field theory**: Natural units (GeV) for all internal computations
- **Vacuum submodule**: Provides both SI functions (Casimir) and natural-unit functions (regularized density)
- **Constants module**: Houses both SI constants (`C`, `G`, `HBAR`) and QFT constants (`ALPHA`, `M_Z_GEV`)
- **Every function documents its unit system** in the doc comment

## Consequences

- QFT function signatures use `mass_gev: f64` parameter names to make units explicit
- Cross-module computations (e.g., unified module) must handle unit conversion at the boundary
- The `GEV_TO_JOULES` constant enables explicit conversion when needed
- No implicit unit conversions happen inside any function
