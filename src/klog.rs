use core::fmt;
use core::fmt::Write;
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
#[cfg(feature = "serial_log")]
use serial;

pub fn print_fmt(args: fmt::Arguments) {
    #[cfg(feature = "serial_log")]
    serial::get_default().write_fmt(args.clone()).ok();
}

macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::klog::print_fmt(format_args!($($arg)*)));
}

struct Logger;
static LOGGER: Logger = Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[{}] {}\n", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}
