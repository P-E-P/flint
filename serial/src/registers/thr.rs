use super::lcr::LineControlRegister;
use super::{Register, WriteRegister};
use crate::io::outb;
use crate::ComPort;

const THR_OFFSET: u16 = 0;

pub struct TransmitterHoldingBuffer {
    pub address: u16,
}

impl Register for TransmitterHoldingBuffer {
    type Value = u8;
}

impl WriteRegister for TransmitterHoldingBuffer {
    /// Note: For performance reason we rely on the fact that DLAB is always
    /// unset, as this function will be called many times compared to the
    /// configuration options with the DLAB bit set. Otherwise we would have to
    /// unset it in every call.
    fn write(&self, value: Self::Value) {
        unsafe {
            outb(value, self.address);
        }
    }
}

impl From<ComPort> for TransmitterHoldingBuffer {
    fn from(port: ComPort) -> Self {
        TransmitterHoldingBuffer {
            address: port as u16 + THR_OFFSET,
        }
    }
}
