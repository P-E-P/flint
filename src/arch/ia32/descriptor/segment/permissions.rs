use core::fmt;
use super::{DescriptorType, SegmentType};
use crate::arch::ia32::PrivilegeLevel;
use bit_field::BitField;

#[derive(Default, Copy, Clone)]
pub struct Permissions(u8);

const SEGMENT_UPPER : usize = 3;
const DESC_TYPE_OFFSET : usize = 4;
const PRIV_LEVEL_LOWER : usize = 5;
const PRIV_LEVEL_UPPER : usize = 6;
const PRESENT_OFFSET : usize = 7;

impl Permissions {
    pub fn segment_type(&mut self, seg_type: SegmentType) -> &mut Self {
        self.0.set_bits(..=SEGMENT_UPPER, seg_type.into());
        self
    }

    pub fn descriptor_type(&mut self, desc_type: DescriptorType) -> &mut Self {
        self.0.set_bit(DESC_TYPE_OFFSET, desc_type.into());
        self
    }

    pub fn privilege_level(&mut self, level: PrivilegeLevel) -> &mut Self {
        self.0.set_bits(PRIV_LEVEL_LOWER..=PRIV_LEVEL_UPPER, level.into());
        self
    }

    pub fn present(&mut self, present: bool) -> &mut Self {
        self.0.set_bit(PRESENT_OFFSET, present);
        self
    }
}

impl fmt::Display for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Type: {}\nS: {}\nDPL: {}\nPresent {}",
            self.0.get_bits(..=SEGMENT_UPPER),
            self.0.get_bit(DESC_TYPE_OFFSET),
            self.0.get_bits(PRIV_LEVEL_LOWER..=PRIV_LEVEL_UPPER),
            self.0.get_bit(PRESENT_OFFSET))
    }
}
