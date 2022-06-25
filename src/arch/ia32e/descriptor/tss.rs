use crate::arch::ia32::descriptor::tss::TssDescriptor as IA32TssDescriptor;
use crate::arch::ia32e::{descriptor::Granularity, PrivilegeLevel};
use bit_field::BitField;

#[must_use]
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct TssDescriptor {
    tss: IA32TssDescriptor,
    base_63_32: u32,
    reserved: u32,
}

impl TssDescriptor {
    pub fn new(base: u64, limit: u32) -> Self {
        TssDescriptor {
            reserved: 0,
            base_63_32: base.get_bits(32..64).try_into().unwrap(),
            tss: IA32TssDescriptor::new(base.get_bits(..32).try_into().unwrap(), limit),
        }
    }

    pub fn busy(self, state: bool) -> Self {
        Self {
            tss: self.tss.busy(state),
            ..self
        }
    }

    pub fn available(self, avl: bool) -> Self {
        Self {
            tss: self.tss.available(avl),
            ..self
        }
    }

    pub fn privilege_level(self, level: PrivilegeLevel) -> Self {
        Self {
            tss: self.tss.privilege_level(level),
            ..self
        }
    }

    pub fn present(self, present: bool) -> Self {
        Self {
            tss: self.tss.present(present),
            ..self
        }
    }

    pub fn granularity(self, granularity: Granularity) -> Self {
        Self {
            tss: self.tss.granularity(granularity),
            ..self
        }
    }
}
