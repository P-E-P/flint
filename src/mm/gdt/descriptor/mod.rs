/// Offset the bits of the given identifier by it's offset.
macro_rules! offset {
    ($n: ident) => {
        bits::$n << offsets::$n
    };
}

/// Clear the flag bits `n` from a number `s` and set those bits to the new
/// `v` value.
macro_rules! setbits {
    ($s: expr, $v: expr, $n: ident) => {
        // - Ensure the value does not overflow
        // - Clear previous flag
        // - Set new flag
        ($s & !flags::$n) | (($v & bits::$n) << offsets::$n)
    };
}

mod lower;
mod upper;

#[repr(u8)]
pub enum PrivilegeLevel {
    Kernel = 0,
    Unused1 = 1,
    Unused2 = 2,
    Userland = 3,
}

#[repr(u8)]
pub enum DefaultOperationSize {
    Segment16Bits = 0,
    Segment32Bits = 1,
}

#[repr(u8)]
pub enum Granularity {
    Byte = 0,
    FourKByte = 1,
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

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct SegmentDescriptor {
    upper: upper::Upper,
    lower: lower::Lower,
}

impl Default for SegmentDescriptor {
    fn default() -> Self {
        SegmentDescriptor::new(0, 0).present(false)
    }
}

impl SegmentDescriptor {
    pub fn new(base: u32, limit: u32) -> Self {
        let base_low = base & 0xff;
        let base_mid = (base & 0xff) >> 8;
        let base_high = (base & 0xff) >> 16;

        let limit_low = limit & 0xffff;
        let limit_high = limit >> 16;

        SegmentDescriptor {
            lower: lower::Lower::default()
                .base_low(base_low)
                .limit_low(limit_low),
            upper: upper::Upper::default()
                .base_high(base_high)
                .base_mid(base_mid)
                .limit_high(limit_high),
        }
    }

    pub fn segment_type(self, seg_type: SegmentType) -> Self {
        Self {
            lower: self.lower,
            upper: self.upper.segment_type(seg_type.into()),
        }
    }

    pub fn descriptor_type(self, desc_type: DescriptorType) -> Self {
        Self {
            lower: self.lower,
            upper: self.upper.descriptor_type(desc_type as u32),
        }
    }

    pub fn available(self, avl: bool) -> Self {
        Self {
            lower: self.lower,
            upper: self.upper.available(avl.into()),
        }
    }

    pub fn ia32e_mode(self, mode: bool) -> Self {
        Self {
            lower: self.lower,
            upper: self.upper.ia32e_mode(mode.into()),
        }
    }

    pub fn privilege_level(self, level: PrivilegeLevel) -> Self {
        Self {
            lower: self.lower,
            upper: self.upper.privilege_level(level as u32),
        }
    }

    pub fn present(self, present: bool) -> Self {
        Self {
            lower: self.lower,
            upper: self.upper.present(present.into()),
        }
    }

    pub fn default_operation_size(self, size: DefaultOperationSize) -> Self {
        Self {
            lower: self.lower,
            upper: self.upper.default_operation_size(size as u32),
        }
    }

    pub fn granularity(self, granularity: Granularity) -> Self {
        Self {
            lower: self.lower,
            upper: self.upper.granularity(granularity as u32),
        }
    }
}
