use super::{ReadRegister, Register, WriteRegister};
use crate::io::{inb, outb};

pub struct ScratchRegister {
    address: u16,
}

impl Register for ScratchRegister {
    type Value = u8;
}

impl ReadRegister for ScratchRegister {
    fn read(&self) -> Self::Value {
        unsafe { inb(self.address) }
    }
}

impl WriteRegister for ScratchRegister {
    fn write(&self, value: Self::Value) {
        unsafe {
            outb(value, self.address);
        }
    }
}
