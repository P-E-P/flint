//! Utility module for any test function, trait or structure.

use crate::qemu::{self, ExitCode};
use core::panic::PanicInfo;

/// Print panic infos and tries to quit Qemu with the [`ExitCode::Failed`]
/// error code.
pub fn panic_handler(info: &PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}\n", info);
    qemu::exit(ExitCode::Failed);
    crate::arch::endless();
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
/// exiting qemu with a [`ExitCode::Success`] status code after all tests
/// ran.
pub fn runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    qemu::exit(ExitCode::Success);
}
