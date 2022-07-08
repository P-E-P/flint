//! A module containing the implementation, tests and types around the
//! [`TssDescriptor`] structure to describe the permissions and capacities
//! of a task state segment.
use super::Granularity;
use crate::arch::ia32::PrivilegeLevel;
use bit_field::BitField;
use configuration::Configuration;
use permissions::Permissions;

mod configuration;
mod permissions;

/// A task state segment descriptor structure that can be used directly by the
/// processor to describe a task state segment.
#[must_use]
#[derive(Default, Copy, Clone)]
#[repr(C, packed)]
pub struct TssDescriptor {
    /// Bits 0 to 15 of the segment limit.
    limit_15_0: u16,
    /// Bits 0 to 15 of the segment base address.
    base_15_0: u16,
    /// Bits 16 to 23 of the segment base address.
    base_23_16: u8,
    /// Segment permissions (TYPE, B, DPL, P).
    permissions: Permissions,
    /// Segment configuration (Limit bits 16 to 19, AVL, G)
    configuration: Configuration,
    /// Bits 24 to 31 of the segment base address.
    base_31_24: u8,
}

impl TssDescriptor {
    /// Creates a new [`TssDescriptor`] from a base address and a segment limit.
    ///
    /// # Arguments
    ///
    /// * `base` - The segments base adress.
    /// * `limit` - The limit value for the tss descriptor.
    ///
    /// # Note
    ///
    /// The present bit will be enabled when using this constructor.
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

    /// Change a [`TssDescriptor`]'s busy bit.
    ///
    /// # Arguments
    ///
    /// * `busy` - The desired bit value, `true` for bit value 1 and `false`
    /// for bit value 0.
    pub fn busy(self, state: bool) -> Self {
        Self {
            permissions: self.permissions.busy(state),
            ..self
        }
    }

    /// Change a [`TssDescriptor`]'s available bit value.
    ///
    /// # Arguments
    ///
    /// * `value` - The desired bit value, a value of `true` will store a `1`,
    /// `false` will store the bit `0`.
    pub fn available(self, avl: bool) -> Self {
        Self {
            configuration: self.configuration.available(avl),
            ..self
        }
    }

    /// Change a [`TssDescriptor`]'s privilege level by another one.
    ///
    /// # Arguments
    ///
    /// * `level` - The desired [`PrivilegeLevel`] value.
    pub fn privilege_level(self, level: PrivilegeLevel) -> Self {
        Self {
            permissions: self.permissions.privilege_level(level),
            ..self
        }
    }

    /// Change a [`TssDescriptor`]'s present bit.
    ///
    /// # Arguments
    ///
    /// * `present` - The desired bit value, `true` for bit value 1 and `false`
    /// for bit value 0.
    pub fn present(self, present: bool) -> Self {
        Self {
            permissions: self.permissions.present(present),
            ..self
        }
    }

    /// Change a [`TssDescriptor`]'s granularity.
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn structure_size() {
        use core::mem::size_of;
        assert_eq!(size_of::<TssDescriptor>(), 8);
    }

    #[test_case]
    fn present() {
        let desc = TssDescriptor::new(0, 0);

        assert_eq!(
            unsafe { core::mem::transmute::<TssDescriptor, u64>(desc) }.get_bit(47),
            true
        )
    }
}
