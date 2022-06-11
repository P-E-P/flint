pub mod interrupts;
pub mod io;
pub mod mm;
pub mod ia32;
#[cfg(target_arch = "x86_64")]
pub mod ia32e;
#[cfg(target_arch = "x86")]
use ia32::*;
#[cfg(target_arch = "x86_64")]
use ia32e::*;

trait InOut {
    unsafe fn in_reg(address: u16) -> Self;
    unsafe fn out_reg(address: u16, value: Self);
}

impl InOut for u8 {
    unsafe fn in_reg(address: u16) -> Self {
        in_byte(address)
    }

    unsafe fn out_reg(address: u16, value: Self) {
        out_byte(address, value);
    }
}

impl InOut for u16 {
    unsafe fn in_reg(address: u16) -> Self {
        in_word(address)
    }

    unsafe fn out_reg(address: u16, value: Self) {
        out_word(address, value);
    }
}

impl InOut for u32 {
    unsafe fn in_reg(address: u16) -> Self {
        in_double_word(address)
    }

    unsafe fn out_reg(address: u16, value: Self) {
        out_double_word(address, value);
    }
}

/// A spinloop function that accepts a loop condition and will loop over arch
/// specific instructions.
pub fn spin_loop<F>(loop_condition: F)
where
    F: Fn() -> bool,
{
    while loop_condition() {
        pause();
    }
}

pub fn endless() -> ! {
    loop {
        halt();
    }
}
