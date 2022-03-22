#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod interrupts;
mod mm;
pub mod vga;
#[macro_use]
pub mod klog;
mod qemu;
pub mod test;

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test::panic_handler(info)
}

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
