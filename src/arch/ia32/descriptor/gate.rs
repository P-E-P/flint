pub use crate::arch::ia32::PrivilegeLevel;
use core::fmt;
use core::mem::transmute;
use interrupt::InterruptGate;
use task::TaskGate;
use trap::TrapGate;

pub mod interrupt;
pub mod lower;
pub mod task;
pub mod trap;

#[repr(u8)]
pub enum GateSize {
    Gate16Bits = 0,
    Gate32Bits = 1,
}

impl From<GateSize> for bool {
    fn from(value: GateSize) -> Self {
        match value {
            GateSize::Gate16Bits => false,
            GateSize::Gate32Bits => true,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct Gate(u64);

impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:08X?};{:08X?}", self.0 >> 32, self.0 & 0xffffffff)
    }
}

macro_rules! gate {
    ($g: expr) => {
        Self(u64::from($g.upper.0) << 32 | u64::from($g.lower.0))
    };
}

impl From<TaskGate> for Gate {
    fn from(gate: TaskGate) -> Self {
        unsafe { Self(transmute::<TaskGate, u64>(gate)) }
    }
}

impl From<InterruptGate> for Gate {
    fn from(gate: InterruptGate) -> Self {
        unsafe { Self(transmute::<InterruptGate, u64>(gate)) }
    }
}

impl From<TrapGate> for Gate {
    fn from(gate: TrapGate) -> Self {
        unsafe { Self(transmute::<TrapGate, u64>(gate)) }
    }
}
