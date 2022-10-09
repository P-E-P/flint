use crate::arch::ia32::address::VirtualAddress;
use core::fmt;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct InterruptStackFrame {
    /// Next or faulting instruction
    pub rip: VirtualAddress,
    /// Code segment selector (CS)
    pub code_segment: u64,
    /// CPU flags
    pub rflags: u64,
    /// Stack pointer (RSP)
    pub stack_pointer: VirtualAddress,
    /// Stack segment (SS)
    pub stack_segment: u64,
}

impl fmt::Display for InterruptStackFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Interrupt Stack Frame:\nRIP: {}\nCS: {:X}\nRLAGS: {:X}\nRSP: {}\nSS: {:X}\n",
            self.rip,
            self.code_segment,
            self.rflags,
            self.stack_pointer,
            self.stack_segment
        )
    }
}
