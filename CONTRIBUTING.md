# Contributing to hisab-mimamsa

## Getting Started

```bash
git clone https://github.com/MacCracken/hisab-mimamsa.git
cd hisab-mimamsa
cargo test --all-features
```

## Guidelines

- All physics functions must validate inputs and return `Result` for fallible operations.
- Use `crate::constants` for physical constants — never define them locally.
- Tracing:
  - `error!` for computational failures (RK4 divergence, FFT failure)
  - `warn!` for domain boundary violations (superluminal, singularity, negative energy)
  - `#[instrument]` on significant public functions (`level = "trace"` for computations, `level = "debug"` for aggregators)
  - Skip `#[instrument]` on trivial one-liners, tight-loop helpers, and hot constructors
- Every public function needs a doc comment with the formula it implements.
- Tests must validate against known physical values, not arbitrary numbers.
- Zero `unsafe`. Zero `println!`. Zero clippy warnings.

## Testing

```bash
cargo test --all-features       # all tests
cargo clippy --all-features     # lint
cargo bench                     # benchmarks
```

## License

By contributing you agree that your contributions will be licensed under GPL-3.0-only.
