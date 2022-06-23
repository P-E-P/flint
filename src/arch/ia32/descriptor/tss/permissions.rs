use crate::arch::ia32::PrivilegeLevel;
use bit_field::BitField;
use core::fmt;

#[derive(Copy, Clone)]
pub struct Permissions(u8);

const PRIV_LEVEL_LOWER: usize = 5;
const PRIV_LEVEL_UPPER: usize = 6;
const PRESENT_OFFSET: usize = 7;

impl Default for Permissions {
    fn default() -> Self {
        Permissions(0b00001001)
    }
}

impl Permissions {
    pub fn busy(&mut self, busy: bool) -> &mut Self {
        self.0.set_bit(1, busy);
        self
    }

    pub fn privilege_level(&mut self, level: PrivilegeLevel) -> &mut Self {
        self.0
            .set_bits(PRIV_LEVEL_LOWER..=PRIV_LEVEL_UPPER, level.into());
        self
    }

    pub fn present(&mut self, present: bool) -> &mut Self {
        self.0.set_bit(PRESENT_OFFSET, present);
        self
    }
}

impl fmt::Display for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Busy: {}\nDPL: {}\nPresent {}",
            self.0.get_bit(1),
            self.0.get_bits(PRIV_LEVEL_LOWER..=PRIV_LEVEL_UPPER),
            self.0.get_bit(PRESENT_OFFSET)
        )
    }
}
