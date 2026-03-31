# Changelog

## 0.1.0 — 2026-03-31

### Added

#### Relativity
- Special relativity: Lorentz factor, time dilation, length contraction, relativistic energy/momentum, velocity addition, Doppler effect, four-vectors with Lorentz boosts
- General relativity: Schwarzschild metric (event horizon, ISCO, photon sphere), gravitational time dilation, redshift, orbital velocity
- Geodesics: effective potential, weak-field light deflection, Shapiro delay
- Black hole thermodynamics: Hawking temperature, Bekenstein-Hawking entropy, evaporation time, surface gravity
- Gravitational lensing: Einstein ring radius, point-source magnification, critical surface density

#### Cosmology (feature-gated)
- Friedmann equations: Hubble parameter H(z), critical density, deceleration parameter, age of universe
- Cosmic expansion: comoving/luminosity/angular-diameter distances, lookback time, Hubble distance, scale factor, CMB temperature

#### Quantum Field Theory (feature-gated)
- Propagators: scalar (Klein-Gordon), fermion (Dirac scalar part), gauge boson (Feynman gauge), position-space via FFT
- Vacuum energy: zero-point energy, Casimir force/energy, regularized density, dimensional regularization
- Running couplings: QED/QCD one-loop beta functions, numerical (RK4) and analytic running, asymptotic freedom
- Feynman diagrams: tree-level amplitude evaluation, Mandelstam variables, differential/total cross-sections

#### Unified Field (feature-gated)
- Holographic principle: Bekenstein bound, holographic entropy bound, information content, BH information bits, cosmological horizon entropy
- Fixed point convergence: cosmic phase classification, entropy ratio, manifestation intensity, Unity parameter
- Scale bridge: RG coupling wrappers, bhava Scales 3-7 bridge functions, PlanetaryField for Scale 3

#### Infrastructure
- Centralized physical constants (SI + QFT natural units)
- Input validation (require_finite) and output validation (ensure_finite) on all public functions
- Complex number validation (require_finite_complex, ensure_finite_complex)
- Structured logging (tracing::warn) on physics-boundary violations
- 257 tests: unit, adversarial fuzzing, physical invariants, serde roundtrips, doc tests
- 14 criterion benchmarks across all modules
- 4 runnable examples (relativity, cosmology, qft, unified)
- 9 doc tests with verified physics
