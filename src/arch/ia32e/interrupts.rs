pub use crate::arch::ia32::interrupts::{disable, enable};

pub mod frame;
pub mod idt;

pub fn setup() {
    unsafe {
        idt::setup_idt();
        enable();
    }
}
