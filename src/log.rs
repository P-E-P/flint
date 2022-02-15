use lazy_static::lazy_static;
use serial::Serial;

lazy_static! {
    static ref OUT: Serial = Serial::default();
}

pub fn printk(msg: &str) {
    OUT.write_string(msg);
}
