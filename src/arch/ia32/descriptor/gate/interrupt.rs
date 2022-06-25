use super::{GateSize, PrivilegeLevel};
use crate::arch::ia32::selector::SegmentSelector;
use bit_field::BitField;
use configuration::Configuration;

mod configuration;

#[must_use]
#[derive(Default, Copy, Clone)]
#[repr(C, packed)]
pub struct InterruptGate {
    offset_15_0: u16,
    segment_selector: SegmentSelector,
    configuration: Configuration,
    offset_31_16: u16,
}

impl InterruptGate {
    pub fn new(offset: u32, segment_selector: SegmentSelector) -> Self {
        InterruptGate {
            offset_15_0: offset.get_bits(0..16).try_into().unwrap(),
            offset_31_16: offset.get_bits(16..32).try_into().unwrap(),
            configuration: Configuration::default().present(true),
            segment_selector,
        }
    }

    pub fn privilege_level(self, level: PrivilegeLevel) -> Self {
        Self {
            configuration: self.configuration.privilege_level(level),
            ..self
        }
    }

    pub fn size(self, size: GateSize) -> Self {
        Self {
            configuration: self.configuration.size(size),
            ..self
        }
    }
}
