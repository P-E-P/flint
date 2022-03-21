#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(flint::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use flint::klog;

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("Kernel Panic!:\n{}", info);
    arch::endless();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    flint::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    klog::init().ok();

    #[cfg(test)]
    test_main();

    flint::setup();

    arch::endless();
}
