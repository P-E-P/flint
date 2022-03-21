use core::arch::asm;
pub use ia32::interrupts::{disable, enable};

pub mod idt;

pub fn setup() {
    unsafe {
        idt::setup_idt();
        //enable();
    }
}
