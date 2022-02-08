use super::Register;
use crate::io::inb;

pub struct ReceiverBuffer {
    pub address: u16,
}

impl Register for ReceiverBuffer {
    type Value = u8;
    fn read(&self) -> Self::Value {
        unsafe {
            inb(self.address)
        }
    }

    fn write(&self, value: Self::Value) {
        unimplemented!("This register is read only");
    }
}
