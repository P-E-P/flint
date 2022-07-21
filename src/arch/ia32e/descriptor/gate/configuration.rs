//! A module containing the implementation and tests for the [`Configuration`]
//! structure.
use super::{Kind, PrivilegeLevel};
use crate::utils::bitfield::BitField;
use core::fmt;

/// A structure representing the bits 0 to 15 from a
/// [Gate](super::Gate) structure.
///
/// It gathers the following fields:
/// - Interrupt stack table (IST)
/// - Gate type (TYPE)
/// - Descriptor privilege level (DPL)
/// - Present bit (P)
#[derive(Default, Copy, Clone)]
pub struct Configuration(u16);

/// The set of all field offsets for the [`Configuration`] structure.
mod offset {
    /// Bounds of the ist bits within the [Configuration](super::Configuration)
    /// structure.
    pub mod ist {
        pub const LOWER: usize = 0;
        pub const UPPER: usize = 2;
    }

    /// Bounds of the gate kind (TYPE) bits for the
    /// [Configuration](super::Configuration) structure.
    pub mod kind {
        pub const LOWER: usize = 8;
        pub const UPPER: usize = 11;
    }

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

    /// Change a [`Configuration`]'s interrupt stack table value.
    ///
    /// # Arguments
    ///
    /// * `ist` - The desired interrupt stack table value.
    ///
    /// # Panics
    ///
    /// This method will panic if the given limit value is greater than 15
    /// because [`Configuration`] structure hold only bits 16 to 19 of the
    /// segment descriptor structure.
    pub fn interrupt_stack_table(self, ist: u8) -> Self {
        use offset::ist::{LOWER, UPPER};
        Self(self.0.set_bits(LOWER..=UPPER, ist.into()))
    }

    /// Change the [`Configuration`]'s type.
    ///
    /// # Arguments
    ///
    /// * `kind` - The desired gate type.
    pub fn kind(self, kind: Kind) -> Self {
        use offset::kind::{LOWER, UPPER};
        Self(self.0.set_bits(LOWER..=UPPER, kind.into()))
    }
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use offset::{ist, kind, privilege_level, PRESENT};
        write!(
            f,
            "DPL: {}\nPresent {}\nType:{}\nIST:{}",
            self.0
                .get_bits(privilege_level::LOWER..=privilege_level::UPPER),
            self.0.get_bit(PRESENT),
            self.0.get_bits(kind::LOWER..=kind::UPPER),
            self.0.get_bits(ist::LOWER..=ist::UPPER)
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
    fn default_value_zeroes() {
        let conf = Configuration::default();
        assert_eq!(conf.0.get_bits(3..=7), 0);
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

    #[test_case]
    fn ist_value() {
        let perm = Configuration::default().interrupt_stack_table(7);
        assert_eq!(perm.0.get_bits(0..=2), 7);
    }
}
