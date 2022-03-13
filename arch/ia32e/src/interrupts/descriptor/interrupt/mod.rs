use super::PrivilegeLevel;

mod lower;
mod upper;

#[derive(Default, Copy, Clone)]
pub struct InterruptGate {
    pub upper: upper::Upper,
    pub lower: lower::Lower,
}

pub enum GateSize {
    Gate16Bits = 0,
    Gate32Bits = 1,
}

impl InterruptGate {
    pub fn new(offset: u32, segment_selector: u16) -> Self {
        let offset_low = offset & 0xFFFF;
        let offset_high = offset >> 16;
        InterruptGate {
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
