//! A module containing the implementation and tests for the [`Permissions`]
//! structure.
use super::{DescriptorType, SegmentType};
use crate::arch::ia32::PrivilegeLevel;
use crate::utils::bitfield::BitField;
use core::fmt;

/// A structure representing the bits 8 to 15 from a
/// [SegmentDescriptor](super::SegmentDescriptor) structure.
///
/// It gathers the following fields:
/// - Segment type (TYPE)
/// - Descriptor type (S)
/// - Descriptor privilege level (DPL)
/// - Present bit (P)
#[derive(Default, Copy, Clone)]
pub struct Permissions(u8);

/// The set of all field offsets for the [`Permissions`] structure.
mod offset {
    /// Bounds of the segment type (TYPE) bits within the
    /// [Permissions](super::Permissions) structure.
    pub mod segment_type {
        /// Upper bit.
        pub const UPPER: usize = 3;
    }

    /// Offset of the descriptor type (S) bit within the
    /// [Permissions](super::Permissions) structure.
    pub const DESC_TYPE: usize = 4;

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

impl Permissions {
    /// Change a [`Permissions`]'s segment type value.
    ///
    /// # Arguments
    ///
    /// * `seg_type` - The desired [`SegmentType`] value.
    pub fn segment_type(self, seg_type: SegmentType) -> Self {
        use offset::segment_type::UPPER;
        Self(self.0.set_bits(..=UPPER, seg_type.into()))
    }

    /// Change a [`Permissions`]'s descriptor type value.
    ///
    /// # Arguments
    ///
    /// * `desc_type` - The desired [`DescriptorType`] value.
    pub fn descriptor_type(self, desc_type: DescriptorType) -> Self {
        use offset::DESC_TYPE;
        Self(self.0.set_bit(DESC_TYPE, desc_type.into()))
    }

    /// Change a [`Permissions`]'s privilege level by another one.
    ///
    /// # Arguments
    ///
    /// * `level` - The desired [`PrivilegeLevel`] value.
    pub fn privilege_level(self, level: PrivilegeLevel) -> Self {
        use offset::privilege_level::{LOWER, UPPER};
        Self(self.0.set_bits(LOWER..=UPPER, level.into()))
    }

    /// Change a [`Permissions`]'s present bit.
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

impl fmt::Display for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use offset::{privilege_level, segment_type, DESC_TYPE, PRESENT};
        write!(
            f,
            "Type: {}\nS: {}\nDPL: {}\nPresent {}",
            self.0.get_bits(..=segment_type::UPPER),
            self.0.get_bit(DESC_TYPE),
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
        assert_eq!(perm.0, 0);
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
    fn descriptor_system() {
        use DescriptorType::System;
        let perm = Permissions::default().descriptor_type(System);
        assert_eq!(perm.0.get_bit(4), false);
    }

    #[test_case]
    fn descriptor_code_data() {
        use DescriptorType::CodeOrData;
        let perm = Permissions::default().descriptor_type(CodeOrData);
        assert_eq!(perm.0.get_bit(4), true);
    }

    #[test_case]
    fn segment_data() {
        let perm = Permissions::default().segment_type(SegmentType::Data {
            accessed: false,
            write: false,
            expand_down: false,
        });
        assert_eq!(perm.0.get_bit(3), false);
    }

    #[test_case]
    fn segment_code() {
        let perm = Permissions::default().segment_type(SegmentType::Code {
            accessed: false,
            read: false,
            conforming: false,
        });
        assert_eq!(perm.0.get_bit(3), true);
    }

    #[test_case]
    fn segment_data_accessed() {
        let perm = Permissions::default().segment_type(SegmentType::Data {
            accessed: true,
            write: false,
            expand_down: false,
        });
        assert_eq!(perm.0.get_bit(0), true);
    }

    #[test_case]
    fn segment_code_accessed() {
        let perm = Permissions::default().segment_type(SegmentType::Code {
            accessed: true,
            read: false,
            conforming: false,
        });
        assert_eq!(perm.0.get_bit(0), true);
    }

    #[test_case]
    fn segment_read() {
        let perm = Permissions::default().segment_type(SegmentType::Code {
            accessed: false,
            read: true,
            conforming: false,
        });
        assert_eq!(perm.0.get_bit(1), true);
    }

    #[test_case]
    fn segment_no_read() {
        let perm = Permissions::default().segment_type(SegmentType::Code {
            accessed: false,
            read: false,
            conforming: false,
        });
        assert_eq!(perm.0.get_bit(1), false);
    }

    #[test_case]
    fn segment_conforming() {
        let perm = Permissions::default().segment_type(SegmentType::Code {
            accessed: false,
            read: false,
            conforming: true,
        });
        assert_eq!(perm.0.get_bit(2), true);
    }

    #[test_case]
    fn segment_non_conforming() {
        let perm = Permissions::default().segment_type(SegmentType::Code {
            accessed: false,
            read: false,
            conforming: false,
        });
        assert_eq!(perm.0.get_bit(2), false);
    }

    #[test_case]
    fn segment_write() {
        let perm = Permissions::default().segment_type(SegmentType::Data {
            accessed: false,
            write: true,
            expand_down: false,
        });
        assert_eq!(perm.0.get_bit(1), true);
    }

    #[test_case]
    fn segment_no_write() {
        let perm = Permissions::default().segment_type(SegmentType::Data {
            accessed: false,
            write: false,
            expand_down: false,
        });
        assert_eq!(perm.0.get_bit(1), false);
    }

    #[test_case]
    fn segment_expand_down() {
        let perm = Permissions::default().segment_type(SegmentType::Data {
            accessed: false,
            write: false,
            expand_down: true,
        });
        assert_eq!(perm.0.get_bit(2), true);
    }

    #[test_case]
    fn segment_expand_up() {
        let perm = Permissions::default().segment_type(SegmentType::Data {
            accessed: false,
            write: false,
            expand_down: false,
        });
        assert_eq!(perm.0.get_bit(2), false);
    }
}
