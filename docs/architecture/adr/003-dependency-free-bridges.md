# ADR-003: Dependency-Free Bridge Functions

**Status**: Accepted  
**Date**: 2026-03-31

## Context

The unified module provides bridge functions that map physical data (from jyotish, tara, brahmanda) to scalars/structs consumed by bhava (emotional states) and soorat (visual world). The AGNOS ecosystem pattern (established by garjan and bodh) requires that bridge functions be dependency-free.

## Decision

Bridge functions in `unified::scale_bridge`:
- Accept **only f64 primitives and fixed-size arrays** (`&[f64; 10]`, `&[f64; 12]`, `&[(f64, f64)]`)
- **Never import types from upstream science crates** (no `jyotish::Planet`, no `tara::Star`)
- Return library-owned types (`PlanetaryField`, `BridgeOutput`, `f64`)
- The **game engine orchestrates**: it calls jyotish to compute positions, then passes the f64 results through hisab-mimamsa's bridge

## Consequences

- hisab-mimamsa has zero dependency on jyotish, tara, or brahmanda at compile time
- Bridge functions can be tested entirely with synthetic data (no astronomy crate needed)
- Consumers (bhava, soorat) import only hisab-mimamsa types, not the full science stack
- The orchestration layer (kiran, joshua) is the only component that imports all crates
- Adding new upstream crates (e.g., brahmanda for Scale 5) requires no API changes in hisab-mimamsa — just new bridge functions accepting f64
