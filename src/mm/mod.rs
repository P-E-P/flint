use log::trace;
use arch::mm;

pub fn setup() {
    trace!("Setting up memory");
    mm::setup();
}
