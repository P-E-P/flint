mod lower;
mod upper;

#[repr(u8)]
pub enum PrivilegeLevel {
    Kernel = 0,
    Unused1 = 1,
    Unused2 = 2,
    Userland = 3,
}

#[derive(Default, Copy, Clone)]
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
            lower: self.lower,
            upper: self.upper.privilege_level(level as u32),
        }
    }
}
