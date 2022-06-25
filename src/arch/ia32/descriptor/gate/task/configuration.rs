use super::PrivilegeLevel;
use bit_field::BitField;
use core::fmt;

#[derive(Copy, Clone)]
pub struct Configuration(u16);

const PRIV_LEVEL_LOWER: usize = 13;
const PRIV_LEVEL_UPPER: usize = 14;
const PRESENT_OFFSET: usize = 15;

impl Default for Configuration {
    fn default() -> Self {
        Configuration(0x0500)
    }
}

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
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DPL: {}\nPresent {}",
            self.0.get_bits(PRIV_LEVEL_LOWER..=PRIV_LEVEL_UPPER),
            self.0.get_bit(PRESENT_OFFSET)
        )
    }
}
