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
    let message = colourize(
        record.level(),
        format!("{}  {}", symbol, style(record.args()).underlined()),
    );
    println!("{}", message);
    println!("   {}", format_line(&record));
    println!("{}", format_key_val("d128", "0E+3"));
    println!("{}", format_key_val("HTTPMeth", "GET"));
    println!("{}", format_key_val("Status", "200"));
    println!();
}

fn format_key_val(key: &str, val: &str) -> String {
    format!("   › {}: {}", style(key).magenta(), val)
}

fn format_line(record: &Record<'_>) -> String {
    match (record.file(), record.line()) {
        (Some(file), Some(line)) => format!("{}:{}", file, line),
        _ => String::new(),
    }
}

fn colourize(level: Level, print: String) -> String {
    use Level::*;
    match level {
        Trace | Debug | Info => format!("{}", style(print).green()),
        Warn => format!("{}", style(print).yellow()),
        Error => format!("{}", style(print).red()),
    }
}

fn get_level_symbol(level: Level) -> String {
    use Level::*;
    match level {
        Trace => format!("{}", "◯"),
        Debug => format!("{}", "◎"),
        Info => format!("{}", "●"),
        Warn => format!("{}", "⌿"),
        Error => format!("{}", "✖"),
    }
}
