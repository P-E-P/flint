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

    pub fn limit(self, limit: u8) -> Self {
        Self(*self.0.clone().set_bits(..4, limit))
    }

    pub fn available(self, value: bool) -> Self {
        Self(*self.0.clone().set_bit(offset::AVL, value))
    }

    pub fn ia32e_mode(self, mode: bool) -> Self {
        let mut result = self.0;
        //If L-bit is set, then D-bit must be cleared
        // cf. Intel 3.4.5 "L (64 bit code segment) flag"
        if mode {
            result.set_bit(offset::D_B, false);
        }
        result.set_bit(offset::L, mode);
        Self(result)
    }

    pub fn default_operation_size(self, size: DefaultOperationSize) -> Self {
        Self(*self.0.clone().set_bit(offset::D_B, size.into()))
    }

    pub fn granularity(self, granularity: Granularity) -> Self {
        Self(*self.0.clone().set_bit(offset::G, granularity.into()))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn default_values() {
        let conf = Configuration::default();
        assert_eq!(conf.0, 0);
    }

    #[test_case]
    fn limit_bits() {
        let limit = 12;
        let conf = Configuration::default().limit(limit);
        assert_eq!(conf.0.get_bits(0..4), limit);
    }

    #[test_case]
    fn limit_getter() {
        let limit = 0xf;
        let conf = Configuration::default().limit(limit);
        assert_eq!(conf.get_limit(), limit);
    }

    #[test_case]
    fn available() {
        let conf = Configuration::default().available(true);
        assert_eq!(conf.0.get_bit(4), true);
    }

    #[test_case]
    fn not_available() {
        let conf = Configuration::default().available(false);
        assert_eq!(conf.0.get_bit(4), false);
    }

    #[test_case]
    fn ia32e_mode() {
        let conf = Configuration::default().ia32e_mode(true);
        assert_eq!(conf.0.get_bit(5), true);
        // Intel 3.4.5/L (64-bit code segment) flag
        // > If L-bit is set, then D-bit must be cleared.
        assert_eq!(conf.0.get_bit(6), false);
    }

    #[test_case]
    fn db_16bits() {
        use DefaultOperationSize::Segment16Bits;
        let conf = Configuration::default().default_operation_size(Segment16Bits);
        assert_eq!(conf.0.get_bit(6), false);
    }

    #[test_case]
    fn db_32bits() {
        use DefaultOperationSize::Segment32Bits;
        let conf = Configuration::default().default_operation_size(Segment32Bits);
        assert_eq!(conf.0.get_bit(6), true);
    }

    #[test_case]
    fn granularity_byte() {
        use Granularity::Byte;
        let conf = Configuration::default().granularity(Byte);
        assert_eq!(conf.0.get_bit(7), false);
    }

    #[test_case]
    fn granularity_4kbyte() {
        use Granularity::FourKByte;
        let conf = Configuration::default().granularity(FourKByte);
        assert_eq!(conf.0.get_bit(7), true);
    }
}
