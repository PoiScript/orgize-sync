use colored::Colorize;
use log::{Level, Log, Metadata, Record};

pub struct Logger;

impl Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        match record.level() {
            Level::Error => print!("{}", "ERR:".bright_red()),
            Level::Warn => print!("{}", "WARN:".red()),
            Level::Info => print!("{}", "INFO:".green()),
            Level::Debug => print!("{}", "DEBUG:".cyan()),
            Level::Trace => print!("{}", "TRACE:".white()),
        }
        println!(" {}", record.args());
    }

    fn flush(&self) {}
}

pub static LOGGER: Logger = Logger;
