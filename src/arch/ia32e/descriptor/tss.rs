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

    pub fn busy(&mut self, state: bool) -> &mut Self {
        self.tss.busy(state);
        self
    }

    pub fn available(&mut self, avl: bool) -> &mut Self {
        self.tss.available(avl);
        self
    }

    pub fn privilege_level(&mut self, level: PrivilegeLevel) -> &mut Self {
        self.tss.privilege_level(level);
        self
    }

    pub fn present(&mut self, present: bool) -> &mut Self {
        self.tss.present(present);
        self
    }

    pub fn granularity(&mut self, granularity: Granularity) -> &mut Self {
        self.tss.granularity(granularity);
        self
    }
}
