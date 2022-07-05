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
            permissions: Permissions::default().present(true),
            configuration: Configuration::default()
                .limit(limit.get_bits(16..20).try_into().unwrap()),
            base_31_24: base.get_bits(24..32).try_into().unwrap(),
        }
    }

    pub fn busy(self, state: bool) -> Self {
        Self {
            permissions: self.permissions.busy(state),
            ..self
        }
    }

    pub fn available(self, avl: bool) -> Self {
        Self {
            configuration: self.configuration.available(avl),
            ..self
        }
    }

    pub fn privilege_level(self, level: PrivilegeLevel) -> Self {
        Self {
            permissions: self.permissions.privilege_level(level),
            ..self
        }
    }

    pub fn present(self, present: bool) -> Self {
        Self {
            permissions: self.permissions.present(present),
            ..self
        }
    }

    pub fn granularity(self, granularity: Granularity) -> Self {
        Self {
            configuration: self.configuration.granularity(granularity),
            ..self
        }
    }
}
