//! Print logs as ndjson.

use log::{kv, LevelFilter, Log, Metadata, Record};
use std::fmt::Write;
use std::io;
use std::time;

/// Start logging.
pub(crate) fn start(level: LevelFilter) {
    let logger = Box::new(Logger {});
    log::set_boxed_logger(logger).expect("Could not start logging");
    log::set_max_level(level);
}

#[derive(Debug)]
pub(crate) struct Logger {}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record<'_>) {
        if self.enabled(record.metadata()) {
            let mut out = String::from("{");
            write!(&mut out, "\"level\":{}", get_level(record.level())).unwrap();
            write!(&mut out, ",\"time\":{}", time::UNIX_EPOCH.elapsed().unwrap().as_millis()).unwrap();
            write!(&mut out, ",\"msg\":\"{}\"", record.args()).unwrap();
            format_kv_pairs(&mut out, &record);
            writeln!(&mut out, "{}", "}").unwrap();
            io::Write::write(&mut io::stdout(), out.as_bytes()).unwrap();
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

fn format_kv_pairs(mut out: &mut String, record: &Record) {
    struct Visitor<'a> {
        string: &'a mut String,
    }

    impl<'kvs, 'a> kv::Visitor<'kvs> for Visitor<'a> {
        fn visit_pair(
            &mut self,
            key: kv::Key<'kvs>,
            val: kv::Value<'kvs>,
        ) -> Result<(), kv::Error> {
            write!(self.string, ",\"{}\":{}", key, val).unwrap();
            Ok(())
        }
    }

    let mut visitor = Visitor { string: &mut out };
    record.key_values().visit(&mut visitor).unwrap();
}
