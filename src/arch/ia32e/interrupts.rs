pub use crate::arch::ia32::interrupts::{disable, enable};

pub mod idt;
pub mod frame;

pub fn setup() {
    unsafe {
        idt::setup_idt();
        enable();
    }
}
