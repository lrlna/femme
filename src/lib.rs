use log::{LevelFilter, Log, Metadata, Record};

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
    println!("{} - {}", record.level(), record.args());
}
