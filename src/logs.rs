// Copyright 2026 Pranav R S.
// Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use tracing_subscriber::EnvFilter;

use crate::config::LogConfig;

// Initializes the logging system using the provided configuration and returns the log level as a string.
pub fn init(cfg: &LogConfig) -> String {
    let filter = EnvFilter::try_new(&cfg.level)
        .expect("failed to initialize logger: invalid log level");

    tracing_subscriber::fmt().with_env_filter(filter).init();
    cfg.level.clone()
}
