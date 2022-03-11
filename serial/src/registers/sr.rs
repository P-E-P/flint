use crate::ComPort;
use arch::io::{
    port::Port,
    register::{ReadRegister, Register, WriteRegister},
};

const SR_OFFSET: u16 = 7;

pub struct ScratchRegister {
    port: Port<u8>,
}

impl ScratchRegister {
    pub fn new(address: u16) -> Self {
        ScratchRegister {
            port: Port::new(address),
        }
    }
}

impl Register for ScratchRegister {
    type Value = u8;
}

impl ReadRegister for ScratchRegister {
    unsafe fn read(&self) -> Self::Value {
        self.port.read()
    }
}

impl WriteRegister for ScratchRegister {
    unsafe fn write(&self, value: Self::Value) {
        self.port.write(value);
    }
}

impl From<ComPort> for ScratchRegister {
    fn from(port: ComPort) -> Self {
        ScratchRegister::new(port as u16 + SR_OFFSET)
    }
}
