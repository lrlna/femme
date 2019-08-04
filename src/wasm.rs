//! Print logs as ndjson.

use log::{LevelFilter, Log, Metadata, Record, Level};

/// A WASM logger for the browser.
#[derive(Debug)]
pub struct Logger {}

impl Logger {
    pub fn new() -> Self {
        Self {}
    }

    /// Start logging.
    pub fn start(self, filter: LevelFilter) -> Result<(), log::SetLoggerError> {
        let res = log::set_boxed_logger(Box::new(self));
        if res.is_ok() {
            log::set_max_level(filter);
        }
        res
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record<'_>) {
         if self.enabled(record.metadata()) {
             let string = format!("{} {}", record.args(), format_line(&record));
             match record.level() {
                 Level::Error => web_sys::console::error_1(&string.into()),
                 Level::Warn => web_sys::console::warn_1(&string.into()),
                 Level::Info => web_sys::console::debug_1(&string.into()),
                 _ => web_sys::console::log_1(&string.into()),
             }
        }
    }
    fn flush(&self) {}
}

fn format_line(record: &Record<'_>) -> String {
    match (record.file(), record.line()) {
        (Some(file), Some(line)) => format!("({}:{})", file, line),
        _ => String::new(),
    }
}
