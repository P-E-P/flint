use crate::arch::io::port::Port;
use crate::arch::io::register::WriteRegister;

/// Set of every Qemu exit codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ExitCode {
    /// Successful run status exit code.
    Success = 0x10,
    /// Failed run status exit code.
    Failed = 0x11,
}

/// Call Qemu IO mapped debug exit.
pub fn exit(exit_code: ExitCode) {
    unsafe {
        Port::<u32>::new(0xf4).write(exit_code as u32);
    }
}
