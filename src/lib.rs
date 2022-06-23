#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "test_main"]

/// Offset the bits of the given identifier by it's offset.
#[macro_export]
macro_rules! offset {
    ($n: ident) => {
        bits::$n << offsets::$n
    };
}

/// Clear the flag bits `n` from a number `s` and set those bits to the new
/// `v` value.
#[macro_export]
macro_rules! setbits {
    ($s: expr, $v: expr, $n: ident) => {
        // - Ensure the value does not overflow
        // - Clear previous flag
        // - Set new flag
        ($s & !flags::$n) | (($v & bits::$n) << offsets::$n)
    };
}

mod interrupts;
mod mm;
pub mod vga;
#[macro_use]
pub mod klog;
pub mod qemu;
pub mod serial;
pub mod test;
#[macro_use]
pub mod arch;

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test::panic_handler(info)
}

/// Test specific start function.
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    klog::init().ok();
    test_main();
    arch::endless();
}

pub fn setup() {
    mm::setup();
    interrupts::setup();
}
