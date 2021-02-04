//! Filtering for log records.
//!
//! ```
//! use log::LevelFilter;
//!
//! let filter = femme::Builder::new()
//!     .filter_level(LevelFilter::Warn)
//!     .filter_module("main", LevelFilter::Debug)
//!     .filter_module("test_mod", LevelFilter::Trace)
//!     .filter_module("another_mod", LevelFilter::Info)
//!     .build();
//! femme::with_filter(filter);
//! ```
use log::{Metadata, Level, LevelFilter};
use std::mem;

/// A log filter.
///
/// This struct can be used to determine whether or not a log record
/// should be written to the output.
#[derive(Debug)]
pub struct Filter {
    directives: Vec<Directive>,
}

impl Filter {
    /// Determines if a log message with the specified metadata would be logged.
    pub fn enabled(&self, metadata: &Metadata) -> bool {
        let level = metadata.level();
        let target = metadata.target();

        enabled(&self.directives, level, target)
    }

    /// Returns the maximum `LevelFilter` that this filter instance is
   /// configured to output.
    pub fn filter(&self) -> LevelFilter {
        self.directives
            .iter()
            .map(|d| d.level)
            .max()
            .unwrap_or(LevelFilter::Off)
    }
}

// Check whether a level and target are enabled by the set of directives.
fn enabled(directives: &[Directive], level: Level, target: &str) -> bool {
    // Search for the longest match, the vector is assumed to be pre-sorted.
    for directive in directives.iter().rev() {
        match directive.name {
            Some(ref name) if !target.starts_with(&**name) => {}
            Some(..) | None => return level <= directive.level,
        }
    }
    false
}

/// A builder for a log filter.
#[derive(Debug)]
struct Directive {
    name: Option<String>,
    level: LevelFilter,
}

pub struct Builder {
    directives: Vec<Directive>,
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder {
    /// Initializes the filter builder with defaults.
    pub fn new() -> Builder {
        Builder {
            directives: Vec::new(),
        }
    }

    /// Adds a directive to the filter for a specific module.
    pub fn filter_module(&mut self, module: &str, level: LevelFilter) -> &mut Self {
        self.filter(Some(module), level)
    }

    /// Adds a directive to the filter for all modules.
    pub fn filter_level(&mut self, level: LevelFilter) -> &mut Self {
        self.filter(None, level)
    }

    /// Adds a directive to the filter.
   ///
   /// The given module (if any) will log at most the specified level provided.
   /// If no module is provided then the filter will apply to all log messages.
    pub fn filter(&mut self, module: Option<&str>, level: LevelFilter) -> &mut Self {
        self.directives.push(Directive {
            name: module.map(|s| s.to_string()),
            level,
        });
        self
    }

    /// Build a log filter.
    pub fn build(&mut self) -> Filter {
        if self.directives.is_empty() {
            // Adds the default filter if none exist
            self.directives.push(Directive {
                name: None,
                level: LevelFilter::Error,
            });
        } else {
            // Sort the directives by length of their name, this allows a
            // little more efficient lookup at runtime.
            self.directives.sort_by(|a, b| {
                let alen = a.name.as_ref().map(|a| a.len()).unwrap_or(0);
                let blen = b.name.as_ref().map(|b| b.len()).unwrap_or(0);
                alen.cmp(&blen)
            });
        }

        Filter {
            directives: mem::replace(&mut self.directives, Vec::new()),
        }
    }
}