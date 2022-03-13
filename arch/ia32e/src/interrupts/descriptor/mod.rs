use interrupt::InterruptGate;
use task::TaskGate;

pub mod interrupt;
pub mod task;

#[repr(u8)]
pub enum PrivilegeLevel {
    Kernel = 0,
    Unused1 = 1,
    Unused2 = 2,
    Userland = 3,
}

#[repr(C, packed)]
pub struct Gate(u64);

impl From<TaskGate> for Gate {
    fn from(gate: TaskGate) -> Self {
        Self(u64::from(gate.upper.0) << 32 | u64::from(gate.lower.0))
    }
}

impl From<InterruptGate> for Gate {
    fn from(gate: InterruptGate) -> Self {
        Self(u64::from(gate.upper.0) << 32 | u64::from(gate.lower.0))
    }
}
