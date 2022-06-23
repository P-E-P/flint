use super::{DefaultOperationSize, Granularity};
use bit_field::BitField;
use core::fmt;

#[derive(Default, Copy, Clone)]
pub struct Configuration(u8);

mod offset {
    pub const AVL: usize = 4;
    pub const L: usize = 5;
    pub const D_B: usize = 6;
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

    pub fn ia32e_mode(&mut self, mode: bool) -> &mut Self {
        //If L-bit is set, then D-bit must be cleared
        // cf. Intel 3.4.5 "L (64 bit code segment) flag"
        if mode {
            self.0.set_bit(offset::D_B, false);
        }
        self.0.set_bit(offset::L, mode);
        self
    }

    pub fn default_operation_size(&mut self, size: DefaultOperationSize) -> &mut Self {
        self.0.set_bit(offset::D_B, size.into());
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
            "AVL: {}\n64-bit code segment: {}\nDefault operation size: {}\nGranularity: {}",
            self.0.get_bit(offset::AVL),
            self.0.get_bit(offset::L),
            self.0.get_bit(offset::D_B),
            self.0.get_bit(offset::G)
        )
    }
}
