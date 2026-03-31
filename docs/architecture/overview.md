# Architecture Overview

> hisab-mimamsa — Theoretical physics engine for AGNOS

## System Diagram

```
                    ┌─────────────────────────────────────────┐
                    │           hisab (math foundation)        │
                    │  Complex, FFT, RK4, integration, linalg │
                    └──────────────────┬──────────────────────┘
                                       │
                    ┌──────────────────┴──────────────────────┐
                    │           hisab-mimamsa                  │
                    │       theoretical physics engine         │
                    ├─────────────────────────────────────────┤
                    │                                         │
                    │  ┌───────────┐   ┌──────────────┐      │
                    │  │ relativity│   │  cosmology    │      │
                    │  │  SR + GR  │   │  Friedmann +  │      │
                    │  │  BH thermo│   │  expansion    │      │
                    │  │  lensing  │   │  CMB          │      │
                    │  └─────┬─────┘   └──────┬───────┘      │
                    │        │                │               │
                    │  ┌─────┴─────┐          │               │
                    │  │quantum_   │          │               │
                    │  │field      │          │               │
                    │  │propagators│          │               │
                    │  │vacuum     │          │               │
                    │  │coupling   │          │               │
                    │  │feynman    │          │               │
                    │  └─────┬─────┘          │               │
                    │        │                │               │
                    │  ┌─────┴────────────────┴───────┐      │
                    │  │         unified               │      │
                    │  │  holographic  fixed_point      │      │
                    │  │  scale_bridge (Scales 3-7)     │      │
                    │  └──────────────┬────────────────┘      │
                    │                 │                        │
                    └─────────────────┼────────────────────────┘
                                      │
              ┌───────────────────────┼───────────────────────┐
              │                       │                       │
       ┌──────┴──────┐      ┌────────┴────────┐    ┌────────┴───────┐
       │   bodh      │      │     bhava       │    │    soorat      │
       │ (cognition) │      │ (emotion/       │    │ (visual world) │
       │             │      │  personality)   │    │                │
       └─────────────┘      └─────────────────┘    └────────────────┘
```

## Module Structure

### `relativity` (always available)

Special and general relativity. No feature gate — core to all physics.

| Submodule | Purpose | Key Types |
|-----------|---------|-----------|
| `lorentz` | SR: Lorentz transforms, four-vectors | `FourVector`, `IntervalType` |
| `metric` | Schwarzschild metric, time dilation | `MetricSignature`, `SpacetimeMetric` |
| `geodesic` | Geodesic paths, light deflection | `GeodesicPoint`, `GeodesicType` |
| `black_hole` | Hawking radiation, entropy | `BlackHoleProperties` |
| `lensing` | Einstein rings, magnification | — |

### `cosmology` (feature: `cosmology`)

Friedmann equations and cosmic expansion history.

| Submodule | Purpose | Key Types |
|-----------|---------|-----------|
| `friedmann` | H(z), critical density, age | `CosmologicalParameters` |
| `expansion` | Distances, scale factor, CMB | — |

### `quantum_field` (feature: `qft`)

QFT: propagators, vacuum energy, running couplings, Feynman diagrams.

| Submodule | Purpose | Key Types |
|-----------|---------|-----------|
| `propagator` | Klein-Gordon, Dirac, gauge boson | — |
| `vacuum` | Casimir effect, zero-point energy | — |
| `coupling` | Beta functions, RG running | `CouplingAnalysis` |
| `feynman` | Tree-level amplitudes, cross-sections | `TreeDiagram`, `ParticleType` |

Shared types: `FourMomentum`, `FieldType`, `GaugeChoice`

### `unified` (feature: `unified`, implies `cosmology` + `qft`)

GR+QFT bridge, holographic principle, bhava/soorat consciousness bridge.

| Submodule | Purpose | Key Types |
|-----------|---------|-----------|
| `holographic` | Entropy bounds, information content | — |
| `fixed_point` | Cosmic phase, manifestation intensity | `CosmicPhase`, `FixedPointState` |
| `scale_bridge` | Scales 3-7 bridge functions | `PlanetaryField`, `BridgeOutput` |

## Data Flow: Science → Consciousness

```
jyotish → planetary longitudes, aspects, houses (f64 primitives)
    ↓
unified::scale_bridge::bridge_scale_3() → PlanetaryField
    ↓
bodh/bhava consume PlanetaryField for personality modulation
soorat consumes PlanetaryField for field visualization

cosmology → H(z), a(z), Ω parameters
    ↓
unified::fixed_point::manifestation_intensity() → f64 [0,1]
unified::scale_bridge::bridge_scale_6/7() → BridgeOutput
    ↓
bodh/bhava consume for cosmic-scale consciousness modulation
```

**Bridge pattern**: All bridge functions are dependency-free — they take f64 primitives, not external crate types. The game engine orchestrates, calling science crates and bridge functions separately.

## Error Handling

Every public function returns `Result<T, MimamsaError>`. Three-layer validation:

1. **Input**: `require_finite()` / `require_all_finite()` — rejects NaN/Inf
2. **Domain**: Physics boundary checks with `tracing::warn!` (superluminal, singularity, Landau pole)
3. **Output**: `ensure_finite()` / `ensure_finite_complex()` — catches overflow, 0/0

## Observability

Structured tracing via the `tracing` crate at three levels:

| Level | Usage | Example |
|-------|-------|---------|
| `error!` | Computational failures (RK4/FFT divergence) | `error!(%e, "RK4 integration failed")` |
| `warn!` | Domain boundary violations (superluminal, singularity, Landau pole) | `warn!(v, "superluminal velocity rejected")` |
| `#[instrument]` | Span instrumentation on ~50 public functions | Entry/exit tracing with arguments |

Span levels: `trace` for individual computations, `debug` for higher-level aggregators (bridge outputs, integration results). Key aggregator functions (`BlackHoleProperties::from_mass`, `FixedPointState::at_redshift`, `BridgeOutput::at_redshift`) also log return values via `ret`.

Enable with the `logging` feature and `HISAB_MIMAMSA_LOG` env var (e.g., `HISAB_MIMAMSA_LOG=debug`).

## Constants

Centralized in `src/constants.rs`. SI units for GR/cosmology, natural units (GeV) for QFT.

Modules re-export relevant constants for backward compatibility (e.g., `lorentz::C`, `metric::G`).

## Feature Flags

| Feature | Default | Depends On | Description |
|---------|---------|------------|-------------|
| `std` | yes | — | Standard library support |
| `cosmology` | no | — | Friedmann + expansion |
| `qft` | no | — | Quantum field theory |
| `unified` | no | `cosmology`, `qft` | GR+QFT bridge, consciousness model |
| `full` | no | all | Everything |
| `logging` | no | — | Structured tracing (subscriber + env filter) |

## Consumers

- **bodh** — psychology engine: consumes Scale 3-7 bridge outputs for cognitive modeling
- **bhava** — emotion/personality: consumes bridge outputs for emotional state modulation
- **soorat** — visual engine: consumes bridge outputs for field visualization
- **kiran/joshua** — game engines: orchestrate science crate calls and bridge function invocations
