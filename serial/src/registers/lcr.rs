use crate::ComPort;
use arch::io::{
    port::Port,
    register::{ReadRegister, Register, WriteRegister},
};

const LCR_OFFSET: u16 = 3;

pub struct LineControlRegister {
    port: Port<u8>,
}

impl LineControlRegister {
    pub fn new(address: u16) -> Self {
        LineControlRegister {
            port: Port::new(address),
        }
    }
}

impl Register for LineControlRegister {
    type Value = LineControl;
}

impl ReadRegister for LineControlRegister {
    unsafe fn read(&self) -> Self::Value {
        self.port.read().into()
    }
}

impl WriteRegister for LineControlRegister {
    unsafe fn write(&self, value: Self::Value) {
        self.port.write(value.0);
    }
}

impl From<ComPort> for LineControlRegister {
    fn from(port: ComPort) -> Self {
        LineControlRegister::new(port as u16 + LCR_OFFSET)
    }
}

impl LineControlRegister {
    /// Set or unset the divisor latch access bit.
    ///
    /// # Safety
    ///
    /// May prevent access to either transmission registers or divisor latch
    /// registers. Ensure you do not use those registers manually while
    /// manipulating this value.
    pub unsafe fn set_dlab(&self, value: bool) {
        let current = self.read();
        if value {
            self.write(LineControl(current.0 | flags::DLAB));
        } else {
            self.write(LineControl(current.0 & !flags::DLAB));
        }
    }
}

pub struct LineControl(pub u8);

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

    pub const PARITY_OFFSET: u8 = 3;
    pub const STOP_BIT_OFFSET: u8 = 2;

    #[repr(u8)]
    pub enum Parity {
        NoParity = 0 << PARITY_OFFSET,
        OddParity = 1 << PARITY_OFFSET,
        EvenParity = 3 << PARITY_OFFSET,
        Mark = 5 << PARITY_OFFSET,
        Space = 7 << PARITY_OFFSET,
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
        OneStop = 0 << STOP_BIT_OFFSET,
        TwoStop = 1 << STOP_BIT_OFFSET,
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
}

impl LineControl {
    pub fn dlab(&self) -> bool {
        self.0 & flags::DLAB != 0
    }

    pub fn break_enable(&self) -> bool {
        self.0 & flags::BREAK_ENABLE != 0
    }

    pub fn parity(&self) -> flags::Parity {
        ((self.0 & flags::PARITY) >> 3)
            .try_into()
            .expect("Invalid parity value")
    }

    pub fn stop_bits(&self) -> flags::StopBit {
        ((self.0 & flags::STOP_BIT) >> 2).into()
    }

    pub fn word_length(&self) -> flags::WordLengthBits {
        (self.0 & flags::WORD_LENGTH).into()
    }
}
