//! A module containing the operations, internal fields and flags accessible for
//! a [`InterruptEnableRegister`].
use crate::serial::ComPort;
use arch::io::{
    port::Port,
    register::{ReadRegister, Register, WriteRegister},
};

/// The offset of the [`InterruptEnableRegister`] relatively to the UART's base
/// port address.
const IER_OFFSET: u16 = 1;

/// A structure containing the informations to identify a
/// [`InterruptEnableRegister`].
pub struct InterruptEnableRegister {
    /// The port of the [`InterruptEnableRegister`].
    port: Port<u8>,
}

impl InterruptEnableRegister {
    pub fn new(address: u16) -> Self {
        InterruptEnableRegister {
            port: Port::new(address),
        }
    }
}

impl Register for InterruptEnableRegister {
    type Value = InterruptEnable;
}

impl ReadRegister for InterruptEnableRegister {
    unsafe fn read(&self) -> Self::Value {
        self.port.read().into()
    }
}

impl WriteRegister for InterruptEnableRegister {
    unsafe fn write(&self, value: Self::Value) {
        self.port.write(value.0);
    }
}

impl From<ComPort> for InterruptEnableRegister {
    fn from(port: ComPort) -> Self {
        InterruptEnableRegister::new(port as u16 + IER_OFFSET)
    }
}

/// Flags values and bitmasks for the InterruptEnableRegister's internal value.
pub mod flags {
    pub const RESERVED_1: u8 = 0b10000000;
    pub const RESERVED_2: u8 = 0b01000000;
    /// Low power mode enable/disable bitmask.
    pub const LOW_POWER_MODE: u8 = 0b00100000;
    /// Sleep mode enable/disable bitmask.
    pub const SLEEP_MODE: u8 = 0b00010000;
    /// Modem status interrupt enable/disable bitmask.
    pub const MODEM_STATUS_INTERRUPT: u8 = 0b00001000;
    /// Receiver line status enable/disable bitmask.
    pub const RECEIVER_LINE_STATUS_INTERRUPT: u8 = 0b00000100;
    /// Transmitter holding register empty interrupt enable/disable bitmask.
    pub const TRANSMITTER_HOLDING_REGISTER_EMPTY_INTERRUPT: u8 = 0b00000010;
    /// Received data available interrupt enable/disable bitmask.
    pub const RECEIVED_DATA_AVAILABLE_INTERRUPT: u8 = 0b00000001;
}

/// Internal value of the [`InterruptEnableRegister`].
pub struct InterruptEnable(pub u8);

impl From<u8> for InterruptEnable {
    fn from(value: u8) -> Self {
        InterruptEnable(value)
    }
}

impl InterruptEnable {
    /// Whether the low power mode bit is set.
    pub fn low_power_mode(&self) -> bool {
        self.0 & flags::LOW_POWER_MODE != 0
    }

    /// Whether the sleep mode bit is set.
    pub fn sleep_mode(&self) -> bool {
        self.0 & flags::SLEEP_MODE != 0
    }

    /// Whether the model status interrupt bit is set.
    pub fn modem_status_interrupt(&self) -> bool {
        self.0 & flags::MODEM_STATUS_INTERRUPT != 0
    }

    /// Whether the line status interrupt bit is set.
    pub fn receiver_line_status_interrupt(&self) -> bool {
        self.0 & flags::RECEIVER_LINE_STATUS_INTERRUPT != 0
    }

    /// Whether the holding register empty interrupt bit is set.
    pub fn transmitter_holding_register_empty_interrupt(&self) -> bool {
        self.0 & flags::TRANSMITTER_HOLDING_REGISTER_EMPTY_INTERRUPT != 0
    }

    /// Whether the data available interrupt bit is set.
    pub fn received_data_available_interrupt(&self) -> bool {
        self.0 & flags::RECEIVED_DATA_AVAILABLE_INTERRUPT != 0
    }
}
