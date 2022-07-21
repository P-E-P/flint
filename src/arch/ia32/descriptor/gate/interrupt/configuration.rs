//! A module containing the implementation and tests for the [`Configuration`]
//! structure.
use super::{GateSize, PrivilegeLevel};
use crate::utils::bitfield::BitField;
use core::fmt;

/// A structure representing the bits 0 to 15 from an
/// [InterruptGate](super::InterruptGate) structure.
///
/// It gathers the following fields:
/// - Size (D)
/// - Descriptor privilege level (DPL)
/// - Present bit (P)
#[derive(Copy, Clone)]
pub struct Configuration(u16);

/// The set of all field offsets for the [`Configuration`] structure.
mod offset {
    /// Offset of the size (D) bit within the
    /// [Configuration](super::Configuration) structure.
    pub const SIZE: usize = 11;

    /// Bounds of the descriptor privilege level (DPL) bits within the
    /// [Configuration](super::Configuration) structure.
    pub mod privilege_level {
        /// Lower bit offset.
        pub const LOWER: usize = 13;
        /// Upper bit offset.
        pub const UPPER: usize = 14;
    }

    /// Offset of the present bit (P) within the
    /// [Configuration](super::Configuration) structure.
    pub const PRESENT: usize = 15;
}

impl Default for Configuration {
    fn default() -> Self {
        // We set up interrupt gate specific bits as specified by the Intel
        // manual
        // Those bits will allow the processor to identify the kind of gate
        // used.
        Configuration(0x0600)
    }
}

impl Configuration {
    /// Change a [`Configuration`]'s gate size bits with another size.
    ///
    /// # Arguments
    ///
    /// * `size` - The desired [`GateSize`].
    pub fn size(self, size: GateSize) -> Self {
        use offset::SIZE;
        Self(self.0.set_bit(SIZE, size.into()))
    }

    /// Change a [`Configuration`]'s privilege level by another one.
    ///
    /// # Arguments
    ///
    /// * `level` - The desired [`PrivilegeLevel`] value.
    pub fn privilege_level(self, level: PrivilegeLevel) -> Self {
        use offset::privilege_level::{LOWER, UPPER};
        Self(self.0.set_bits(LOWER..=UPPER, u8::from(level).into()))
    }

    /// Change a [`Configuration`]'s present bit.
    ///
    /// # Arguments
    ///
    /// * `present` - The desired bit value, `true` for bit value 1 and `false`
    /// for bit value 0.
    pub fn present(self, present: bool) -> Self {
        use offset::PRESENT;
        Self(self.0.set_bit(PRESENT, present))
    }
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use offset::{privilege_level, PRESENT, SIZE};
        write!(
            f,
            "Size: {}\nDPL: {}\nPresent {}",
            self.0.get_bit(SIZE),
            self.0
                .get_bits(privilege_level::LOWER..=privilege_level::UPPER),
            self.0.get_bit(PRESENT)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn structure_size() {
        use core::mem::size_of;
        assert_eq!(size_of::<Configuration>(), 2);
    }

    #[test_case]
    fn default_value_type() {
        let conf = Configuration::default();
        assert_eq!(conf.0.get_bits(8..=10), 0b110);
    }

    #[test_case]
    fn default_value_zeroes() {
        let conf = Configuration::default();
        assert_eq!(conf.0.get_bits(5..=7), 0b000);
    }

    #[test_case]
    fn present() {
        let conf = Configuration::default().present(true);
        assert_eq!(conf.0.get_bit(15), true);
    }

    #[test_case]
    fn size_16bits() {
        use GateSize::Gate16Bits;
        let conf = Configuration::default().size(Gate16Bits);
        assert_eq!(conf.0.get_bit(11), false);
    }

    #[test_case]
    fn size_32bits() {
        use GateSize::Gate32Bits;
        let conf = Configuration::default().size(Gate32Bits);
        assert_eq!(conf.0.get_bit(11), true);
    }

    #[test_case]
    fn privilege_kernel() {
        use PrivilegeLevel::Kernel;
        let perm = Configuration::default().privilege_level(Kernel);
        assert_eq!(perm.0.get_bits(13..=14), 0);
    }

    #[test_case]
    fn privilege_userland() {
        use PrivilegeLevel::Userland;
        let perm = Configuration::default().privilege_level(Userland);
        assert_eq!(perm.0.get_bits(13..=14), 3);
    }
}
