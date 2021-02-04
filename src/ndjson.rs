//! Print logs as ndjson.

use log::{kv, LevelFilter, Log, Metadata, Record};
use std::io::{self, StdoutLock, Write};
use std::time;
use crate::filter::{Filter, Builder};

/// Start logging.
pub(crate) fn start(level: LevelFilter) {
    let filter = Builder::new()
        .filter_level(level)
        .build();
    let logger = Box::new(Logger { filter });
    log::set_boxed_logger(logger).expect("Could not start logging");
    log::set_max_level(level);
}

/// Start logging with filter.
pub(crate) fn with_filter(filter: Filter) {
    let max_level = filter.filter();
    let logger = Box::new(Logger { filter });
    log::set_boxed_logger(logger).expect("Could not start logging");
    log::set_max_level(max_level);
}

#[derive(Debug)]
pub(crate) struct Logger {
    filter: Filter,
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
       self.filter.enabled(metadata)
    }

    fn log(&self, record: &Record<'_>) {
        if self.enabled(record.metadata()) {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            let level = get_level(record.level());
            let time = time::UNIX_EPOCH.elapsed().unwrap().as_millis();
            write!(&mut handle, "{{\"level\":{},\"time\":{},\"msg\":", level, time).unwrap();
            serde_json::to_writer(&mut handle, record.args()).unwrap();
            format_kv_pairs(&mut handle, &record);
            writeln!(&mut handle, "}}").unwrap();
        }
    }
    fn flush(&self) {}
}

fn get_level(level: log::Level) -> u8 {
    use log::Level::*;
    match level {
        Trace => 10,
        Debug => 20,
        Info => 30,
        Warn => 40,
        Error => 50,
    }
}

fn format_kv_pairs<'b>(mut out: &mut StdoutLock<'b>, record: &Record) {
    struct Visitor<'a, 'b> {
        string: &'a mut StdoutLock<'b>,
    }

    impl<'kvs, 'a, 'b> kv::Visitor<'kvs> for Visitor<'a, 'b> {
        fn visit_pair(
            &mut self,
            key: kv::Key<'kvs>,
            val: kv::Value<'kvs>,
        ) -> Result<(), kv::Error> {
            write!(self.string, ",\"{}\":\"{}\"", key, val)?;
            Ok(())
        }
    }

    let mut visitor = Visitor { string: &mut out };
    record.key_values().visit(&mut visitor).unwrap();
}
