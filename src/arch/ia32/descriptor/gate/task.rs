use super::PrivilegeLevel;
use crate::arch::ia32::selector::SegmentSelector;
use configuration::Configuration;

mod configuration;

#[must_use]
#[derive(Default, Copy, Clone)]
#[repr(C, packed)]
pub struct TaskGate {
    _reserved_1: u16,
    tss_segment_selector: SegmentSelector,
    configuration: Configuration,
    _reserved_2: u16,
}

impl TaskGate {
    pub fn new(tss_segment_selector: SegmentSelector) -> Self {
        TaskGate {
            configuration: Configuration::default().present(true),
            tss_segment_selector,
            ..Default::default()
        }
    }

    pub fn privilege_level(self, level: PrivilegeLevel) -> Self {
        Self {
            configuration: self.configuration.privilege_level(level),
            ..self
        }
    }
}
