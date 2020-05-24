//! Pretty print logs.

use console::style;
use log::{kv, Level, LevelFilter, Log, Metadata, Record};
use std::io::{self, StdoutLock, Write};

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
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            format_src(&mut handle, &record);
            write!(handle, " {}", &record.args()).unwrap();
            format_kv_pairs(&mut handle, &record);
            writeln!(&mut handle, "").unwrap();
        }
    }
    fn flush(&self) {}
}

fn format_kv_pairs<'b>(mut out: &mut StdoutLock<'b>, record: &Record) {
    struct Visitor<'a, 'b> {
        stdout: &'a mut StdoutLock<'b>,
    }

    impl<'kvs, 'a, 'b> kv::Visitor<'kvs> for Visitor<'a, 'b> {
        fn visit_pair(
            &mut self,
            key: kv::Key<'kvs>,
            val: kv::Value<'kvs>,
        ) -> Result<(), kv::Error> {
            write!(self.stdout, "\n    {} {}", style(key).bold(), val).unwrap();
            Ok(())
        }
    }

    let mut visitor = Visitor { stdout: &mut out };
    record.key_values().visit(&mut visitor).unwrap();
}

fn format_src(out: &mut StdoutLock<'_>, record: &Record<'_>) {
    let msg = record.target();
    match record.level() {
        Level::Trace | Level::Debug | Level::Info => {
            write!(out, "{}", style(msg).green().bold()).unwrap()
        }
        Level::Warn => write!(out, "{}", style(msg).yellow().bold()).unwrap(),
        Level::Error => write!(out, "{}", style(msg).red().bold()).unwrap(),
    }
}
