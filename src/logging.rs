//! Optional logging initialisation.
//!
//! Activated by the `logging` feature. Uses `HISAB_MIMAMSA_LOG` env var.

/// Initialise logging with default level `info`.
pub fn init() {
    init_with_level("info");
}

/// Initialise logging with a custom default level.
pub fn init_with_level(default_level: &str) {
    use tracing_subscriber::EnvFilter;
    use tracing_subscriber::fmt;
    use tracing_subscriber::prelude::*;

    let filter = EnvFilter::try_from_env("HISAB_MIMAMSA_LOG")
        .unwrap_or_else(|_| EnvFilter::new(default_level));

    let _ = tracing_subscriber::registry()
        .with(fmt::layer().with_target(true).with_thread_ids(true))
        .with(filter)
        .try_init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        init();
    }

    #[test]
    fn test_init_with_level() {
        init_with_level("warn");
    }
}
