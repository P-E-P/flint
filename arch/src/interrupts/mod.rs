#[cfg(target_arch = "x86")]
use ia32::interrupts;
#[cfg(target_arch = "x86_64")]
use ia32e::interrupts;

pub fn setup() {
    interrupts::setup();
}
