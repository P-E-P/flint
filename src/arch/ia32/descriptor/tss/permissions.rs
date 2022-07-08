//! A module containing the implementation and tests for the [`Permissions`]
//! structure.
use crate::arch::ia32::PrivilegeLevel;
use bit_field::BitField;
use core::fmt;

/// A structure representing the bits 8 to 15 from a
/// [TssDescriptor](super::TssDescriptor) structure.
///
/// It gathers the following fields:
/// - Segment type (TYPE)
/// - Descriptor privilege level (DPL)
/// - Present bit (P)
#[derive(Copy, Clone)]
pub struct Permissions(u8);

/// The set of all field offsets for the [`Permissions`] structure.
mod offset {
    /// Offset of the busy bit (B) within the [Permissions](super::Permissions)
    /// structure
    pub const BUSY: usize = 1;

    /// Bounds of the descriptor privilege level (DPL) bits within the
    /// [Permissions](super::Permissions) structure.
    pub mod privilege_level {
        /// Lower bit offset.
        pub const LOWER: usize = 5;

        /// Upper bit offset.
        pub const UPPER: usize = 6;
    }

    /// Offset of the present bit (P) within the
    /// [Permissions](super::Permissions) structure.
    pub const PRESENT: usize = 7;
}

impl Default for Permissions {
    fn default() -> Self {
        // We set up tss specific bits as specified by the Intel manual.
        // Those bits will allow the processor to identify the kind of
        // descriptor.
        Permissions(0b00001001)
    }
}

impl Permissions {
    /// Change a [`Permissions`]'s busy bit.
    ///
    /// # Arguments
    ///
    /// * `busy` - The desired bit value, `true` for bit value 1 and `false`
    /// for bit value 0.
    pub fn busy(self, busy: bool) -> Self {
        use offset::BUSY;
        Self(*self.0.clone().set_bit(BUSY, busy))
    }

    /// Change a [`Permissions`]'s privilege level by another one.
    ///
    /// # Arguments
    ///
    /// * `level` - The desired [`PrivilegeLevel`] value.
    pub fn privilege_level(self, level: PrivilegeLevel) -> Self {
        use offset::privilege_level::{LOWER, UPPER};
        Self(*self.0.clone().set_bits(LOWER..=UPPER, level.into()))
    }

    /// Change a [`Permissions`]'s present bit.
    ///
    /// # Arguments
    ///
    /// * `present` - The desired bit value, `true` for bit value 1 and `false`
    /// for bit value 0.
    pub fn present(self, present: bool) -> Self {
        use offset::PRESENT;
        Self(*self.0.clone().set_bit(PRESENT, present))
    }
}

impl fmt::Display for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use offset::{privilege_level, BUSY, PRESENT};
        write!(
            f,
            "Busy: {}\nDPL: {}\nPresent: {}",
            self.0.get_bit(BUSY),
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
        assert_eq!(size_of::<Permissions>(), 1);
    }

    #[test_case]
    fn default_values() {
        let perm = Permissions::default();
        assert_eq!(perm.0.get_bits(..5), 0b01001);
    }

    #[test_case]
    fn present() {
        let perm = Permissions::default().present(true);
        assert_eq!(perm.0.get_bit(7), true);
    }

    #[test_case]
    fn privilege_kernel() {
        use PrivilegeLevel::Kernel;
        let perm = Permissions::default().privilege_level(Kernel);
        assert_eq!(perm.0.get_bits(5..=6), 0);
    }

    #[test_case]
    fn privilege_userland() {
        use PrivilegeLevel::Userland;
        let perm = Permissions::default().privilege_level(Userland);
        assert_eq!(perm.0.get_bits(5..=6), 3);
    }

    #[test_case]
    fn busy() {
        let perm = Permissions::default().busy(true);
        assert_eq!(perm.0.get_bit(1), true);
    }

    #[test_case]
    fn not_busy() {
        let perm = Permissions::default().busy(false);
        assert_eq!(perm.0.get_bit(1), false);
    }
}
