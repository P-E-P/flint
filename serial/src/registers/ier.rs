use super::{ReadRegister, Register, WriteRegister};
use crate::io::{inb, outb};
use crate::ComPort;

const IER_OFFSET: u16 = 1;

pub struct InterruptEnableRegister {
    pub address: u16,
}

impl Register for InterruptEnableRegister {
    type Value = InterruptEnable;
}

impl ReadRegister for InterruptEnableRegister {
    fn read(&self) -> Self::Value {
        unsafe { inb(self.address).into() }
    }
}

impl WriteRegister for InterruptEnableRegister {
    fn write(&self, value: Self::Value) {
        unsafe {
            outb(value.0, self.address);
        }
    }
}

impl From<ComPort> for InterruptEnableRegister {
    fn from(port: ComPort) -> Self {
        InterruptEnableRegister {
            address: port as u16 + IER_OFFSET,
        }
    }
}

pub mod flags {
    pub const RESERVED_1: u8 = 0b10000000;
    pub const RESERVED_2: u8 = 0b01000000;
    pub const LOW_POWER_MODE: u8 = 0b00100000;
    pub const SLEEP_MODE: u8 = 0b00010000;
    pub const MODEM_STATUS_INTERRUPT: u8 = 0b00001000;
    pub const RECEIVER_LINE_STATUS_INTERRUPT: u8 = 0b00000100;
    pub const TRANSMITTER_HOLDING_REGISTER_EMPTY_INTERRUPT: u8 = 0b00000010;
    pub const RECEIVED_DATA_AVAILABLE_INTERRUPT: u8 = 0b00000001;
}

pub struct InterruptEnable(pub u8);

impl From<u8> for InterruptEnable {
    fn from(value: u8) -> Self {
        InterruptEnable(value)
    }
}

impl InterruptEnable {
    pub fn low_power_mode(&self) -> bool {
        self.0 & flags::LOW_POWER_MODE != 0
    }

    pub fn sleep_mode(&self) -> bool {
        self.0 & flags::SLEEP_MODE != 0
    }

    pub fn modem_status_interrupt(&self) -> bool {
        self.0 & flags::MODEM_STATUS_INTERRUPT != 0
    }

    pub fn receiver_line_status_interrupt(&self) -> bool {
        self.0 & flags::RECEIVER_LINE_STATUS_INTERRUPT != 0
    }

    pub fn transmitter_holding_register_empty_interrupt(&self) -> bool {
        self.0 & flags::TRANSMITTER_HOLDING_REGISTER_EMPTY_INTERRUPT != 0
    }

    pub fn received_data_available_interrupt(&self) -> bool {
        self.0 & flags::RECEIVED_DATA_AVAILABLE_INTERRUPT != 0
    }
}
