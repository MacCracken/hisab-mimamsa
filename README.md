# hisab-mimamsa

> **Hisab** (Arabic: حساب — calculation) + **Mimamsa** (Sanskrit: मीमांसा — critical inquiry into the nature of things)

**Theoretical physics engine** — general relativity, quantum field theory, cosmology, and unified field models. Built on the [hisab](https://crates.io/crates/hisab) math foundation.

[![License: GPL-3.0-only](https://img.shields.io/badge/license-GPL--3.0--only-blue.svg)](LICENSE)

## Features

### Relativity (`relativity`)

**Special relativity** — Lorentz transformations, four-vectors, boosts, time dilation, length contraction, relativistic energy/momentum, velocity addition, Doppler effect.

**General relativity** — Schwarzschild metric (event horizon, ISCO, photon sphere), gravitational time dilation, redshift, geodesics (effective potential, light deflection, Shapiro delay), black hole thermodynamics (Hawking temperature, Bekenstein-Hawking entropy, evaporation time), gravitational lensing (Einstein rings, magnification, critical density).

### Cosmology (`cosmology`, feature-gated)

**Friedmann equations** — Hubble parameter H(z), critical density, deceleration parameter, age of universe. Planck 2018 ΛCDM parameters (H₀=67.4 km/s/Mpc, Ω_m=0.315, Ω_Λ=0.685).

**Cosmic expansion** — comoving, luminosity, and angular diameter distances, lookback time, Hubble distance, scale factor, CMB temperature evolution.

### Quantum Field Theory (`qft`, feature-gated)

**Propagators** — scalar (Klein-Gordon), fermion (Dirac scalar part), gauge boson (Feynman gauge), position-space via FFT.

**Vacuum energy** — zero-point energy, Casimir force/energy, regularized vacuum density, dimensional regularization.

**Running couplings** — QED/QCD one-loop β-functions, numerical (RK4) and analytic running, asymptotic freedom detection.

**Feynman diagrams** — tree-level amplitude evaluation, Mandelstam variables (s, t, u), differential and total cross-sections.

### Unified Field (`unified`, feature-gated)

**Holographic principle** — Bekenstein bound, holographic entropy bound, information content, black hole information bits, cosmological horizon entropy.

**Fixed point convergence** — cosmic phase classification, entropy ratio, manifestation intensity scalar, unity parameter.

**Scale bridge** — Scales 3–7 bridge functions for bhava/soorat: planetary field (Scale 3), stellar influence stub (Scale 4), galactic structure stub (Scale 5), manifestation intensity (Scale 6), cosmic breath phase (Scale 7).

## Quick Start

### Relativity

```rust
use hisab_mimamsa::relativity::{lorentz, metric, black_hole};

// Special relativity: muon time dilation at 0.994c
let gamma = lorentz::lorentz_factor(0.994 * lorentz::C).unwrap();
println!("Lorentz factor: {gamma:.2}"); // ~9.14

// General relativity: Schwarzschild radius of the Sun
let rs = metric::schwarzschild_radius(1.989e30).unwrap();
println!("Sun r_s: {rs:.0} m"); // ~2953 m

// Black hole thermodynamics: Hawking temperature
let t = black_hole::hawking_temperature(1.989e30).unwrap();
println!("Solar mass BH temp: {t:.2e} K"); // ~6.2e-8 K
```

### Cosmology

```rust
use hisab_mimamsa::cosmology::{friedmann, expansion};

let params = friedmann::CosmologicalParameters::planck2018();

// Age of the universe
let age_s = friedmann::age_of_universe(&params, 1100.0, 10000).unwrap();
let age_gyr = age_s / (365.25 * 24.0 * 3600.0 * 1e9);
println!("Age: {age_gyr:.1} Gyr"); // ~13.8 Gyr

// CMB temperature at decoupling (z ≈ 1100)
let t_cmb = expansion::cmb_temperature(1100.0).unwrap();
println!("CMB at decoupling: {t_cmb:.0} K"); // ~3000 K
```

### Quantum Field Theory

```rust
use hisab_mimamsa::quantum_field::{FourMomentum, propagator, coupling};
use hisab_mimamsa::constants::{ALPHA_S_MZ, M_Z_GEV};

// Scalar propagator near the mass shell
let p = FourMomentum::new(0.511e-3, 0.0, 0.0, 0.0).unwrap();
let prop = propagator::scalar_propagator(&p, 0.511e-3, propagator::DEFAULT_EPSILON).unwrap();
println!("|propagator| = {:.2e}", prop.abs());

// QCD asymptotic freedom: coupling decreases with energy
let alpha_1tev = coupling::running_coupling_qcd_analytic(ALPHA_S_MZ, M_Z_GEV, 1000.0, 6).unwrap();
println!("alpha_s(1 TeV) = {alpha_1tev:.4}"); // < alpha_s(M_Z)
```

### Unified Field / bhava Bridge

```rust
use hisab_mimamsa::unified::{fixed_point, scale_bridge};
use hisab_mimamsa::cosmology::friedmann::CosmologicalParameters;

let params = CosmologicalParameters::planck2018();

// Manifestation intensity: how far the universe is from heat death
let intensity = fixed_point::manifestation_intensity(&params, 0.0).unwrap();
println!("Manifestation intensity (now): {intensity:.4}"); // ~0.315

// Full bridge output for bhava/soorat
let bridge = scale_bridge::BridgeOutput::at_redshift(&params, 0.0).unwrap();
println!("Unity parameter: {:.4}", bridge.unity_param); // ~0.685
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `std` | yes | Standard library support |
| `cosmology` | no | Friedmann equations, expansion history, CMB |
| `qft` | no | Quantum field theory: propagators, vacuum, couplings, Feynman |
| `unified` | no | GR+QFT bridge, holographic principle, Scale 3-7 bridges |
| `full` | no | Enables all features |
| `logging` | no | Structured tracing via `HISAB_MIMAMSA_LOG` env var |

```toml
[dependencies]
hisab-mimamsa = { version = "0.1", features = ["unified"] }
```

## Validated Physics

All implementations validated against known results:

| Test | Expected | Verified |
|------|----------|----------|
| Electron rest energy | 0.511 MeV | 0.511 MeV |
| Sun Schwarzschild radius | ~2953 m | 2953 m |
| Solar light deflection (Eddington) | 1.75 arcsec | 1.75 arcsec |
| Velocity addition (0.9c + 0.9c) | < c (~0.9945c) | 0.9945c |
| Age of universe (Planck ΛCDM) | ~13.8 Gyr | 13.8 Gyr |
| CMB temperature today | 2.7255 K | 2.7255 K |
| Hawking T ∝ 1/M | 2× mass → 0.5× temp | confirmed |
| BH entropy ∝ M² | 2× mass → 4× entropy | confirmed |
| Lorentz boost preserves interval | Δs² invariant | confirmed |
| q₀ < 0 (accelerating expansion) | dark energy dominated | confirmed |
| QED coupling increases with energy | α(1 TeV) > α(M_Z) | confirmed |
| QCD asymptotic freedom | α_s(1 TeV) < α_s(M_Z) | confirmed |
| Casimir force at 1 μm | ~1.3e-3 N/m² | confirmed |
| Mandelstam identity s+t+u = Σm² | for 2→2 scattering | confirmed |
| Holographic bound = BH entropy | for Schwarzschild BH | confirmed |
| Entropy ratio(z=0) ≈ Ω_Λ | ~0.685 | confirmed |

## Relationship to AGNOS Science Stack

```
hisab (math foundation)
  ├── hisab-mimamsa (this) — theoretical physics
  │     ├── relativity — spacetime, geodesics, black holes
  │     ├── cosmology — Friedmann, CMB, expansion
  │     ├── quantum_field — propagators, vacuum, couplings, Feynman
  │     └── unified — holographic, fixed point, Scale 3-7 bridges
  ├── brahmanda — galactic / large-scale structure (Scale 5)
  ├── tara — stellar astrophysics (Scale 4)
  ├── jyotish — astronomical computation (Scale 3)
  ├── falak — orbital mechanics
  ├── kana — quantum mechanics (circuits, states, operators)
  └── tanmatra — atomic/subatomic (Standard Model, nuclear, decay)
```

## bhava Bridge (consciousness model)

The unified module provides bridge functions for [bhava](https://crates.io/crates/bhava) (emotional states) and [soorat](https://crates.io/crates/soorat) (visible world):

- **Scale 3**: Planetary field → personality manifestation (via jyotish) — `PlanetaryField`
- **Scale 4**: Stellar influence → soul motivation layers (via tara) — stub
- **Scale 5**: Galactic structure → civilizational personality fields (via brahmanda) — stub
- **Scale 6**: Cosmic expansion → manifestation intensity scalar — `bridge_scale_6()`
- **Scale 7**: Cosmic breath phase → unity/differentiation cycle — `bridge_scale_7()`

The fixed point at zero (Unity) emerges from the cosmological model: at maximum entropy, all fields converge to ground state, all manifestation intensity → 0.0.

Bridge functions are dependency-free (f64 primitives only) — see [ADR-003](docs/architecture/adr/003-dependency-free-bridges.md).

## Status

| Module | Tests | Status |
|--------|-------|--------|
| relativity | 31 | Complete (SR + GR + BH + lensing) |
| cosmology | 15 | Complete (Friedmann + expansion) |
| quantum_field | 47 | Complete (propagators + vacuum + coupling + Feynman) |
| unified | 40 | Complete (holographic + fixed point + Scale 3-7 bridges) |

257 tests, 14 benchmarks, 4 examples, 9 doc tests, clippy clean, zero `unsafe`.

## Documentation

- [Architecture Overview](docs/architecture/overview.md) — system diagram, module structure, data flow
- [Development Roadmap](docs/development/roadmap.md) — milestones and planned work
- Architecture Decision Records:
  - [ADR-001: Input/Output Validation](docs/architecture/adr/001-input-output-validation.md)
  - [ADR-002: Natural Units in QFT](docs/architecture/adr/002-natural-units-in-qft.md)
  - [ADR-003: Dependency-Free Bridges](docs/architecture/adr/003-dependency-free-bridges.md)
  - [ADR-004: Manifestation Intensity Model](docs/architecture/adr/004-manifestation-intensity-model.md)
  - [ADR-005: Feature-Gated Modules](docs/architecture/adr/005-feature-gated-modules.md)

## License

GPL-3.0-only

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.
