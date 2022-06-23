pub use super::Granularity;
pub use crate::arch::ia32::PrivilegeLevel;
use bit_field::BitField;
use core::fmt;

#[repr(u8)]
pub enum DefaultOperationSize {
    Segment16Bits = 0,
    Segment32Bits = 1,
}

impl From<DefaultOperationSize> for bool {
    fn from(value: DefaultOperationSize) -> Self {
        match value {
            DefaultOperationSize::Segment16Bits => false,
            DefaultOperationSize::Segment32Bits => true,
        }
    }
}

#[repr(u8)]
pub enum DescriptorType {
    System = 0,
    CodeOrData = 1,
}

impl From<DescriptorType> for bool {
    fn from(value: DescriptorType) -> Self {
        match value {
            DescriptorType::System => false,
            DescriptorType::CodeOrData => true,
        }
    }
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

impl From<SegmentType> for u8 {
    fn from(value: SegmentType) -> Self {
        match value {
            SegmentType::Data {
                accessed,
                write,
                expand_down,
            } => u8::from(accessed) << 2 | u8::from(write) << 1 | u8::from(expand_down),
            SegmentType::Code {
                accessed,
                read,
                conforming,
            } => 0x8 | u8::from(accessed) << 2 | u8::from(read) << 1 | u8::from(conforming),
        }
    }
}

#[derive(Default, Copy, Clone)]
struct Configuration(u8);

impl Configuration {
    pub fn limit(&mut self, limit: u8) -> &mut Self {
        self.0.set_bits(..4, limit);
        self
    }

    pub fn available(&mut self, value: bool) -> &mut Self {
        self.0.set_bit(4, value);
        self
    }

    pub fn ia32e_mode(&mut self, mode: bool) -> &mut Self {
        //If L-bit is set, then D-bit must be cleared
        // cf. Intel 3.4.5 "L (64 bit code segment) flag"
        if mode {
            self.0.set_bit(6, false);
        }
        self.0.set_bit(5, mode);
        self
    }

    pub fn default_operation_size(&mut self, size: DefaultOperationSize) -> &mut Self {
        self.0.set_bit(6, size.into());
        self
    }

    pub fn granularity(&mut self, granularity: Granularity) -> &mut Self {
        self.0.set_bit(7, granularity.into());
        self
    }
}

#[derive(Default, Copy, Clone)]
struct Permissions(u8);

impl Permissions {
    pub fn segment_type(&mut self, seg_type: SegmentType) -> &mut Self {
        self.0.set_bits(..4, seg_type.into());
        self
    }

    pub fn descriptor_type(&mut self, desc_type: DescriptorType) -> &mut Self {
        self.0.set_bit(4, desc_type.into());
        self
    }

    pub fn privilege_level(&mut self, level: PrivilegeLevel) -> &mut Self {
        self.0.set_bits(5..7, level.into());
        self
    }

    pub fn present(&mut self, present: bool) -> &mut Self {
        self.0.set_bit(7, present);
        self
    }
}

/// A segment descriptor structure that can be used directly by the
/// processor.
#[must_use]
#[derive(Default, Copy, Clone)]
#[repr(C, packed)]
pub struct SegmentDescriptor {
    limit_15_0: u16,
    base_15_0: u16,
    base_23_16: u8,
    permissions: Permissions,
    configuration: Configuration,
    base_31_24: u8,
}

impl SegmentDescriptor {
    /// Create a new [`SegmentDescriptor`] from an address and a limit
    /// with all other flags set to their default value.
    pub fn new(base: u32, limit: u32) -> Self {
        SegmentDescriptor {
            limit_15_0: limit.get_bits(..16).try_into().unwrap(),
            base_15_0: base.get_bits(..16).try_into().unwrap(),
            base_23_16: base.get_bits(16..24).try_into().unwrap(),
            permissions: Permissions::default(),
            configuration: *Configuration::default()
                .limit(limit.get_bits(16..20).try_into().unwrap()),
            base_31_24: base.get_bits(24..32).try_into().unwrap(),
        }
    }

    /// Change the type of the segment by another [`SegmentType`].
    pub fn segment_type(&mut self, seg_type: SegmentType) -> &mut Self {
        self.permissions.segment_type(seg_type);
        self
    }

    /// Change the descriptor type by another [`DescriptorType`].
    pub fn descriptor_type(&mut self, desc_type: DescriptorType) -> &mut Self {
        self.permissions.descriptor_type(desc_type);
        self
    }

    /// Set the privilege level of the segment.
    pub fn privilege_level(&mut self, level: PrivilegeLevel) -> &mut Self {
        self.permissions.privilege_level(level);
        self
    }

    /// Set or clear the presence bit of the segment.
    pub fn present(&mut self, present: bool) -> &mut Self {
        self.permissions.present(present);
        self
    }

    /// Set or clear the available bit of the [`SegmentDescriptor`].
    pub fn available(&mut self, avl: bool) -> &mut Self {
        self.configuration.available(avl);
        self
    }

    /// Set or clear the 64-bit code segment flag. If the bit is set, also
    /// clear the D flag.
    pub fn ia32e_mode(&mut self, mode: bool) -> &mut Self {
        self.configuration.ia32e_mode(mode);
        self
    }

    /// Set the default operation size of the segment.
    pub fn default_operation_size(&mut self, size: DefaultOperationSize) -> &mut Self {
        self.configuration.default_operation_size(size);
        self
    }

    /// Set the granularity of the segment.
    pub fn granularity(&mut self, granularity: Granularity) -> &mut Self {
        self.configuration.granularity(granularity);
        self
    }
}

impl fmt::Display for SegmentDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Not implemented")
    }
}
