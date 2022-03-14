#[cfg(target_arch = "x86")]
use ia32::mm;
#[cfg(target_arch = "x86_64")]
use ia32e::mm;

pub fn setup() {
    mm::setup();
}
