//! A module containing the implementation and tests for the Gate structure.
use crate::arch::ia32e::{selector::SegmentSelector, PrivilegeLevel};
use bit_field::BitField;
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
#[derive(Default, Copy, Clone)]
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
    pub fn new(offset: u64, segment_selector: SegmentSelector) -> Self {
        Self {
            offset_15_0: offset.get_bits(0..16).try_into().unwrap(),
            offset_31_16: offset.get_bits(16..32).try_into().unwrap(),
            offset_63_32: offset.get_bits(32..64).try_into().unwrap(),
            configuration: Configuration::default().present(true),
            segment_selector,
            ..Default::default()
        }
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

impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Display trait not implemented for ia32e interrupt gates")
    }
}
