use super::{DescriptorType, SegmentType};
use crate::arch::ia32::PrivilegeLevel;
use bit_field::BitField;

#[derive(Default, Copy, Clone)]
pub struct Permissions(u8);

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
