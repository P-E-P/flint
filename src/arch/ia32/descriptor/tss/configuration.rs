use super::Granularity;
use bit_field::BitField;
use core::fmt;

#[derive(Default, Copy, Clone)]
pub struct Configuration(u8);

mod offset {
    pub const AVL: usize = 4;
    pub const G: usize = 7;
}

impl Configuration {
    pub fn get_limit(&self) -> u8 {
        self.0.clone().get_bits(..4)
    }

    pub fn limit(self, limit: u8) -> Self {
        Self(*self.0.clone().set_bits(..4, limit))
    }

    pub fn available(self, value: bool) -> Self {
        Self(*self.0.clone().set_bit(offset::AVL, value))
    }

    pub fn granularity(self, granularity: Granularity) -> Self {
        Self(*self.0.clone().set_bit(offset::G, granularity.into()))
    }
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AVL: {}\nGranularity: {}",
            self.0.get_bit(offset::AVL),
            self.0.get_bit(offset::G)
        )
    }
}
