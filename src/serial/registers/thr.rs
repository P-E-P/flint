use crate::serial::ComPort;
use arch::io::{
    port::Port,
    register::{Register, WriteRegister},
};

const THR_OFFSET: u16 = 0;

pub struct TransmitterHoldingBuffer {
    port: Port<u8>,
}

impl TransmitterHoldingBuffer {
    pub fn new(address: u16) -> Self {
        TransmitterHoldingBuffer {
            port: Port::new(address),
        }
    }
}

impl Register for TransmitterHoldingBuffer {
    type Value = u8;
}

impl WriteRegister for TransmitterHoldingBuffer {
    /// Write the content to the UART's register.
    ///
    /// # Safety
    ///
    /// For performance reason we rely on the fact that DLAB is always
    /// unset, as this function will be called many times compared to the
    /// configuration options with the DLAB bit set. Otherwise we would have to
    /// unset it in every call.
    unsafe fn write(&self, value: Self::Value) {
        self.port.write(value);
    }
}

impl From<ComPort> for TransmitterHoldingBuffer {
    fn from(port: ComPort) -> Self {
        TransmitterHoldingBuffer::new(port as u16 + THR_OFFSET)
    }
}
