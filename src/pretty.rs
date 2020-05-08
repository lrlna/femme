//! Pretty print logs.

use console::style;
use log::{kv, Level, LevelFilter, Log, Metadata, Record};

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
            println!(
                "{} {}{}",
                format_src(&record),
                &record.args(),
                format_kv_pairs(&record),
            );
        }
    }
    fn flush(&self) {}
}

fn format_kv_pairs(record: &Record) -> String {
    struct Visitor {
        string: String,
    }

    impl<'kvs> kv::Visitor<'kvs> for Visitor {
        fn visit_pair(
            &mut self,
            key: kv::Key<'kvs>,
            val: kv::Value<'kvs>,
        ) -> Result<(), kv::Error> {
            let string = &format!("\n    {} {}", style(key).bold(), val);
            self.string.push_str(string);
            Ok(())
        }
    }

    let mut visitor = Visitor {
        string: String::new(),
    };
    record.key_values().visit(&mut visitor).unwrap();
    visitor.string
}

fn format_src(record: &Record<'_>) -> String {
    let msg = record.target();
    match record.level() {
        Level::Trace | Level::Debug | Level::Info => format!("{}", style(msg).green().bold()),
        Level::Warn => format!("{}", style(msg).yellow().bold()),
        Level::Error => format!("{}", style(msg).red().bold()),
    }
}
