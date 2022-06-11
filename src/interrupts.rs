use crate::arch::interrupts;
use log::trace;

pub fn setup() {
    trace!("Setting up interrupts");
    interrupts::setup();
}
