use super::lcr::LineControlRegister;
use super::{ReadRegister, Register};
use crate::io::inb;

pub struct ReceiverBuffer {
    pub address: u16,
    pub lcr: LineControlRegister,
}

impl Register for ReceiverBuffer {
    type Value = u8;
}

impl ReadRegister for ReceiverBuffer {
    fn read(&self) -> Self::Value {
        unsafe { inb(self.address) }
    }
}
