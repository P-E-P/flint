use super::{Register, WriteRegister};
use crate::io::outb;
use crate::ComPort;

const FCR_OFFSET: u16 = 2;

pub struct FifoControlRegister {
    pub address: u16,
}

impl Register for FifoControlRegister {
    type Value = FifoControl;
}

impl WriteRegister for FifoControlRegister {
    fn write(&self, value: Self::Value) {
        unsafe {
            outb(value.0, self.address);
        }
    }
}

impl From<ComPort> for FifoControlRegister {
    fn from(port: ComPort) -> Self {
        FifoControlRegister {
            address: port as u16 + FCR_OFFSET,
        }
    }
}

pub struct FifoControl(pub u8);

impl From<u8> for FifoControl {
    fn from(value: u8) -> Self {
        FifoControl(value)
    }
}

pub mod flags {
    pub const TRIGGER_LEVEL: u8 = 0b11000000;
    pub const ENABLE_64B_FIFO: u8 = 0b00100000;
    pub const RESERVED: u8 = 0b00010000;
    pub const DMA_MODE_SELECT: u8 = 0b00001000;
    pub const CLEAR_TRANSMIT_FIFO: u8 = 0b00000100;
    pub const CLEAR_RECEIVE_FIFO: u8 = 0b00000010;
    pub const ENABLE_FIFOS: u8 = 0b00000001;

    #[repr(u8)]
    pub enum TriggerLevel {
        Itl1 = 0 << 6,
        Itl4 = 1 << 6,
        Itl8 = 2 << 6,
        Itl14 = 3 << 6,
    }

    impl From<u8> for TriggerLevel {
        fn from(value: u8) -> Self {
            match value {
                0 => TriggerLevel::Itl1,
                1 => TriggerLevel::Itl4,
                2 => TriggerLevel::Itl8,
                3 => TriggerLevel::Itl14,
                _ => panic!("Invalid value for interrupt trigger level"),
            }
        }
    }
}
