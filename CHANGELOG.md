# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] — 2026-03-31

### Added

#### Relativity (always available)
- Special relativity: Lorentz factor, time dilation, length contraction, relativistic energy/momentum, velocity addition, Doppler effect
- Four-vectors with Lorentz boosts, interval classification (timelike/lightlike/spacelike)
- Schwarzschild metric: event horizon, ISCO, photon sphere, orbital velocity
- Gravitational time dilation and redshift
- Geodesics: effective potential, weak-field light deflection, Shapiro delay
- Black hole thermodynamics: Hawking temperature, Bekenstein-Hawking entropy, evaporation time, surface gravity, `BlackHoleProperties` bundle
- Gravitational lensing: Einstein ring radius, point-source magnification, critical surface density

#### Cosmology (feature: `cosmology`)
- Friedmann equations: Hubble parameter H(z), critical density, deceleration parameter, age of universe
- Planck 2018 ΛCDM parameters (H₀=67.4, Ω_m=0.315, Ω_Λ=0.685)
- Cosmic expansion: comoving, luminosity, angular-diameter distances, lookback time, Hubble distance
- Scale factor, redshift ↔ scale factor conversion, CMB temperature evolution

#### Quantum Field Theory (feature: `qft`)
- Momentum-space propagators: scalar (Klein-Gordon), fermion (Dirac scalar part), gauge boson (Feynman gauge)
- Position-space propagator via numerical FFT
- Vacuum energy: zero-point energy, Casimir force/energy per area, regularized density, dimensional regularization
- Running coupling constants: QED/QCD one-loop β-functions, numerical (RK4) and analytic running
- Asymptotic freedom detection
- Feynman diagrams: `TreeDiagram`, tree-level amplitude evaluation, Mandelstam variables (s, t, u)
- Cross-sections: differential and total for 2→2 scattering
- `FourMomentum` type with Minkowski inner product, addition, subtraction

#### Unified Field (feature: `unified`)
- Holographic principle: Bekenstein bound, holographic entropy bound, information content (bits), black hole information, cosmological horizon entropy (Gibbons-Hawking)
- Fixed point convergence: cosmic phase classification (radiation/matter/dark energy), entropy ratio, manifestation intensity ∈ [0,1], unity parameter
- `FixedPointState` and `BridgeOutput` bundles
- Scale bridge: Scales 3-7 dependency-free bridge functions for bhava/soorat
- Scale 3: `PlanetaryField` — element/modality balance, aspect tension/harmony, house emphasis, retrograde fraction (from jyotish f64 primitives)
- Scale 4-5: stubs for tara (stellar) and brahmanda (galactic)
- Scale 6: manifestation intensity from cosmological entropy ratio
- Scale 7: cosmic breath phase (ΛCDM, extensible to cyclic cosmology)
- RG coupling wrappers: `scale_coupling_qed`, `scale_coupling_qcd`

#### Foundation
- Centralized physical constants (`constants.rs`): SI (C, G, ℏ, k_B) + QFT natural units (α, α_s, M_Z, particle masses, conversion factors)
- Error handling (`error.rs`): `MimamsaError` with 10 variants including `NonFinite`
- Input validation: `require_finite`, `require_all_finite` on all public function inputs
- Output validation: `ensure_finite`, `ensure_finite_complex` on all computed results
- Structured tracing: `warn!` on domain boundary violations, `error!` on computational failures (RK4, FFT), `#[instrument]` spans on ~50 public functions (`trace`/`debug` levels with argument capture and `ret` on key aggregators)

#### Testing & Quality
- 257 tests across 6 test suites: unit (133), adversarial fuzzing (65), integration (4), physical invariants (29), serde roundtrip (17), doc tests (9)
- Adversarial fuzzing with [NaN, ±Inf, ±0, f64::MIN, f64::MAX, f64::EPSILON, -1, 1e-300] on every public function
- Physical invariant tests: Lorentz boost interval preservation, Mandelstam identity, thermodynamic scaling laws, Etherington reciprocity, entropy monotonicity
- Serde roundtrip tests for all public types
- 14 criterion benchmarks with CSV history tracking
- 4 runnable examples (relativity, cosmology, qft, unified)
- Zero `unsafe`, zero `println!`, zero clippy warnings

#### Documentation
- Architecture overview with system diagram and data flow
- Development roadmap with versioned milestones
- 5 Architecture Decision Records (input validation, natural units, dependency-free bridges, manifestation intensity model, feature gates)
- CHANGELOG, CONTRIBUTING, SECURITY, CODE_OF_CONDUCT
- CI/CD workflows (ci.yml, release.yml)
- Makefile with standard targets
- deny.toml for license and advisory compliance
