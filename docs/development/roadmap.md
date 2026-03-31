# Roadmap

> hisab-mimamsa — Versioned milestones

## Completed

### v0.1.0 — Relativity + Cosmology (March 2026)

- [x] Special relativity (Lorentz transforms, four-vectors, time dilation, Doppler)
- [x] General relativity (Schwarzschild metric, geodesics, gravitational lensing)
- [x] Black hole thermodynamics (Hawking radiation, Bekenstein-Hawking entropy)
- [x] Friedmann cosmology (H(z), critical density, age of universe)
- [x] Cosmic expansion (distances, scale factor, CMB temperature)
- [x] Adversarial input hardening (NaN/Inf fuzzing, ensure_finite on all outputs)
- [x] 116 tests, 5 benchmarks

### v0.2.0 — Quantum Field Theory (March 2026)

- [x] Momentum-space propagators (scalar, fermion, gauge boson)
- [x] Position-space propagator via FFT
- [x] Vacuum energy (zero-point, Casimir effect, regularized density, dimreg)
- [x] Running coupling constants (QED/QCD beta functions, RK4 + analytic)
- [x] Asymptotic freedom detection
- [x] Feynman diagrams (tree-level amplitudes, Mandelstam variables, cross-sections)
- [x] Complex number validation (require_finite_complex, ensure_finite_complex)

### v0.3.0 — Unified Field (March 2026)

- [x] Holographic principle (Bekenstein bound, holographic entropy, information content)
- [x] Fixed point convergence (cosmic phase, entropy ratio, manifestation intensity)
- [x] Scale bridge Scales 6-7 (manifestation intensity, cosmic breath phase)
- [x] Scale bridge Scale 3 (PlanetaryField — element/modality/aspect/house analysis)
- [x] Scale bridge Scales 4-5 (stubs for tara and brahmanda)
- [x] 257 tests, 14 benchmarks, 4 examples, 9 doc tests

## In Progress

### v1.0 — Release Hardening

- [ ] Full first-party standards compliance (CI, Makefile, deny.toml, etc.)
- [ ] 80%+ code coverage via cargo-llvm-cov
- [ ] CI/CD workflows (ci.yml, release.yml)
- [ ] bench-history.sh with CSV tracking
- [ ] Performance optimization pass
- [ ] MSRV verification (1.89)

## Planned

### Post-v1.0 — Scale Integration

- [ ] Scale 4 implementation (tara stellar data → soul motivation layers)
- [ ] Scale 5 implementation (brahmanda galactic data → civilizational fields)
- [ ] Scale 3 enrichment (additional jyotish features: nakshatras, dasas, yogas)
- [ ] Cyclic cosmology extension for Scale 7 (cosmic breath oscillation)

### Post-v1.0 — Physics Extensions

- [ ] Kerr black holes (rotating)
- [ ] Reissner-Nordstrom black holes (charged)
- [ ] Loop corrections in QFT (requires Gamma function, dimensional regularization)
- [ ] Spinor algebra for full Dirac propagator
- [ ] BAO wiggles in cosmological power spectrum
- [ ] Dark energy equation of state w(z) models

## Dependencies for Scale Completion

| Scale | Upstream Crate | Status |
|-------|---------------|--------|
| 3 | jyotish (planetary astronomy) | v0.1.0, ~v1 ready |
| 4 | tara (stellar astrophysics) | v0.2.0, exists |
| 5 | brahmanda (large-scale structure) | v0.1.0, scaffolded |
| 6 | (cosmology — internal) | Complete |
| 7 | (cosmology — internal) | Complete |
