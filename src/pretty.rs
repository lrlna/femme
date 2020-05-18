//! Pretty print logs.

use console::style;
use log::{kv, Level, LevelFilter, Log, Metadata, Record};
use std::fmt::Write;
use std::io;

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
        use io::Write;
        if self.enabled(record.metadata()) {
            let mut out = String::new();
            format_src(&mut out, &record);
            write!(out, " {}", &record.args()).unwrap();
            format_kv_pairs(&mut out, &record);
            writeln!(io::stdout(), "{}", out).unwrap();
        }
    }
    fn flush(&self) {}
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
            write!(self.string, "\n    {} {}", style(key).bold(), val).unwrap();
            Ok(())
        }
    }

    let mut visitor = Visitor { string: &mut out };
    record.key_values().visit(&mut visitor).unwrap();
}

fn format_src(out: &mut String, record: &Record<'_>) {
    let msg = record.target();
    match record.level() {
        Level::Trace | Level::Debug | Level::Info => {
            write!(out, "{}", style(msg).green().bold()).unwrap()
        }
        Level::Warn => write!(out, "{}", style(msg).yellow().bold()).unwrap(),
        Level::Error => write!(out, "{}", style(msg).red().bold()).unwrap(),
    }
}
