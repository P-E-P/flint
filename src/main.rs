#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use arch::io::port::Port;
use arch::io::register::WriteRegister;
use core::panic::PanicInfo;

mod vga;
#[macro_use]
mod klog;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where T: Fn(),
{
    fn run(&self) {
        print!("{}...\t", core::any::type_name::<T>());
        self();
        println!("[ok]");
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/// Call Qemu IO mapped debug exit.
pub fn exit_qemu(exit_code: QemuExitCode) {
    Port::<u32>::new(0xf4).write(exit_code as u32);
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
