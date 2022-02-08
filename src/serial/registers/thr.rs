use super::Register;
use crate::io::outb;

pub struct TransmitterHoldingBuffer {
    pub address: u16,
}

impl Register for TransmitterHoldingBuffer {
    type Value = u8;
    fn read(&self) -> Self::Value {
        unimplemented!("This register is write only");
    }

    fn write(&self, value: Self::Value) {
        unsafe {
            outb(value, self.address);
        }
    }
}
