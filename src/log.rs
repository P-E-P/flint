use lazy_static::lazy_static;
use core::fmt::Write;
use core::fmt;
use serial::Serial;

static mut OUT: Option<Serial> = None;

fn get_serial() -> &'static mut Serial {
    unsafe {
        if OUT.is_none() {
            OUT = Some(Serial::default());
        }
        OUT.as_mut().unwrap()
    }
}

pub fn print_fmt(args: fmt::Arguments) {
    get_serial().write_fmt(args);
}

#[macro_export]
macro_rules! printk {
    ($($args: tt)*) => {
        crate::log::print_fmt(format_args!($($args)*))
    }
}
