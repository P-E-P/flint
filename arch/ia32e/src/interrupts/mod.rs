use core::arch::asm;

pub mod descriptor;

/// Disable maskable external interrupts.
pub unsafe fn disable() {
    asm!("cli");
}

/// Enable maskable external interrupts.
pub unsafe fn enable() {
    asm!("sti");
}
