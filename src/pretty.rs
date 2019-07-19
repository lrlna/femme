//! Pretty print logs.

use console::style;
use log::{Level, LevelFilter, Log, Metadata, Record};

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
            pretty_print(record)
        }
    }
    fn flush(&self) {}
}

// TODO: use format_key_val to pretty print kv's from log crate
fn pretty_print(record: &Record<'_>) {
    println!("{}{}", format_message(&record), format_line(&record));
}

// TODO: use with key values that could eventually be put through log macros
fn _format_key_val(key: &str, val: &str) -> String {
    format!("   › {}: {}\n", style(key).magenta(), val)
}

fn format_line(record: &Record<'_>) -> String {
    match (record.file(), record.line()) {
        (Some(file), Some(line)) => format!("   {}:{}\n", file, line),
        _ => String::new(),
    }
}

fn format_message(record: &Record<'_>) -> String {
    use Level::*;
    let symbol = match record.level() {
        Trace => format!("{}", "◯"),
        Debug => format!("{}", "◎"),
        Info => format!("{}", "●"),
        Warn => format!("{}", "⌿"),
        Error => format!("{}", "✖"),
    };

    let msg = format!("{}  {}\n", symbol, style(record.args()).underlined());
    match record.level() {
        Trace | Debug | Info => format!("{}", style(msg).green()),
        Warn => format!("{}", style(msg).yellow()),
        Error => format!("{}", style(msg).red()),
    }
}
