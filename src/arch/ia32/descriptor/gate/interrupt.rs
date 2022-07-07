//! A module containing the implementation and the test suite along revolving
//! types around the [`InterruptGate`] structure.
use super::{GateSize, PrivilegeLevel};
use crate::arch::ia32::selector::SegmentSelector;
use bit_field::BitField;
use configuration::Configuration;

mod configuration;

/// Interrupt gate structure that can be converted into a generic gate structure
/// once set up.
#[must_use]
#[derive(Default, Copy, Clone)]
#[repr(C, packed)]
pub struct InterruptGate {
    /// Bits 0 to 15 of the interrupt routine address.
    offset_15_0: u16,
    /// The segment selector pointing to the segment to use.
    segment_selector: SegmentSelector,
    /// The descriptor configuration.
    configuration: Configuration,
    /// Bits 16 to 31 of the interrupt routine address.
    offset_31_16: u16,
}

impl InterruptGate {
    /// Creates a new [`InterruptGate`] from an interrupt routine address
    /// and a segment descriptor.
    ///
    /// # Arguments
    ///
    /// * `offset` - The interrupt routine address.
    /// * `segment_selector` - The segment selector used for the interruption.
    ///
    /// # Note
    ///
    /// The present bit will be enabled when using this constructor.
    pub fn new(offset: u32, segment_selector: SegmentSelector) -> Self {
        InterruptGate {
            offset_15_0: offset.get_bits(0..16).try_into().unwrap(),
            offset_31_16: offset.get_bits(16..32).try_into().unwrap(),
            configuration: Configuration::default().present(true),
            segment_selector,
        }
    }

    /// Change a [`InterruptGate`]'s privilege level by another one.
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

    /// Change the interrupt gate size
    ///
    /// # Arguments
    ///
    /// * `size` - The desired [`GateSize`].
    pub fn size(self, size: GateSize) -> Self {
        Self {
            configuration: self.configuration.size(size),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn structure_size() {
        use core::mem::size_of;
        assert_eq!(size_of::<InterruptGate>(), 8);
    }

    #[test_case]
    fn present() {
        let igate = InterruptGate::new(0, SegmentSelector::default());

        assert_eq!(
            unsafe { core::mem::transmute::<InterruptGate, u64>(igate) }.get_bit(47),
            true
        )
    }
}
