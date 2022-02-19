use super::{ReadRegister, Register, WriteRegister};
use crate::arch::{inb, outb};
use crate::ComPort;

const SR_OFFSET: u16 = 7;

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

impl From<ComPort> for ScratchRegister {
    fn from(port: ComPort) -> Self {
        ScratchRegister {
            address: port as u16 + SR_OFFSET,
        }
    }
}
