use super::Register;
use crate::io::inb;

pub struct LineStatusRegister {
    pub address: u16,
}

impl Register for LineStatusRegister {
    type Value = LineStatus;
    fn read(&self) -> Self::Value {
        unsafe {
            inb(self.address).into()
        }
    }

    fn write(&self, value: Self::Value) {
        unimplemented!("This register is read only");
    }
}

#[repr(u8)]
pub enum LineStatusField {
    ErrorReceivedFifo = 0b10000000,
    EmptyDataHoldingRegister = 0b01000000,
    EmptyTransmitterHoldingRegister = 0b00100000,
    BreakInterrupt = 0b00010000,
    FramingError = 0b00001000,
    ParityError = 0b00000100,
    OverrunError = 0b00000010,
    DataReady = 0b00000001,
}

pub struct LineStatus(u8);

impl From<u8> for LineStatus {
    fn from(status: u8) -> Self {
        LineStatus(status)
    }
}

impl LineStatus {
    pub fn error_received_fifo(&self) -> bool {
        use LineStatusField::ErrorReceivedFifo;
        self.0 & (ErrorReceivedFifo as u8) != 0
    }

    pub fn empty_data_holding_registers(&self) -> bool {
        use LineStatusField::EmptyDataHoldingRegister;
        self.0 & (EmptyDataHoldingRegister as u8) != 0
    }

    pub fn empty_transmitter_holding_register(&self) -> bool {
        use LineStatusField::EmptyTransmitterHoldingRegister;
        self.0 & (EmptyTransmitterHoldingRegister as u8) != 0
    }

    pub fn break_interrupt(&self) -> bool {
        use LineStatusField::BreakInterrupt;
        self.0 & (BreakInterrupt as u8) != 0
    }

    pub fn framing_error(&self) -> bool {
        use LineStatusField::FramingError;
        self.0 & (FramingError as u8) != 0
    }

    pub fn parity_error(&self) -> bool {
        use LineStatusField::ParityError;
        self.0 & (ParityError as u8) != 0
    }

    pub fn overrun_error(&self) -> bool {
        use LineStatusField::OverrunError;
        self.0 & (OverrunError as u8) != 0
    }

    pub fn data_ready(&self) -> bool {
        use LineStatusField::DataReady;
        self.0 & (DataReady as u8) != 0
    }
}
