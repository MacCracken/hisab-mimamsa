# ADR-005: Feature-Gated Physics Modules

**Status**: Accepted  
**Date**: 2026-03-30

## Context

The crate covers four physics domains with different dependency profiles and compile-time costs. Not all consumers need all physics — a game engine doing only SR/GR shouldn't pay for QFT compile time.

## Decision

```toml
[features]
default = ["std"]
cosmology = []
qft = []
unified = ["cosmology", "qft"]
full = ["std", "logging", "cosmology", "qft", "unified"]
```

- **relativity** is always available (no feature gate) — it's the foundation
- **cosmology**, **qft** are independently selectable
- **unified** requires both (it bridges them)
- Each feature combination compiles and tests independently

## Consequences

- `cargo check` (default) only compiles relativity — fast iteration for SR/GR work
- `cargo test --features qft` tests QFT in isolation without cosmology overhead
- The `unified` feature automatically enables both upstream features
- CI tests all feature combinations: default, cosmology, qft, unified, full
- Test modules use `#[cfg(feature = "...")]` to gate feature-specific tests
