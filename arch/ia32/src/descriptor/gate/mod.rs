pub use super::PrivilegeLevel;
use interrupt::InterruptGate;
use task::TaskGate;
use trap::TrapGate;

pub mod interrupt;
pub mod task;
pub mod trap;

mod lower;

#[repr(u8)]
pub enum GateSize {
    Gate16Bits = 0,
    Gate32Bits = 1,
}

#[repr(C, packed)]
pub struct Gate(u64);

macro_rules! gate {
    ($g: expr) => {
        Self(u64::from($g.upper.0) << 32 | u64::from($g.lower.0))
    };
}

impl From<TaskGate> for Gate {
    fn from(gate: TaskGate) -> Self {
        gate!(gate)
    }
}

impl From<InterruptGate> for Gate {
    fn from(gate: InterruptGate) -> Self {
        gate!(gate)
    }
}

impl From<TrapGate> for Gate {
    fn from(gate: TrapGate) -> Self {
        gate!(gate)
    }
}
