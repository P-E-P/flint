use crate::selector::SegmentSelector;
use crate::PrivilegeLevel;
use lower::Lower;
use upper::Upper;

mod lower;
mod upper;

#[must_use]
#[derive(Default, Copy, Clone)]
#[repr(C, packed)]
pub struct TaskGate {
    pub upper: Upper,
    pub lower: Lower,
}

impl TaskGate {
    pub fn new(tss_segment_selector: SegmentSelector) -> Self {
        TaskGate {
            lower: Lower::default().tss_segment_selector(tss_segment_selector.into()),
            upper: Upper::default().present(1),
        }
    }

    pub fn privilege_level(self, level: PrivilegeLevel) -> Self {
        Self {
            upper: self.upper.privilege_level(level as u32),
            ..self
        }
    }
}
