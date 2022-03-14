use super::lower;
use super::{GateSize, PrivilegeLevel};

mod upper;

#[derive(Default, Copy, Clone)]
pub struct TrapGate {
    pub upper: upper::Upper,
    pub lower: lower::Lower,
}

impl TrapGate {
    pub fn new(offset: u32, segment_selector: u16) -> Self {
        let offset_low = offset & 0xFFFF;
        let offset_high = offset >> 16;
        TrapGate {
            lower: lower::Lower::default()
                .offset_low(offset_low)
                .segment_selector(segment_selector.into()),
            upper: upper::Upper::default().offset_high(offset_high).present(1),
        }
    }

    pub fn privilege_level(self, level: PrivilegeLevel) -> Self {
        Self {
            lower: self.lower,
            upper: self.upper.privilege_level(level as u32),
        }
    }

    pub fn size(self, size: GateSize) -> Self {
        Self {
            lower: self.lower,
            upper: self.upper.size(size as u32),
        }
    }
}
