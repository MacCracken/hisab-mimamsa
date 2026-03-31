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

**Fixed point convergence** — cosmic phase classification, entropy ratio, manifestation intensity scalar, unity parameter, `FixedPointState` bundle.

**Scale bridge** — RG running coupling wrappers, bhava Scale 6 (manifestation intensity) and Scale 7 (cosmic breath phase) bridge functions, `BridgeOutput` bundle.

## Quick Start

```rust
use hisab_mimamsa::relativity::{lorentz, metric, black_hole};

// Special relativity: muon time dilation at 0.994c
let gamma = lorentz::lorentz_factor(0.994 * lorentz::C).unwrap();
println!("Lorentz factor: {gamma:.2}"); // ~9.14

// General relativity: Schwarzschild radius of the Sun
let rs = metric::schwarzschild_radius(1.989e30);
println!("Sun r_s: {rs:.0} m"); // ~2953 m

// Black hole thermodynamics: Hawking temperature
let t = black_hole::hawking_temperature(1.989e30);
println!("Solar mass BH temp: {t:.2e} K"); // ~6.2e-8 K
```

```rust
use hisab_mimamsa::cosmology::{friedmann, expansion};

let params = friedmann::CosmologicalParameters::planck2018();

// Age of the universe
let age_s = friedmann::age_of_universe(&params, 1100.0, 10000);
let age_gyr = age_s / (365.25 * 24.0 * 3600.0 * 1e9);
println!("Age: {age_gyr:.1} Gyr"); // ~13.8 Gyr

// CMB temperature at decoupling (z ≈ 1100)
let t_cmb = expansion::cmb_temperature(1100.0);
println!("CMB at decoupling: {t_cmb:.0} K"); // ~3000 K
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `std` | yes | Standard library support |
| `cosmology` | no | Friedmann equations, expansion history, CMB |
| `qft` | no | Quantum field theory (planned) |
| `unified` | no | GR+QFT bridge, holographic principle (planned) |
| `full` | no | Enables all features |
| `logging` | no | `HISAB_MIMAMSA_LOG` env var logging |

```toml
[dependencies]
hisab-mimamsa = { version = "0.1", features = ["cosmology"] }
```

## Validated Physics

All implementations use real physics validated against known results:

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

## Relationship to AGNOS Science Stack

```
hisab (math foundation)
  ├── hisab-mimamsa (this) — theoretical physics
  │     ├── relativity — spacetime, geodesics, black holes
  │     ├── cosmology — Friedmann, CMB, expansion
  │     ├── quantum_field — QFT (planned)
  │     └── unified — GR+QFT bridge (planned)
  ├── kana — quantum mechanics (circuits, states, operators)
  ├── tanmatra — atomic/subatomic (Standard Model, nuclear, decay)
  ├── falak — orbital mechanics (Keplerian, transfers)
  ├── tara — stellar astrophysics (classification, evolution)
  └── jyotish — astronomical computation (ephemeris, calendar)
```

## bhava Bridge (consciousness model)

The unified module (v0.3.0) will provide bridge functions for the [bhava](https://crates.io/crates/bhava) personality engine's multi-scale consciousness model:

- **Scale 3**: Planetary field → personality manifestation (via jyotish)
- **Scale 4**: Stellar influence → soul motivation layers (via tara)
- **Scale 5**: Galactic structure → civilizational personality fields
- **Scale 6**: Cosmic expansion → manifestation intensity scalar
- **Scale 7**: Cosmic breath phase → unity/differentiation cycle

The fixed point at zero (Unity) emerges from the cosmological model: at maximum entropy, all fields converge to ground state, all manifestation intensity → 0.0.

## Status

| Module | Tests | Status |
|--------|-------|--------|
| relativity | 29 | Complete (SR + GR + BH + lensing) |
| cosmology | 19 | Complete (Friedmann + expansion) |
| quantum_field | 44 | Complete (propagators + vacuum + coupling + Feynman) |
| unified | 30 | Complete (holographic + fixed point + scale bridge) |

244 tests, 14 benchmarks, 4 examples, 9 doc tests, clippy clean, zero `unsafe`.

## License

GPL-3.0-only

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.
