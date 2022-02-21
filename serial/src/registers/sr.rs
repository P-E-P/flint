use crate::ComPort;
use arch::io::{
    port::Port,
    register::{ReadRegister, Register, WriteRegister},
};

const SR_OFFSET: u16 = 7;

pub struct ScratchRegister {
    address: u16,
}

impl Register for ScratchRegister {
    type Value = u8;
}

impl ReadRegister for ScratchRegister {
    fn read(&self) -> Self::Value {
        Port::<u8>::new(self.address).read()
    }
}

impl WriteRegister for ScratchRegister {
    fn write(&self, value: Self::Value) {
        Port::<u8>::new(self.address).write(value);
    }
}

impl From<ComPort> for ScratchRegister {
    fn from(port: ComPort) -> Self {
        ScratchRegister {
            address: port as u16 + SR_OFFSET,
        }
    }
}
