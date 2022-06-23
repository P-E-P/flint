use crate::arch::io::{
    port::Port,
    register::{ReadRegister, Register},
};
use crate::serial::ComPort;

const LSR_OFFSET: u16 = 5;

pub struct LineStatusRegister {
    port: Port<u8>,
}

impl LineStatusRegister {
    pub fn new(address: u16) -> Self {
        LineStatusRegister {
            port: Port::new(address),
        }
    }
}

impl Register for LineStatusRegister {
    type Value = LineStatus;
}

impl ReadRegister for LineStatusRegister {
    unsafe fn read(&self) -> Self::Value {
        self.port.read().into()
    }
}

impl From<ComPort> for LineStatusRegister {
    fn from(port: ComPort) -> Self {
        LineStatusRegister::new(port as u16 + LSR_OFFSET)
    }
}

pub mod flags {
    pub const ERROR_RECEIVED_FIFO: u8 = 0b10000000;
    pub const EMPTY_DATA_HOLDING_REGISTER: u8 = 0b01000000;
    pub const EMPTY_TRANSMITTER_HOLDING_REGISTER: u8 = 0b00100000;
    pub const BREAK_INTERRUPT: u8 = 0b00010000;
    pub const FRAMING_ERROR: u8 = 0b00001000;
    pub const PARITY_ERROR: u8 = 0b00000100;
    pub const OVERRUN_ERROR: u8 = 0b00000010;
    pub const DATA_READY: u8 = 0b00000001;
}

pub struct LineStatus(u8);

impl From<u8> for LineStatus {
    fn from(status: u8) -> Self {
        LineStatus(status)
    }
}

impl LineStatus {
    pub fn error_received_fifo(&self) -> bool {
        use flags::ERROR_RECEIVED_FIFO;
        self.0 & ERROR_RECEIVED_FIFO != 0
    }

    pub fn empty_data_holding_registers(&self) -> bool {
        use flags::EMPTY_DATA_HOLDING_REGISTER;
        self.0 & EMPTY_DATA_HOLDING_REGISTER != 0
    }

    pub fn empty_transmitter_holding_register(&self) -> bool {
        use flags::EMPTY_TRANSMITTER_HOLDING_REGISTER;
        self.0 & EMPTY_TRANSMITTER_HOLDING_REGISTER != 0
    }

    pub fn break_interrupt(&self) -> bool {
        use flags::BREAK_INTERRUPT;
        self.0 & BREAK_INTERRUPT != 0
    }

    pub fn framing_error(&self) -> bool {
        use flags::FRAMING_ERROR;
        self.0 & FRAMING_ERROR != 0
    }

    pub fn parity_error(&self) -> bool {
        use flags::PARITY_ERROR;
        self.0 & PARITY_ERROR != 0
    }

    pub fn overrun_error(&self) -> bool {
        use flags::OVERRUN_ERROR;
        self.0 & OVERRUN_ERROR != 0
    }

    pub fn data_ready(&self) -> bool {
        use flags::DATA_READY;
        self.0 & DATA_READY != 0
    }
}
