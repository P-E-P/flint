use super::Register;
use crate::io::outb;

pub struct FifoControlRegister {
    address: u16,
}

impl Register for FifoControlRegister {
    type Value = FifoControl;

    fn read(&self) -> Self::Value {
        unimplemented!("This register is write only!");
    }

    fn write(&self, value: Self::Value) {
        unsafe {
            outb(value.0, self.address);
        }
    }
}

pub struct FifoControl(u8);

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
}

#[repr(u8)]
pub enum TriggerLevel {
    Itl1 = 0,
    Itl4 = 1,
    Itl8 = 2,
    Itl14 = 3,
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
