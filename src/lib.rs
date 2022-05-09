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
    with_level(LevelFilter::Info);
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
    Femme::builder().level(level).into_femme().start()
}

/// ```
/// femme::Femme::builder()
///    .ndjson()
///    .level(log::LevelFilter::Trace)
///    .into_femme()
///    .start();
/// ```
#[derive(Debug)]
pub struct Femme {
    level: LevelFilter,
    mode: Option<Mode>,
}

#[derive(Debug)]
enum Mode {
    Pretty,
    NdJson,
}

#[derive(Debug)]
pub struct FemmeBuilder {
    level: Option<LevelFilter>,
    mode: Option<Mode>,
}

impl Femme {
    pub fn builder() -> FemmeBuilder {
        FemmeBuilder {
            level: None,
            mode: None,
        }
    }
    pub fn start(self) {
        #[cfg(target_arch = "wasm32")]
        wasm::start(self.level);
        #[cfg(not(target_arch = "wasm32"))]
        match self.mode {
            Some(Mode::Pretty) => pretty::start(self.level),
            Some(Mode::NdJson) => ndjson::start(self.level),
            None => {
                // By default, use ndjson in release mode, pretty logging while debugging.
                if cfg!(debug_assertions) {
                    pretty::start(self.level);
                } else {
                    ndjson::start(self.level);
                }
            }
        }
    }
}

impl FemmeBuilder {
    pub fn level(mut self, level: LevelFilter) -> FemmeBuilder {
        self.level = Some(level);
        self
    }
    pub fn pretty(mut self) -> FemmeBuilder {
        self.mode = Some(Mode::Pretty);
        self
    }
    pub fn ndjson(mut self) -> FemmeBuilder {
        self.mode = Some(Mode::NdJson);
        self
    }
    pub fn into_femme(self) -> Femme {
        Femme {
            level: self.level.unwrap_or(LevelFilter::Info),
            mode: self.mode,
        }
    }
}
