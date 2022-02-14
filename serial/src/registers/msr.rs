use super::{ReadRegister, Register};
use crate::io::inb;

pub struct ModemStatusRegister {
    address: u16,
}

impl Register for ModemStatusRegister {
    type Value = ModemStatus;
}

impl ReadRegister for ModemStatusRegister {
    fn read(&self) -> Self::Value {
        unsafe { inb(self.address).into() }
    }
}

pub struct ModemStatus(u8);

impl From<u8> for ModemStatus {
    fn from(value: u8) -> Self {
        ModemStatus(value)
    }
}

pub mod flags {
    pub const CARRIER_DETECT: u8 = 0b10000000;
    pub const RING_INDICATOR: u8 = 0b01000000;
    pub const DATA_SET_READY: u8 = 0b00100000;
    pub const CLEAR_TO_SEND: u8 = 0b00010000;
    pub const DELTA_DATA_CARRIER_DETECT: u8 = 0b00001000;
    pub const TRAILING_EDGE_RING_INDICATOR: u8 = 0b00000100;
    pub const DELTA_DATA_SET_READY: u8 = 0b00000010;
    pub const DELTA_CLEAR_TO_SEND: u8 = 0b00000001;
}

impl ModemStatus {
    pub fn carrier_detect(&self) -> bool {
        self.0 & flags::CARRIER_DETECT != 0
    }

    pub fn ring_indicator(&self) -> bool {
        self.0 & flags::RING_INDICATOR != 0
    }

    pub fn data_set_ready(&self) -> bool {
        self.0 & flags::DATA_SET_READY != 0
    }

    pub fn clear_to_send(&self) -> bool {
        self.0 & flags::CLEAR_TO_SEND != 0
    }

    pub fn delta_data_carrier_detect(&self) -> bool {
        self.0 & flags::DELTA_DATA_CARRIER_DETECT != 0
    }

    pub fn trailing_edge_ring_indicator(&self) -> bool {
        self.0 & flags::TRAILING_EDGE_RING_INDICATOR != 0
    }

    pub fn delta_data_set_ready(&self) -> bool {
        self.0 & flags::DELTA_DATA_SET_READY != 0
    }

    pub fn delta_clear_to_send(&self) -> bool {
        self.0 & flags::DELTA_CLEAR_TO_SEND != 0
    }
}
