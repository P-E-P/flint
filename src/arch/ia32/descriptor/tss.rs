use super::Granularity;
use crate::arch::ia32::PrivilegeLevel;
use bit_field::BitField;
use configuration::Configuration;
use permissions::Permissions;

mod configuration;
mod permissions;

#[must_use]
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct TssDescriptor {
    limit_15_0: u16,
    base_15_0: u16,
    base_23_16: u8,
    permissions: Permissions,
    configuration: Configuration,
    base_31_24: u8,
}

impl TssDescriptor {
    pub fn new(base: u32, limit: u32) -> Self {
        TssDescriptor {
            limit_15_0: limit.get_bits(..16).try_into().unwrap(),
            base_15_0: base.get_bits(..16).try_into().unwrap(),
            base_23_16: base.get_bits(16..24).try_into().unwrap(),
            permissions: Permissions::default(),
            configuration: *Configuration::default()
                .limit(limit.get_bits(16..20).try_into().unwrap()),
            base_31_24: base.get_bits(24..32).try_into().unwrap(),
        }
    }

    pub fn busy(&mut self, state: bool) -> &mut Self {
        self.permissions.busy(state);
        self
    }

    pub fn available(&mut self, avl: bool) -> &mut Self {
        self.configuration.available(avl);
        self
    }

    pub fn privilege_level(&mut self, level: PrivilegeLevel) -> &mut Self {
        self.permissions.privilege_level(level);
        self
    }

    pub fn present(&mut self, present: bool) -> &mut Self {
        self.permissions.present(present);
        self
    }

    pub fn granularity(&mut self, granularity: Granularity) -> &mut Self {
        self.configuration.granularity(granularity);
        self
    }
}
