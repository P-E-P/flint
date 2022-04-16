pub use super::Granularity;
use super::{split_address, split_limit};
pub use crate::PrivilegeLevel;
use core::fmt;

pub mod lower;
mod upper;

#[repr(u8)]
pub enum DefaultOperationSize {
    Segment16Bits = 0,
    Segment32Bits = 1,
}

#[repr(u8)]
pub enum DescriptorType {
    System = 0,
    CodeOrData = 1,
}

pub enum SegmentType {
    /// Data segment representation.
    Data {
        accessed: bool,
        write: bool,
        expand_down: bool,
    },
    /// Code segment representation.
    Code {
        accessed: bool,
        read: bool,
        conforming: bool,
    },
}

impl From<SegmentType> for u32 {
    fn from(value: SegmentType) -> Self {
        match value {
            SegmentType::Data {
                accessed,
                write,
                expand_down,
            } => u32::from(accessed) << 2 | u32::from(write) << 1 | u32::from(expand_down),
            SegmentType::Code {
                accessed,
                read,
                conforming,
            } => 0x8 | u32::from(accessed) << 2 | u32::from(read) << 1 | u32::from(conforming),
        }
    }
}

/// A segment descriptor structure that can be used directly by the
/// processor.
#[must_use]
#[derive(Default, Copy, Clone)]
#[repr(C, packed)]
pub struct SegmentDescriptor {
    upper: upper::Upper,
    lower: lower::Lower,
}

impl SegmentDescriptor {
    /// Create a new [`SegmentDescriptor`] from an address and a limit
    /// with all other flags set to their default value.
    pub fn new(base: u32, limit: u32) -> Self {
        let (base_high, base_mid, base_low) = split_address(base);
        let (limit_high, limit_low) = split_limit(limit);

        SegmentDescriptor {
            lower: lower::Lower::default()
                .base_low(base_low)
                .limit_low(limit_low),
            upper: upper::Upper::default()
                .base_high(base_high)
                .base_mid(base_mid)
                .limit_high(limit_high)
                .present(1),
        }
    }

    /// Change the type of the segment by another [`SegmentType`].
    pub fn segment_type(self, seg_type: SegmentType) -> Self {
        Self {
            upper: self.upper.segment_type(seg_type.into()),
            ..self
        }
    }

    /// Change the descriptor type by another [`DescriptorType`].
    pub fn descriptor_type(self, desc_type: DescriptorType) -> Self {
        Self {
            upper: self.upper.descriptor_type(desc_type as u32),
            ..self
        }
    }

    /// Set or clear the available bit of the [`SegmentDescriptor`].
    pub fn available(self, avl: bool) -> Self {
        Self {
            upper: self.upper.available(avl.into()),
            ..self
        }
    }

    /// Set or clear the 64-bit code segment flag. If the bit is set, also
    /// clear the D flag.
    pub fn ia32e_mode(self, mode: bool) -> Self {
        let mut upper = self.upper.ia32e_mode(mode.into());
        //If L-bit is set, then D-bit must be cleared
        // cf. Intel 3.4.5 "L (64 bit code segment) flag"
        if mode {
            upper = upper.default_operation_size(DefaultOperationSize::Segment16Bits as u32);
        }
        Self { upper, ..self }
    }

    /// Set the privilege level of the segment.
    pub fn privilege_level(self, level: PrivilegeLevel) -> Self {
        Self {
            upper: self.upper.privilege_level(level as u32),
            ..self
        }
    }

    /// Set or clear the presence bit of the segment.
    pub fn present(self, present: bool) -> Self {
        Self {
            upper: self.upper.present(present.into()),
            ..self
        }
    }

    /// Set the default operation size of the segment.
    pub fn default_operation_size(self, size: DefaultOperationSize) -> Self {
        Self {
            upper: self.upper.default_operation_size(size as u32),
            ..self
        }
    }

    /// Set the granularity of the segment.
    pub fn granularity(self, granularity: Granularity) -> Self {
        Self {
            upper: self.upper.granularity(granularity as u32),
            ..self
        }
    }
}

impl fmt::Display for SegmentDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:08X?};{:08X?}", self.upper, self.lower)
    }
}
