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
pub mod ndjson;
pub mod pretty;

/// Starts logging depending on current environment. If in production, will print
/// ndjson, otherwise pretty-prints.
///
/// # Examples
/// ```
/// femme::start(log::LevelFilter::Trace).unwrap();
/// log::warn!("Unauthorized access attempt on /login");
/// log::info!("Listening on port 8080");
/// ```
pub fn start(filter: log::LevelFilter) -> Result<(), log::SetLoggerError> {
    if cfg!(debug_assertions) {
        pretty::Logger::new().start(filter)
    } else {
        ndjson::Logger::new().start(filter)
    }
}
