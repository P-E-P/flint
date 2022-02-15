use core::fmt::Write;
use core::fmt;
#[cfg(feature = "serial_log")]
use serial;
use log::{Log, Record, Level, Metadata, SetLoggerError, LevelFilter};

pub fn print_fmt(args: fmt::Arguments) {
    #[cfg(feature = "serial_log")]
    serial::get_default().write_fmt(args.clone()).ok();
}

#[macro_export]
macro_rules! printk {
    ($($args: tt)*) => {
        crate::klog::print_fmt(format_args!($($args)*))
    };
}

struct Logger;
static LOGGER: Logger = Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            printk!("[{}] {}\n", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info)).ok();
    Ok(())
}
