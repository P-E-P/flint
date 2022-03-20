use super::PrivilegeLevel;

mod lower;
mod upper;

#[derive(Default, Copy, Clone)]
#[repr(C, packed)]
pub struct TaskGate {
    pub upper: upper::Upper,
    pub lower: lower::Lower,
}

impl TaskGate {
    pub fn new(tss_segment_selector: u32) -> Self {
        TaskGate {
            lower: lower::Lower::default().tss_segment_selector(tss_segment_selector),
            upper: upper::Upper::default().present(1),
        }
    }

    pub fn privilege_level(self, level: PrivilegeLevel) -> Self {
        Self {
            upper: self.upper.privilege_level(level as u32),
            ..self
        }
    }
}
