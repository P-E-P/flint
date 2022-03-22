//! Utility module for any test function, trait or structure.

use arch::io::port::Port;
use arch::io::register::WriteRegister;
use core::panic::PanicInfo;

/// Print panic infos and tries to quit Qemu with the [`QemuExitCode::Failed`]
/// error code.
pub fn panic_handler(info: &PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    arch::endless();
}

/// Trait representing a test object that could be run.
pub trait Testable {
    /// Run the test.
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        print!("{}...\t", core::any::type_name::<T>());
        self();
        println!("[ok]");
    }
}

/// Test runner, print miscellanous informations about the tests (number, time)
/// and run all tests. Terminates either on the first failing test or by
/// exiting qemu with a [`QemuExitCode::Success`] status code after all tests
/// ran.
pub fn runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

/// Set of every Qemu exit codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    /// Successful run status exit code.
    Success = 0x10,
    /// Failed run status exit code.
    Failed = 0x11,
}

/// Call Qemu IO mapped debug exit.
pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        Port::<u32>::new(0xf4).write(exit_code as u32);
    }
}
