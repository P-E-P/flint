#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(const_trait_impl)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "test_main"]

mod interrupts;
mod mm;
pub mod vga;
#[macro_use]
pub mod klog;
pub mod qemu;
pub mod serial;
pub mod test;
pub mod utils;
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
