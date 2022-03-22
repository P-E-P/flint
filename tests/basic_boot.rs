#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(flint::test::runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use flint::print;

#[test_case]
fn dummy_integration_test() {
    assert_eq!(1, 1);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    arch::endless();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    arch::endless();
}
