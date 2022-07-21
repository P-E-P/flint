//! A module containing the implementation, tests and revolving types around
//! the [`TssDescriptor`] structure to describe the permissions and capacities
//! of a task state segment.
use crate::arch::ia32::descriptor::tss::TssDescriptor as IA32TssDescriptor;
use crate::arch::ia32e::{descriptor::Granularity, PrivilegeLevel};
use crate::utils::bitfield::BitField;

/// A task state segment descriptor structure that can be used directly by the
/// processor to describe a task state segment.
#[must_use]
#[derive(Default, Copy, Clone)]
#[repr(C, packed)]
pub struct TssDescriptor {
    tss: IA32TssDescriptor,
    /// Bits 32 to 63 of the segment base address.
    base_63_32: u32,
    /// Reserved bits.
    reserved: u32,
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
    pub fn new(base: u64, limit: u32) -> Self {
        TssDescriptor {
            base_63_32: base.get_bits(32..64).try_into().unwrap(),
            tss: IA32TssDescriptor::new(base.get_bits(..32).try_into().unwrap(), limit),
            ..Default::default()
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
            tss: self.tss.busy(state),
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
            tss: self.tss.available(avl),
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
            tss: self.tss.privilege_level(level),
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
            tss: self.tss.present(present),
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
            tss: self.tss.granularity(granularity),
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
        assert_eq!(size_of::<TssDescriptor>(), 16);
    }

    #[test_case]
    fn present() {
        let desc = TssDescriptor::new(0, 0);

        assert_eq!(
            unsafe { core::mem::transmute::<TssDescriptor, u128>(desc) }.get_bit(47),
            true
        )
    }
}
