//! A module containing the implementation and tests for the [`TaskGate`]
//! structure along some utility types revolving around those.
use super::PrivilegeLevel;
use crate::arch::ia32::selector::SegmentSelector;
use configuration::Configuration;

mod configuration;

/// Task gate structure that can be converted into a generic gate structure
/// once set up.
#[must_use]
#[derive(Default, Copy, Clone)]
#[repr(C, packed)]
pub struct TaskGate {
    /// Reserved field, it should not be used as it may have a meaning in the
    /// future.
    _reserved_1: u16,
    /// The segment selector pointing to the tss segment that should be used.
    tss_segment_selector: SegmentSelector,
    /// The descriptor configuration.
    configuration: Configuration,
    /// Reserved field, it should not be used as it may have a meaning in the
    /// future.
    _reserved_2: u16,
}

impl TaskGate {
    /// Creates a new [`TaskGate`] structure from a given task state segment
    /// segment selector.
    ///
    /// # Arguments
    ///
    /// * `tss_segment_selector` - The segment selector used by the gate.
    ///
    /// # Note
    ///
    /// The present bit will be enabled when using this constructor.
    pub fn new(tss_segment_selector: SegmentSelector) -> Self {
        TaskGate {
            configuration: Configuration::default().present(true),
            tss_segment_selector,
            ..Default::default()
        }
    }

    /// Change a [`TaskGate`]'s privilege level by another one.
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use bit_field::BitField;

    #[test_case]
    fn structure_size() {
        use core::mem::size_of;
        assert_eq!(size_of::<TaskGate>(), 8);
    }

    #[test_case]
    fn present() {
        let tgate = TaskGate::new(SegmentSelector::default());

        assert_eq!(
            unsafe { core::mem::transmute::<TaskGate, u64>(tgate) }.get_bit(47),
            true
        )
    }
}
