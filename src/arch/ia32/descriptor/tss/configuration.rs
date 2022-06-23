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
        self.0.get_bits(..4)
    }

    pub fn limit(&mut self, limit: u8) -> &mut Self {
        self.0.set_bits(..4, limit);
        self
    }

    pub fn available(&mut self, value: bool) -> &mut Self {
        self.0.set_bit(offset::AVL, value);
        self
    }

    pub fn granularity(&mut self, granularity: Granularity) -> &mut Self {
        self.0.set_bit(offset::G, granularity.into());
        self
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
