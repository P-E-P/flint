//! A module containing the implementation and tests for the [`Permissions`]
//! structure.
use super::{DescriptorType, SegmentType};
use crate::arch::ia32::PrivilegeLevel;
use bit_field::BitField;
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
        Self(*self.0.clone().set_bits(..=UPPER, seg_type.into()))
    }

    /// Change a [`Permissions`]'s descriptor type value.
    ///
    /// # Arguments
    ///
    /// * `desc_type` - The desired [`DescriptorType`] value.
    pub fn descriptor_type(self, desc_type: DescriptorType) -> Self {
        use offset::DESC_TYPE;
        Self(*self.0.clone().set_bit(DESC_TYPE, desc_type.into()))
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
        write!(
            f,
            "Type: {}\nS: {}\nDPL: {}\nPresent {}",
            self.0.get_bits(..=offset::segment_type::UPPER),
            self.0.get_bit(offset::DESC_TYPE),
            self.0.get_bits(offset::privilege_level::LOWER..=offset::privilege_level::UPPER),
            self.0.get_bit(offset::PRESENT)
        )
    }
}
