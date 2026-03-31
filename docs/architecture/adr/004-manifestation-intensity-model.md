# ADR-004: Manifestation Intensity as Entropy Ratio

**Status**: Accepted  
**Date**: 2026-03-31

## Context

The unified module needs a scalar ∈ [0, 1] representing "how far the universe is from heat death" — the **manifestation intensity** that drives bhava's Scale 6 consciousness modulation. Multiple formulations were considered:

1. Raw deceleration parameter q(z) — unbounded, non-intuitive mapping
2. Dark energy fraction Ω_Λ(z) — approaches 1 monotonically but isn't bounded in early universe
3. Entropy ratio S(z)/S_max — naturally ∈ [0, 1], physically motivated, monotonic

## Decision

**Manifestation intensity = 1 − entropy_ratio**, where:

```
entropy_ratio(z) = Ω_Λ / (Ω_r(1+z)⁴ + Ω_m(1+z)³ + Ω_k(1+z)² + Ω_Λ)
```

This simplification arises because the cosmological horizon entropy S ∝ 1/H², and the ratio S(z)/S_max has constant prefactors that cancel. The denominator is the full Friedmann energy budget.

## Properties

- At z = 0 (now): intensity ≈ 0.315 (matter + radiation fraction of energy budget)
- At z → ∞ (Big Bang): intensity → 1.0 (all energy in radiation/matter, far from equilibrium)
- At z → −1 (heat death): intensity → 0.0 (pure dark energy, maximum entropy, Unity)
- Monotonically decreasing with cosmic time (increases with z)

## Consequences

- Simple, fast computation (no integration required)
- Physically transparent: measures thermodynamic disequilibrium
- Unity parameter = 1 − intensity = entropy_ratio (fixed-point attractor)
- Scale 7 (cosmic breath phase) currently equals Scale 6 for ΛCDM; will diverge for cyclic cosmology
