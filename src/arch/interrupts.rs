#[cfg(target_arch = "x86")]
use crate::arch::ia32::interrupts;
#[cfg(target_arch = "x86_64")]
use crate::arch::ia32e::interrupts;

pub fn setup() {
    interrupts::setup();
}
