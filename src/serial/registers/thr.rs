use super::{Register, WriteRegister};
use crate::io::outb;

pub struct TransmitterHoldingBuffer {
    pub address: u16,
}

impl Register for TransmitterHoldingBuffer {
    type Value = u8;
}

impl WriteRegister for TransmitterHoldingBuffer {
    fn write(&self, value: Self::Value) {
        unsafe {
            outb(value, self.address);
        }
    }
}
