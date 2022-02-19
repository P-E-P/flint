#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use arch::outdw;

mod vga;
#[macro_use]
mod klog;

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/// Call Qemu IO mapped debug exit.
pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        outdw(exit_code as u32, 0xf4);
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log::error!("Kernel Panic!:\n{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    klog::init().ok();

    #[cfg(test)]
    test_main();

    loop {}
}
