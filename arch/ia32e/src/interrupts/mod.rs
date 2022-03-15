use core::arch::asm;

pub mod idt;

/// Disable maskable external interrupts.
pub unsafe fn disable() {
    asm!("cli");
}

/// Enable maskable external interrupts.
pub unsafe fn enable() {
    asm!("sti");
}

pub fn setup() {
    unsafe {
        idt::setup_idt();
        //enable();
    }
}
