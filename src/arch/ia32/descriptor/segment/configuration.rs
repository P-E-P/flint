use super::{DefaultOperationSize, Granularity};
use bit_field::BitField;

#[derive(Default, Copy, Clone)]
pub struct Configuration(u8);

impl Configuration {
    pub fn limit(&mut self, limit: u8) -> &mut Self {
        self.0.set_bits(..4, limit);
        self
    }

    pub fn available(&mut self, value: bool) -> &mut Self {
        self.0.set_bit(4, value);
        self
    }

    pub fn ia32e_mode(&mut self, mode: bool) -> &mut Self {
        //If L-bit is set, then D-bit must be cleared
        // cf. Intel 3.4.5 "L (64 bit code segment) flag"
        if mode {
            self.0.set_bit(6, false);
        }
        self.0.set_bit(5, mode);
        self
    }

    pub fn default_operation_size(&mut self, size: DefaultOperationSize) -> &mut Self {
        self.0.set_bit(6, size.into());
        self
    }

    pub fn granularity(&mut self, granularity: Granularity) -> &mut Self {
        self.0.set_bit(7, granularity.into());
        self
    }
}
