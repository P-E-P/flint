use core::fmt::Write;
use core::fmt;
#[cfg(feature = "serial_log")]
use serial;

pub fn print_fmt(args: fmt::Arguments) {
    #[cfg(feature = "serial_log")]
    serial::get_default().write_fmt(args.clone());
}

#[macro_export]
macro_rules! printk {
    ($($args: tt)*) => {
        crate::log::print_fmt(format_args!($($args)*))
    }
}
