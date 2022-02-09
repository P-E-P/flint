use super::{ReadRegister, Register, WriteRegister};
use crate::io::{inb, outb};

pub struct LineControlRegister {
    address: u16,
}

impl Register for LineControlRegister {
    type Value = LineControl;
}

impl ReadRegister for LineControlRegister {
    fn read(&self) -> Self::Value {
        unsafe { inb(self.address).into() }
    }
}

impl WriteRegister for LineControlRegister {
    fn write(&self, value: Self::Value) {
        unsafe {
            outb(value.0, self.address);
        }
    }
}

pub struct LineControl(u8);

impl From<u8> for LineControl {
    fn from(value: u8) -> Self {
        LineControl(value)
    }
}

pub mod flags {
    pub const DLAB: u8 = 0b10000000;
    pub const BREAK_ENABLE: u8 = 0b01000000;
    pub const PARITY: u8 = 0b00111000;
    pub const STOP_BIT: u8 = 0b00000100;
    pub const WORD_LENGTH: u8 = 0b00000011;
}

#[repr(u8)]
pub enum Parity {
    NoParity = 0,
    OddParity = 1,
    EvenParity = 3,
    Mark = 5,
    Space = 7,
}

impl TryFrom<u8> for Parity {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Parity::NoParity),
            1 => Ok(Parity::OddParity),
            3 => Ok(Parity::EvenParity),
            5 => Ok(Parity::Mark),
            7 => Ok(Parity::Space),
            _ => Err("Invalid value for parity selection."),
        }
    }
}

#[repr(u8)]
pub enum StopBit {
    OneStop = 0,
    TwoStop = 1,
}

impl From<u8> for StopBit {
    fn from(value: u8) -> Self {
        match value {
            0 => StopBit::OneStop,
            1 => StopBit::TwoStop,
            _ => panic!("Invalid value for stop bits."),
        }
    }
}

#[repr(u8)]
pub enum WordLengthBits {
    Five = 0,
    Six = 1,
    Seven = 2,
    Eight = 3,
}

impl From<u8> for WordLengthBits {
    fn from(value: u8) -> Self {
        match value {
            0 => WordLengthBits::Five,
            1 => WordLengthBits::Six,
            2 => WordLengthBits::Seven,
            3 => WordLengthBits::Eight,
            _ => panic!("Invalid value for word length"),
        }
    }
}

impl LineControl {
    pub fn dlab(&self) -> bool {
        self.0 & flags::DLAB != 0
    }

    pub fn break_enable(&self) -> bool {
        self.0 & flags::BREAK_ENABLE != 0
    }

    pub fn parity(&self) -> Parity {
        ((self.0 & flags::PARITY) >> 3)
            .try_into()
            .expect("Invalid parity value")
    }

    pub fn stop_bits(&self) -> StopBit {
        ((self.0 & flags::STOP_BIT) >> 2).into()
    }

    pub fn word_length(&self) -> WordLengthBits {
        (self.0 & flags::WORD_LENGTH).into()
    }
}
