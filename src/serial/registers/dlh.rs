use super::Register;
use crate::io::{inb, outb};

pub struct DivisorLatchLowByte {
    pub address: u16,
}

impl Register for DivisorLatchLowByte {
    type Value = u8;

    fn read(&self) -> Self::Value {
        unsafe { inb(self.address) }
    }

    fn write(&self, value: Self::Value) {
        unsafe {
            outb(value, self.address);
        }
    }
}
