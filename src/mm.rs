use crate::arch::mm;
use log::trace;

pub fn setup() {
    trace!("Setting up memory");
    mm::setup();
}
