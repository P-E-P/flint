use super::{Kind, PrivilegeLevel};
use bit_field::BitField;
use core::fmt;

#[derive(Default, Copy, Clone)]
pub struct Configuration(u16);

const PRIV_LEVEL_LOWER: usize = 13;
const PRIV_LEVEL_UPPER: usize = 14;
const PRESENT_OFFSET: usize = 15;
const TYPE_LOWER: usize = 8;
const TYPE_UPPER: usize = 11;
const IST_LOWER: usize = 0;
const IST_UPPER: usize = 2;

impl Configuration {
    pub fn privilege_level(self, level: PrivilegeLevel) -> Self {
        Self(
            *self
                .0
                .clone()
                .set_bits(PRIV_LEVEL_LOWER..=PRIV_LEVEL_UPPER, u8::from(level).into()),
        )
    }

    pub fn present(self, present: bool) -> Self {
        Self(*self.0.clone().set_bit(PRESENT_OFFSET, present))
    }

    pub fn interrupt_stack_table(self, ist: u8) -> Self {
        Self(*self.0.clone().set_bits(IST_LOWER..=IST_UPPER, ist.into()))
    }

    pub fn kind(self, kind: Kind) -> Self {
        Self(
            *self
                .0
                .clone()
                .set_bits(TYPE_LOWER..=TYPE_UPPER, kind.into()),
        )
    }
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DPL: {}\nPresent {}\nType:{}\nIST:{}",
            self.0.get_bits(PRIV_LEVEL_LOWER..=PRIV_LEVEL_UPPER),
            self.0.get_bit(PRESENT_OFFSET),
            self.0.get_bits(TYPE_LOWER..=TYPE_UPPER),
            self.0.get_bits(IST_LOWER..=IST_UPPER)
        )
    }
}
