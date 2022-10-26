use core::arch::asm;

pub mod idt;
pub mod pic;
pub mod pit8254;

/// Disable maskable external interrupts.
///
/// # Safety
///
/// Disabling interrupt will inhibit some interrupts and may result in
/// unexpected behaviors.
pub unsafe fn disable() {
    asm!("cli");
}

/// Enable maskable external interrupts.
///
/// # Safety
///
/// Enabling interrupts requires a fully functional interrupt handling
/// implementation, it may result with processor reset if something
/// hasn't been set correctly.
pub unsafe fn enable() {
    asm!("sti");
}

pub fn setup() {
    unsafe {
        idt::setup_idt();
        enable();
    }
}
