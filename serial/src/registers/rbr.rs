use super::lcr::LineControlRegister;
use super::{ReadRegister, Register};
use crate::io::inb;

pub struct ReceiverBuffer {
    pub address: u16,
}

impl Register for ReceiverBuffer {
    type Value = u8;
}

impl ReadRegister for ReceiverBuffer {
    /// Note: For performance reason we rely on the fact that DLAB is always
    /// unset, as this function will be called many times compared to the
    /// configuration options with the DLAB bit set. Otherwise we would have to
    /// unset it in every call.
    fn read(&self) -> Self::Value {
        unsafe { inb(self.address) }
    }
}
