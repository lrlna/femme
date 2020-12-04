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

use log::SetLoggerError;

pub use log::LevelFilter;

#[cfg(not(target_arch = "wasm32"))]
mod ndjson;

#[cfg(not(target_arch = "wasm32"))]
mod pretty;

#[cfg(target_arch = "wasm32")]
mod wasm;

/// Starts logging depending on current environment.
///
/// ## Log output
///
/// - when compiling with `--release` uses ndjson.
/// - pretty-prints otherwise.
/// - works in WASM out of the box.
///
/// ## Examples
///
/// ```
/// femme::start();
/// log::warn!("Unauthorized access attempt on /login");
/// log::info!("Listening on port 8080");
/// ```
///
/// ## Panics
///
/// This function will panic if it is called more than once, or if another library has already initialized a global logger.
///
pub fn start() {
    with_level(LevelFilter::Info)
}

/// Starts logging depending on current environment.
///
/// ## Log output
///
/// - when compiling with `--release` uses ndjson.
/// - pretty-prints otherwise.
/// - works in WASM out of the box.
///
/// ## Examples
///
/// ```
/// femme::try_start().ok(); // Handle Result or ignore
/// log::warn!("Unauthorized access attempt on /login");
/// log::info!("Listening on port 8080");
/// ```
///
/// ## Errors
///
/// This function will fail if it is called more than once, or if another library has already initialized a global logger.
///
pub fn try_start() -> Result<(), SetLoggerError> {
    try_with_level(LevelFilter::Info)
}

/// Start logging with a log level.
///
/// All messages under the specified log level will statically be filtered out.
///
/// ## Examples
///
/// ```
/// femme::with_level(log::LevelFilter::Warn);
/// log::warn!("Unauthorized access attempt on /login");
/// log::info!("Listening on port 8080"); // Will be hidden
/// ```
///
/// ## Panics
///
/// This function will panic if it is called more than once, or if another library has already initialized a global logger.
///
pub fn with_level(level: log::LevelFilter) {
    try_with_level(level).expect("Could not start logging")
}

/// Start logging with a log level.
///
/// All messages under the specified log level will statically be filtered out.
///
/// ## Examples
///
/// ```
/// femme::try_with_level(log::LevelFilter::Warn).ok(); // Handle Result or ignore
/// log::warn!("Unauthorized access attempt on /login");
/// log::info!("Listening on port 8080"); // Will be hidden
/// ```
///
/// ## Errors
///
/// This function will fail if it is called more than once, or if another library has already initialized a global logger.
///
pub fn try_with_level(level: log::LevelFilter) -> Result<(), SetLoggerError> {
    #[cfg(target_arch = "wasm32")]
    wasm::start(level)?;

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Use ndjson in release mode, pretty logging while debugging.
        if cfg!(debug_assertions) {
            pretty::start(level)?;
        } else {
            ndjson::start(level)?;
        }
    }

    Ok(())
}
