//! Not just a pretty (inter)face.
//!
//! A pretty-printer and [ndjson](http://ndjson.org/) logger for the [log](https://docs.rs/log) crate.
//!
//! ## Examples
//! ```
//! femme::start();
//! log::warn!("Unauthorized access attempt on /login");
//! log::info!("Listening on port 8080");
//! ```

use env_logger::filter::{Builder, Filter};
pub use log::LevelFilter;

#[cfg(not(target_arch = "wasm32"))]
mod ndjson;

#[cfg(not(target_arch = "wasm32"))]
mod pretty;

#[cfg(target_arch = "wasm32")]
mod wasm;

/// Starts logging depending on current environment.
///
/// Always logs with 'Info' LevelFilter.
/// For other filters use with_level.
///
/// # Log output
///
/// - when compiling with `--release` uses ndjson.
/// - pretty-prints otherwise.
/// - works in WASM out of the box.
///
/// # Examples
///
/// ```
/// femme::start();
/// log::warn!("Unauthorized access attempt on /login");
/// log::info!("Listening on port 8080");
/// ```
pub fn start() {
    let filter = Builder::from_env("RUST_LOG").build();
    with_filter(filter);
}

/// Start logging with a log level.
///
/// All messages under the specified log level will statically be filtered out.
///
/// # Examples
/// ```
/// femme::with_level(log::LevelFilter::Trace);
/// ```
pub fn with_level(level: log::LevelFilter) {
    let filter = Builder::new().filter_level(level).build();
    with_filter(filter)
}

/// Start logging with a log filter.
///
/// # Examples
/// ```
/// let filter = env_logger::filter::Builder::from_env("RUST_LOG").build();
/// femme::with_filter(filter);
/// ```
pub fn with_filter(filter: Filter) {
    #[cfg(target_arch = "wasm32")]
    wasm::start(filter);

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Use ndjson in release mode, pretty logging while debugging.
        if cfg!(debug_assertions) {
            pretty::start(filter);
        } else {
            ndjson::start(filter);
        }
    }
}
