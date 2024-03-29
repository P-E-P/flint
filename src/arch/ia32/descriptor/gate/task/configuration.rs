//! A module containing the implementation and tests for the [`Configuration`]
//! structure.
use super::PrivilegeLevel;
use crate::utils::bitfield::*;
use core::fmt;

/// A structure representing the bits 0 to 15 from a
/// [TaskGate](super::TaskGate) structure.
///
/// It gathers the following fields:
/// - Descriptor privilege level (DPL)
/// - Present bit (P)
#[derive(Copy, Clone)]
pub struct Configuration(u16);

/// The set of all field offsets for the [`Configuration`] structure.
mod offset {
    /// Bounds of the descriptor privilege level (DPL) bits within the
    /// [Configuration](super::Configuration) structure.
    pub mod privilege_level {
        pub const LOWER: usize = 13;
        pub const UPPER: usize = 14;
    }

    /// Offset of the present bit (P) within the
    /// [Configuration](super::Configuration) structure.
    pub const PRESENT: usize = 15;
}

impl Default for Configuration {
    fn default() -> Self {
        // We set up task gate specific bits as specified by the Intel manual.
        // Those bits will allow the processor to identify the kind of gate
        // used.
        Configuration(0x0500)
    }
}

impl Configuration {
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
        use offset::{privilege_level, PRESENT};
        write!(
            f,
            "DPL: {}\nPresent {}",
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
    fn default_value() {
        let conf = Configuration::default();
        assert_eq!(conf.0.get_bits(8..=12), 0b00101);
    }

    #[test_case]
    fn present() {
        let conf = Configuration::default().present(true);
        assert_eq!(conf.0.get_bit(15), true);
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
