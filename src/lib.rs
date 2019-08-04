//! Not just a pretty (inter)face.
//!
//! A pretty-printer and [ndjson](http://ndjson.org/) logger for the [log](https://docs.rs/log) crate.
//!
//! ## Examples
//! ```
//! femme::start(log::LevelFilter::Trace)?;
//! log::warn!("Unauthorized access attempt on /login");
//! log::info!("Listening on port 8080");
//! ```

#[cfg(not(target_arch = "wasm32"))]
pub mod ndjson;

#[cfg(not(target_arch = "wasm32"))]
pub mod pretty;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

/// Starts logging depending on current environment.
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
/// femme::start(log::LevelFilter::Trace).unwrap();
/// log::warn!("Unauthorized access attempt on /login");
/// log::info!("Listening on port 8080");
/// ```
pub fn start(filter: log::LevelFilter) -> Result<(), log::SetLoggerError> {
    #[cfg(target_arch = "wasm32")]
    wasm::Logger::new().start(filter)?;

    #[cfg(not(target_arch = "wasm32"))]
    {
        if cfg!(debug_assertions) {
            pretty::Logger::new().start(filter)?;
        } else {
            ndjson::Logger::new().start(filter)?;
        }
    }

    Ok(())
}
