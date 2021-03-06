use core::fmt;
use core::fmt::Write;
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

#[cfg(feature = "serial_log")]
mod serial_logger {
    use crate::serial::Serial;
    /// An option containing the default serial to use for communication.
    static mut DEFAULT: Option<Serial> = None;

    /// Retrieve a mutable reference to the default serial.
    ///
    /// # Note
    /// This function will initialize the default serial during it's first call.
    pub fn default() -> &'static mut Serial {
        unsafe {
            if DEFAULT.is_none() {
                DEFAULT = Some(Serial::default());
            }
            DEFAULT.as_mut().unwrap()
        }
    }
}

#[cfg(feature = "vga_log")]
mod vga_logger {
    use crate::vga::text::Writer;

    /// An option containing the default vga writer to use for communication.
    static mut DEFAULT: Option<Writer> = None;

    /// Retrieve a mutable reference to the default vga writer
    ///
    /// # Note
    /// This function will initialize the default serial during it's first call.
    pub fn default() -> &'static mut Writer {
        unsafe {
            if DEFAULT.is_none() {
                DEFAULT = Some(Writer::default());
            }
            DEFAULT.as_mut().unwrap()
        }
    }
}

/// Print a precompiled format string and it's arguments to the default serial
/// output.
pub fn serial_print(args: fmt::Arguments) {
    serial_logger::default().write_fmt(args).ok();
}

/// Print a precompiled format string and it's arguments to the default vga
/// output.
pub fn vga_print(args: fmt::Arguments) {
    vga_logger::default().write_fmt(args).ok();
}

pub fn print_fmt(args: fmt::Arguments) {
    #[cfg(feature = "serial_log")]
    serial_print(args);
    #[cfg(feature = "vga_log")]
    vga_print(args);
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
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace))
}
