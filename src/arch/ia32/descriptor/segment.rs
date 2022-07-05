//! A module containing the implementation, tests and revolving types around
//! the [`SegmentDescriptor`] structure to describe the permissions and
//! capacities of a memory segment.
pub use super::Granularity;
pub use crate::arch::ia32::PrivilegeLevel;
use bit_field::BitField;
use configuration::Configuration;
use core::fmt;
use permissions::Permissions;

mod configuration;
mod permissions;

/// The default operation size modes of a memory segment.
///
/// It could be either 16 bits (`0`) or 32 bits (`0`). It performs different
/// functions depending on wheter the segment descriptor is an executable code
/// segment, an expand down data segment or a stack segment (cf. Intel volume
/// III, 3.4.5 D/B (default operation flag))
#[repr(u8)]
pub enum DefaultOperationSize {
    /// 16 bits memory segment default operation size.
    Segment16Bits = 0,
    /// 32 bits memory segment default operation size.
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
        /// Whether the segment has been accessed by the processor.
        accessed: bool,
        write: bool,
        expand_down: bool,
    },
    /// Code segment representation.
    Code {
        /// Whether the segment has been accessed by the processor.
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

/// A segment descriptor structure that can be used directly by the
/// processor to describe a memory segment.
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
    ///
    /// # Arguments
    ///
    /// * `base` - The segments base adress.
    /// * `limit` - The limit value for the segment descriptor.
    ///
    /// # Note
    ///
    /// The present bit will be enabled when using this constructor.
    pub fn new(base: u32, limit: u32) -> Self {
        SegmentDescriptor {
            limit_15_0: limit.get_bits(..16).try_into().unwrap(),
            base_15_0: base.get_bits(..16).try_into().unwrap(),
            base_23_16: base.get_bits(16..24).try_into().unwrap(),
            permissions: Permissions::default().present(true),
            configuration: Configuration::default()
                .limit(limit.get_bits(16..20).try_into().unwrap()),
            base_31_24: base.get_bits(24..32).try_into().unwrap(),
        }
    }

    /// Change a [`SegmentDescriptor`]'s segment type by another
    /// [`SegmentType`].
    ///
    /// # Arguments
    ///
    /// * `seg_type` - The desired segment type.
    pub fn segment_type(self, seg_type: SegmentType) -> Self {
        Self {
            permissions: self.permissions.segment_type(seg_type),
            ..self
        }
    }

    /// Change a [`SegmentDescriptor`]'s [`DescriptorType`] by another one.
    ///
    /// # Arguments
    ///
    /// * `desc_type` - The desired descriptor type.
    pub fn descriptor_type(self, desc_type: DescriptorType) -> Self {
        Self {
            permissions: self.permissions.descriptor_type(desc_type),
            ..self
        }
    }

    /// Change a [`SegmentDescriptor`]'s privilege level by another one.
    ///
    /// # Arguments
    ///
    /// * `privilege_level` - The desired [`PrivilegeLevel`].
    pub fn privilege_level(self, level: PrivilegeLevel) -> Self {
        Self {
            permissions: self.permissions.privilege_level(level),
            ..self
        }
    }

    /// Change a [`SegmentDescriptor`]'s presence bit value.
    ///
    /// # Arguments
    ///
    /// * `present` - The desired bit value. Use `true` for 1 and `false` for 0.
    pub fn present(self, present: bool) -> Self {
        Self {
            permissions: self.permissions.present(present),
            ..self
        }
    }

    /// Set or clear the available bit of the [`SegmentDescriptor`].
    ///
    /// # Arguments
    ///
    /// * `avl` - The desired bit value. Use `true` for 1 and `false` for 0.
    pub fn available(self, avl: bool) -> Self {
        Self {
            configuration: self.configuration.available(avl),
            ..self
        }
    }

    /// Change a [`SegmentDescriptor`]'s current mode to 32 bits or 64 bits.
    ///
    /// # Arguments
    ///
    /// * `mode` - The desired mode, `true` for 64 bits and `false` for 32 bits.
    ///
    /// # Note
    ///
    /// This method will overwrite any previous call to
    /// [default_operation_size](SegmentDescriptor#method.default_operation_size)
    /// if the mode is set to 64bits as it required the bit `D/B` set to `0`
    /// (cf. Intel Volume III 3.4.5).
    pub fn ia32e_mode(self, mode: bool) -> Self {
        Self {
            configuration: self.configuration.ia32e_mode(mode),
            ..self
        }
    }

    /// Change a [`SegmentDescriptor`]'s default operation size.
    ///
    /// # Arguments
    ///
    /// * `size` - The desired mode operation size.
    pub fn default_operation_size(self, size: DefaultOperationSize) -> Self {
        Self {
            configuration: self.configuration.default_operation_size(size),
            ..self
        }
    }

    /// Change a [`SegmentDescriptor`]'s granularity.
    ///
    /// # Arguments
    ///
    /// * `granularity` - The desired granularity.
    pub fn granularity(self, granularity: Granularity) -> Self {
        Self {
            configuration: self.configuration.granularity(granularity),
            ..self
        }
    }

    /// Get the whole reassembled base address from a [`SegmentDescriptor`]
    /// fields.
    fn get_address(&self) -> u32 {
        u32::from(self.base_31_24) << 24
            | u32::from(self.base_23_16) << 16
            | u32::from(self.base_15_0)
    }

    /// Get the whole reassembled segment limit from a [`SegmentDescriptor`]
    /// fields.
    fn get_limit(&self) -> u32 {
        u32::from(self.configuration.get_limit()) << 16 | u32::from(self.limit_15_0)
    }
}

impl fmt::Display for SegmentDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Base address: {}\nLimit: {}\n{}\n{}",
            self.get_address(),
            self.get_limit(),
            self.configuration,
            self.permissions
        )
    }
}
