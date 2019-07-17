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

fn pretty_print(record: &Record<'_>) {
    let symbol = get_level_symbol(record.level());
    println!("{} - {}", symbol, record.args());
}

fn get_level_symbol(level: Level) -> String {
    use Level::*;
    match level {
        Info => format!("{}", style("*").cyan()),
        _ => format!("{}", style("=").red()),
    }
}
