use core::arch::asm;

/// Disable maskable external interrupts.
pub unsafe fn disable() {
    asm!("cli");
}

/// Enable maskable external interrupts.
pub unsafe fn enable() {
    asm!("sti");
}
