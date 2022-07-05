use super::{GateSize, PrivilegeLevel};
use bit_field::BitField;
use core::fmt;

#[derive(Copy, Clone)]
pub struct Configuration(u16);

/// The set of all field offsets for the [`Configuration`] structure.
mod offset {
    /// Offset of the size (D) bit within the
    /// [Configuration](super::Configuration) structure.
    pub const SIZE: usize = 11;

    /// Bounds of the descriptor privilege level (DPL) bits within the
    /// [Configuration](super::Configuration) structure.
    pub mod privilege_level {
        /// Lower bit offset.
        pub const LOWER: usize = 13;
        /// Upper bit offset.
        pub const UPPER: usize = 14;
    }

    /// Offset of the present bit (P) within the
    /// [Configuration](super::Configuration) structure.
    pub const PRESENT: usize = 15;
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration(0x0600)
    }
}

impl Configuration {
    pub fn size(self, size: GateSize) -> Self {
        use offset::SIZE;
        Self(*self.0.clone().set_bit(SIZE, size.into()))
    }

    pub fn privilege_level(self, level: PrivilegeLevel) -> Self {
        use offset::privilege_level::{LOWER, UPPER};
        Self(
            *self
                .0
                .clone()
                .set_bits(LOWER..=UPPER, u8::from(level).into()),
        )
    }

    pub fn present(self, present: bool) -> Self {
        use offset::PRESENT;
        Self(*self.0.clone().set_bit(PRESENT, present))
    }
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use offset::{privilege_level, PRESENT, SIZE};
        write!(
            f,
            "Size: {}\nDPL: {}\nPresent {}",
            self.0.get_bit(SIZE),
            self.0
                .get_bits(privilege_level::LOWER..=privilege_level::UPPER),
            self.0.get_bit(PRESENT)
        )
    }
}
