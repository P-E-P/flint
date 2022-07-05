use crate::arch::ia32e::{selector::SegmentSelector, PrivilegeLevel};
use configuration::Configuration;
use bit_field::BitField;
use core::fmt;

mod configuration;

#[repr(u16)]
pub enum Kind {
    Interrupt = 0xe,
    Trap = 0xf,
}

impl From<Kind> for u16 {
    fn from(value: Kind) -> Self {
        value as u16
    }
}

#[must_use]
#[derive(Default, Copy, Clone)]
#[repr(C, packed)]
pub struct Gate {
    offset_15_0: u16,
    segment_selector: SegmentSelector,
    configuration: Configuration,
    offset_31_16: u16,
    offset_63_32: u32,
    _reserved: u32,
}

impl Gate {
    /// Creates a new interrupt/trap [`Gate`] from a given offset and segment
    /// selector.
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
    pub fn present(self, value: bool) -> Self {
        Self {
            configuration: self.configuration.present(value.into()),
            ..self
        }
    }

    /// Set the type of [`Gate`].
    pub fn kind(self, kind: Kind) -> Self {
        Self {
            configuration: self.configuration.kind(kind),
            ..self
        }
    }

    /// Set the privilege level of the [`Gate`] descriptor.
    pub fn privilege_level(self, level: PrivilegeLevel) -> Self {
        Self {
            configuration: self.configuration.privilege_level(level),
            ..self
        }
    }

    /// Set the  interrupt stack table.
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
