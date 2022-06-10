//! A module containing the operations, internal fields and flags accessible for
//! a [`FifoControlRegister`].

use crate::serial::ComPort;
use arch::io::{
    port::Port,
    register::{Register, WriteRegister},
};

/// The offset of the [`FifoControlRegister`] relatively to the UART's base
/// address.
pub const FCR_OFFSET: u16 = 2;

/// A structure containing the informations to identify a
/// [`FifoControlRegister`].
pub struct FifoControlRegister {
    /// The port of the [`FifoControlRegister`].
    port: Port<u8>,
}

impl FifoControlRegister {
    pub fn new(address: u16) -> Self {
        FifoControlRegister {
            port: Port::new(address),
        }
    }
}

impl Register for FifoControlRegister {
    type Value = FifoControl;
}

impl WriteRegister for FifoControlRegister {
    unsafe fn write(&self, value: Self::Value) {
        self.port.write(value.0);
    }
}

impl From<ComPort> for FifoControlRegister {
    fn from(port: ComPort) -> Self {
        FifoControlRegister::new(port as u16 + FCR_OFFSET)
    }
}

/// Internal value of the [`FifoControlRegister`].
pub struct FifoControl(pub u8);

impl From<u8> for FifoControl {
    fn from(value: u8) -> Self {
        FifoControl(value)
    }
}

/// Flags values and bitmasks for the FifoControlRegister's internal value.
pub mod flags {
    /// Trigger level bitmask.
    pub const TRIGGER_LEVEL: u8 = 0b11000000;
    /// 64 bits fifo enable/disable bitmask.
    pub const ENABLE_64B_FIFO: u8 = 0b00100000;
    pub const RESERVED: u8 = 0b00010000;
    /// DMA mode selection bitmask.
    pub const DMA_MODE_SELECT: u8 = 0b00001000;
    /// Clear transmit fifo bitmask.
    pub const CLEAR_TRANSMIT_FIFO: u8 = 0b00000100;
    /// Clear receive fifo bitmask.
    pub const CLEAR_RECEIVE_FIFO: u8 = 0b00000010;
    /// Fifo enable bitmask.
    pub const ENABLE_FIFOS: u8 = 0b00000001;

    /// An enumeration of all possible interrupt trigger level values with 16
    /// bytes mode.
    ///
    /// # Note
    ///
    /// The `u8` representation of this enum has already been shifted to the
    /// field's position.
    #[repr(u8)]
    pub enum TriggerLevel16 {
        /// Interrupt trigger level of 1 byte.
        Itl1 = 0 << 6,
        /// Interrupt trigger level of 4 bytes.
        Itl4 = 1 << 6,
        /// Interrupt trigger level of 8 bytes.
        Itl8 = 2 << 6,
        /// Interrupt trigger level of 14 bytes.
        Itl14 = 3 << 6,
    }

    /// An enumeration of all possible interrupt trigger level values with 64
    /// bytes mode.
    ///
    /// # Note
    ///
    /// The `u8` representation of this enum has already been shifted to the
    /// field's position.
    #[repr(u8)]
    pub enum TriggerLevel64 {
        /// Interrupt trigger level of 1 byte.
        Itl1 = 0 << 6,
        /// Interrupt trigger level of 16 bytes.
        Itl16 = 1 << 6,
        /// Interrupt trigger level of 32 bytes.
        Itl32 = 2 << 6,
        /// Interrupt trigger level of 56 bytes.
        Itl56 = 3 << 6,
    }

    impl From<u8> for TriggerLevel16 {
        fn from(value: u8) -> Self {
            match value {
                0 => TriggerLevel16::Itl1,
                1 => TriggerLevel16::Itl4,
                2 => TriggerLevel16::Itl8,
                3 => TriggerLevel16::Itl14,
                _ => panic!("Invalid value for interrupt trigger level"),
            }
        }
    }

    impl From<u8> for TriggerLevel64 {
        fn from(value: u8) -> Self {
            match value {
                0 => TriggerLevel64::Itl1,
                1 => TriggerLevel64::Itl16,
                2 => TriggerLevel64::Itl32,
                3 => TriggerLevel64::Itl56,
                _ => panic!("Invalid value for interrupt trigger level"),
            }
        }
    }
}
