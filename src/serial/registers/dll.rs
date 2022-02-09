use super::lcr::LineControlRegister;
use super::{ReadRegister, Register, WriteRegister};
use crate::io::{inb, outb};

pub struct DivisorLatchLowByte {
    pub address: u16,
    pub lcr: LineControlRegister,
}

impl Register for DivisorLatchLowByte {
    type Value = u8;
}

impl ReadRegister for DivisorLatchLowByte {
    fn read(&self) -> Self::Value {
        unsafe { inb(self.address) }
    }
}

impl WriteRegister for DivisorLatchLowByte {
    fn write(&self, value: Self::Value) {
        unsafe {
            outb(value, self.address);
        }
    }
}
