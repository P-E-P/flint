use super::lower::Lower;
use super::{GateSize, PrivilegeLevel};
use crate::selector::SegmentSelector;
use upper::Upper;

mod upper;

#[must_use]
#[derive(Default, Copy, Clone)]
#[repr(C, packed)]
pub struct TrapGate {
    pub upper: Upper,
    pub lower: Lower,
}

impl TrapGate {
    pub fn new(offset: u32, segment_selector: SegmentSelector) -> Self {
        let offset_15_0 = offset & 0xFFFF;
        let offset_31_16 = offset >> 16;
        TrapGate {
            lower: Lower::default()
                .offset_low(offset_15_0)
                .segment_selector(segment_selector.into()),
            upper: Upper::default().offset_high(offset_31_16).present(1),
        }
    }

    pub fn privilege_level(self, level: PrivilegeLevel) -> Self {
        Self {
            upper: self.upper.privilege_level(level as u32),
            ..self
        }
    }

    pub fn size(self, size: GateSize) -> Self {
        Self {
            upper: self.upper.size(size as u32),
            ..self
        }
    }
}
