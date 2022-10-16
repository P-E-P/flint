//! A module containing the implementation and tests for the Gate structure.
use crate::arch::ia32::address::VirtualAddress;
use crate::arch::ia32e::{selector::SegmentSelector, PrivilegeLevel};
use crate::utils::bitfield::*;
use configuration::Configuration;
use core::fmt;

mod configuration;

/// Type to determine whether a [`Gate`] is a trap gate or an interrupt gate.
#[repr(u16)]
pub enum Kind {
    /// Interrupt gate type.
    Interrupt = 0xe,
    /// Trap gate type.
    Trap = 0xf,
}

impl From<Kind> for u16 {
    fn from(value: Kind) -> Self {
        value as u16
    }
}

/// A gate descriptor structure that can be used to describe either a trap gate
/// or an interrupt gate. This structure can be used directly by the processor.
#[must_use]
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct Gate {
    /// Bits 0 to 15 of the interrupt routine address.
    offset_15_0: u16,
    /// The segment selector to use on interrupt/trap.
    segment_selector: SegmentSelector,
    /// The configuration of the gate descriptor.
    configuration: Configuration,
    /// Bits 16 to 31 of the interrupt routine address.
    offset_31_16: u16,
    /// Bits 32 to 63 of the interrupt routine address.
    offset_63_32: u32,
    /// Reserved bits.
    _reserved: u32,
}

impl Gate {
    /// Create a new null [`Gate`].
    pub const fn const_default() -> Self {
        Gate {
            /// Bits 0 to 15 of the interrupt routine address.
            offset_15_0: 0,
            /// The segment selector to use on interrupt/trap.
            segment_selector: SegmentSelector::const_default(),
            /// The configuration of the gate descriptor.
            configuration: Configuration::const_default(),
            /// Bits 16 to 31 of the interrupt routine address.
            offset_31_16: 0,
            /// Bits 32 to 63 of the interrupt routine address.
            offset_63_32: 0,
            /// Reserved bits.
            _reserved: 0,
        }
    }

    /// Creates a new interrupt/trap [`Gate`] from a given offset and segment
    /// selector.
    ///
    /// # Arguments
    ///
    /// * `base` - The segments base adress.
    /// * `limit` - The limit value for the segment descriptor.
    ///
    /// # Note
    ///
    /// The present bit will be enabled when using this constructor.
    fn new(offset: u64, segment_selector: SegmentSelector) -> Self {
        Self {
            offset_15_0: offset.get_bits(0..16).try_into().unwrap(),
            offset_31_16: offset.get_bits(16..32).try_into().unwrap(),
            offset_63_32: offset.get_bits(32..64).try_into().unwrap(),
            configuration: Configuration::default().present(true),
            segment_selector,
            ..Default::default()
        }
    }

    /// Creates a new interrupt [`Gate`] from a ['VirtualAddress'] and segment
    /// selector.
    ///
    /// # Arguments
    ///
    /// * `base` - The segments base adress.
    /// * `limit` - The limit value for the segment descriptor.
    ///
    /// # Note
    ///
    /// The present bit will be enabled when using this constructor.
    pub fn interrupt(offset: VirtualAddress, segment_selector: SegmentSelector) -> Self {
        Self::new(offset.into(), segment_selector).kind(Kind::Interrupt)
    }

    /// Creates a new trap [`Gate`] from a ['VirtualAddress'] and segment
    /// selector.
    ///
    /// # Arguments
    ///
    /// * `base` - The segments base adress.
    /// * `limit` - The limit value for the segment descriptor.
    ///
    /// # Note
    ///
    /// The present bit will be enabled when using this constructor.
    pub fn trap(offset: VirtualAddress, segment_selector: SegmentSelector) -> Self {
        Self::new(offset.into(), segment_selector).kind(Kind::Trap)
    }

    /// Set or clear the presence bit of the [`Gate`].
    ///
    /// # Arguments
    ///
    /// * `present` - The desired bit value. Use `true` for 1 and `false` for 0.
    pub fn present(self, value: bool) -> Self {
        Self {
            configuration: self.configuration.present(value.into()),
            ..self
        }
    }

    /// Set the type of [`Gate`].
    ///
    /// # Arguments
    ///
    /// * `kind` - The desired type of gate.
    pub fn kind(self, kind: Kind) -> Self {
        Self {
            configuration: self.configuration.kind(kind),
            ..self
        }
    }

    /// Set the privilege level of the [`Gate`] descriptor.
    ///
    /// # Arguments
    ///
    /// * `privilege_level` - The desired [`PrivilegeLevel`].
    pub fn privilege_level(self, level: PrivilegeLevel) -> Self {
        Self {
            configuration: self.configuration.privilege_level(level),
            ..self
        }
    }

    /// Set the  interrupt stack table.
    ///
    /// # Arguments
    ///
    /// * `ist` - The desired interrupt stack table.
    ///
    /// # Panics
    ///
    /// This function will panic if the ist value is too high and cannot fit
    /// the boundaries of the ist field.
    pub fn interrupt_stack_table(self, ist: u8) -> Self {
        Self {
            configuration: self.configuration.interrupt_stack_table(ist),
            ..self
        }
    }
}

impl Default for Gate {
    fn default() -> Self {
        Self::const_default()
    }
}

impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let configuration = self.configuration;
        let segment_selector = self.segment_selector;
        let offset: u64 = 0
            .set_bits(0..16, self.offset_15_0 as u64)
            .set_bits(16..32, self.offset_31_16 as u64)
            .set_bits(32..64, self.offset_63_32 as u64);
        write!(
            f,
            "Offset: {:X}\n{}\n{}",
            offset, configuration, segment_selector
        )
    }
}
