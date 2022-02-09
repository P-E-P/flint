use super::lcr::LineControlRegister;
use super::{ReadRegister, Register, WriteRegister};
use crate::io::{inb, outb};

pub struct DivisorLatchHighByte {
    pub address: u16,
    pub lcr: LineControlRegister,
}

impl Register for DivisorLatchHighByte {
    type Value = u8;
}

impl WriteRegister for DivisorLatchHighByte {
    fn write(&self, value: Self::Value) {
        unsafe {
            outb(value, self.address);
        }
    }
}

impl ReadRegister for DivisorLatchHighByte {
    fn read(&self) -> Self::Value {
        unsafe { inb(self.address) }
    }
}
