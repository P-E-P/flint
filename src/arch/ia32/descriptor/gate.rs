//! A module containing the different descriptors to use for an
//! interrupt descriptor table (IDT).
//!
//! This module brings 3 specialized gate structure:
//! - [`InterruptGate`]
//! - [`TrapGate`]
//! - [`TaskGate`]
//!
//! as well as a generic gate structure for the IDT itself.
//! All of those specialized gate structure can be morphed into the
//! generic one once completed.
pub use crate::arch::ia32::PrivilegeLevel;
use crate::utils::bitfield::BitField;
use core::fmt;
use core::mem::transmute;
use interrupt::InterruptGate;
use task::TaskGate;
use trap::TrapGate;

pub mod interrupt;
pub mod task;
pub mod trap;

/// The size of a gate, either 32 bits or 16bits.
#[repr(u8)]
pub enum GateSize {
    /// 16bits gate.
    Gate16Bits = 0,
    /// 32 bits gate.
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

/// Generic gate structure that can be used to build an interrupt descriptor
/// table.
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct Gate(u64);

impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let internal = self.0;
        write!(
            f,
            "Present:{}\nDPL:{}\nType:{}\nSegment selector:{:X}",
            internal.get_bit(47),
            internal.get_bits(45..=46),
            internal.get_bits(40..=44),
            internal.get_bits(16..=31)
        )
    }
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
