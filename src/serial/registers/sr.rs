use super::Register;
use crate::io::{inb, outb};

pub struct ScratchRegister {
    address: u16,
}

impl Register for ScratchRegister {
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
