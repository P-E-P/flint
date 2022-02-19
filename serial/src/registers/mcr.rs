use super::{ReadRegister, Register, WriteRegister};
use crate::arch::{inb, outb};
use crate::ComPort;

const MCR_OFFSET: u16 = 4;

pub struct ModemControlRegister {
    address: u16,
}

impl Register for ModemControlRegister {
    type Value = ModemControl;
}

impl ReadRegister for ModemControlRegister {
    fn read(&self) -> Self::Value {
        unsafe { inb(self.address).into() }
    }
}

impl WriteRegister for ModemControlRegister {
    fn write(&self, value: Self::Value) {
        unsafe {
            outb(value.0, self.address);
        }
    }
}

impl From<ComPort> for ModemControlRegister {
    fn from(port: ComPort) -> Self {
        ModemControlRegister {
            address: port as u16 + MCR_OFFSET,
        }
    }
}

pub struct ModemControl(u8);

impl From<u8> for ModemControl {
    fn from(value: u8) -> Self {
        ModemControl(value)
    }
}

pub mod flags {
    pub const RESERVED1: u8 = 0b1000000;
    pub const RESERVED2: u8 = 0b0100000;
    pub const AUTOFLOW_CONTROL: u8 = 0b00100000;
    pub const LOOPBACK_MODE: u8 = 0b00010000;
    pub const AUX_OUTPUT_2: u8 = 0b00001000;
    pub const AUX_OUTPUT_1: u8 = 0b00000100;
    pub const REQUEST_TO_SEND: u8 = 0b00000010;
    pub const DATA_TERMINAL_READY: u8 = 0b00000001;
}

impl ModemControl {
    pub fn autoflow_control(&self) -> bool {
        self.0 & flags::AUTOFLOW_CONTROL != 0
    }

    pub fn loopback_mode(&self) -> bool {
        self.0 & flags::LOOPBACK_MODE != 0
    }

    pub fn aux_output2(&self) -> bool {
        self.0 & flags::AUX_OUTPUT_2 != 0
    }

    pub fn aux_output1(&self) -> bool {
        self.0 & flags::AUX_OUTPUT_1 != 0
    }

    pub fn request_to_send(&self) -> bool {
        self.0 & flags::REQUEST_TO_SEND != 0
    }

    pub fn data_terminal_ready(&self) -> bool {
        self.0 & flags::DATA_TERMINAL_READY != 0
    }
}
