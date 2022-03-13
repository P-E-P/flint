use interrupt::InterruptGate;
use task::TaskGate;
use trap::TrapGate;

pub mod interrupt;
pub mod trap;
pub mod task;

#[repr(u8)]
pub enum PrivilegeLevel {
    Kernel = 0,
    Unused1 = 1,
    Unused2 = 2,
    Userland = 3,
}

pub enum GateSize {
    Gate16Bits = 0,
    Gate32Bits = 1,
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

impl From<TrapGate> for Gate {
    fn from(gate: TrapGate) -> Self {
        Self(u64::from(gate.upper.0) << 32 | u64::from(gate.lower.0))
    }
}
