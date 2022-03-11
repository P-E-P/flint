use crate::ComPort;
use arch::io::{
    port::Port,
    register::{ReadRegister, Register},
};

const RBR_OFFSET: u16 = 0;

pub struct ReceiverBuffer {
    port: Port<u8>,
}

impl ReceiverBuffer {
    pub fn new(address: u16) -> Self {
        ReceiverBuffer {
            port: Port::new(address),
        }
    }
}

impl Register for ReceiverBuffer {
    type Value = u8;
}

impl ReadRegister for ReceiverBuffer {
    /// Note: For performance reason we rely on the fact that DLAB is always
    /// unset, as this function will be called many times compared to the
    /// configuration options with the DLAB bit set. Otherwise we would have to
    /// unset it in every call.
    unsafe fn read(&self) -> Self::Value {
        self.port.read()
    }
}

impl From<ComPort> for ReceiverBuffer {
    fn from(port: ComPort) -> Self {
        ReceiverBuffer::new(port as u16 + RBR_OFFSET)
    }
}
