#[cfg(target_arch = "x86")]
use crate::arch::ia32::mm;
#[cfg(target_arch = "x86_64")]
use crate::arch::ia32e::mm;

pub fn setup() {
    mm::setup();
}
