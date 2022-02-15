use super::{ReadRegister, Register};
use crate::io::inb;
use crate::ComPort;

const IIR_OFFSET: u16 = 2;

pub struct InterruptIdentificationRegister {
    pub address: u16,
}

impl Register for InterruptIdentificationRegister {
    type Value = InterruptIdentification;
}

impl ReadRegister for InterruptIdentificationRegister {
    fn read(&self) -> Self::Value {
        unsafe { inb(self.address).into() }
    }
}

impl From<ComPort> for InterruptIdentificationRegister {
    fn from(port: ComPort) -> Self {
        InterruptIdentificationRegister {
            address: port as u16 + IIR_OFFSET,
        }
    }
}

pub struct InterruptIdentification(u8);

impl From<u8> for InterruptIdentification {
    fn from(value: u8) -> Self {
        InterruptIdentification(value)
    }
}

pub mod flags {
    pub const FIFO_STATUS: u8 = 0b11000000;
    pub const FIFO_ENABLED: u8 = 0b00100000;
    pub const RESERVED: u8 = 0b00010000;
    pub const INTERRUPT_EVENT_TYPE: u8 = 0b00001110;
    pub const INTERRUPT_PENDING: u8 = 0b00000001;

    pub const FIFO_STATUS_OFFSET: u8 = 6;
    pub const INTERUPT_EVENT_TYPE_OFFSET: u8 = 1;
    #[repr(u8)]
    pub enum FifoStatus {
        NoFifo = 0 << FIFO_STATUS_OFFSET,
        Reserved = 1 << FIFO_STATUS_OFFSET,
        EnabledNotFunctioning = 2 << FIFO_STATUS_OFFSET,
        Enabled = 3 << FIFO_STATUS_OFFSET,
    }

    impl From<u8> for FifoStatus {
        fn from(value: u8) -> Self {
            match value {
                0 => FifoStatus::NoFifo,
                1 => FifoStatus::Reserved,
                2 => FifoStatus::EnabledNotFunctioning,
                3 => FifoStatus::Enabled,
                _ => panic!("Invalid value for fifo status"),
            }
        }
    }

    #[repr(u8)]
    pub enum InterrupEventType {
        ModemStatus = 0 << INTERUPT_EVENT_TYPE_OFFSET,
        TransmitterHoldingRegisterEmpty = 1 << INTERUPT_EVENT_TYPE_OFFSET,
        ReceivedDataAvailable = 2 << INTERUPT_EVENT_TYPE_OFFSET,
        ReceiverLineStatus = 3 << INTERUPT_EVENT_TYPE_OFFSET,
        Reserved1 = 4 << INTERUPT_EVENT_TYPE_OFFSET,
        Reserved2 = 5 << INTERUPT_EVENT_TYPE_OFFSET,
        TimeoutInterruptPending = 6 << INTERUPT_EVENT_TYPE_OFFSET,
        Reserved3 = 7 << INTERUPT_EVENT_TYPE_OFFSET,
    }

    impl From<u8> for InterrupEventType {
        fn from(value: u8) -> Self {
            match value {
                0 => InterrupEventType::ModemStatus,
                1 => InterrupEventType::TransmitterHoldingRegisterEmpty,
                2 => InterrupEventType::ReceivedDataAvailable,
                3 => InterrupEventType::ReceiverLineStatus,
                4 => InterrupEventType::Reserved1,
                5 => InterrupEventType::Reserved2,
                6 => InterrupEventType::TimeoutInterruptPending,
                7 => InterrupEventType::Reserved3,
                _ => panic!("Invalid value for interrupt event type"),
            }
        }
    }
}

impl InterruptIdentification {
    pub fn fifo_status(&self) -> flags::FifoStatus {
        ((self.0 & flags::FIFO_STATUS) >> flags::FIFO_STATUS_OFFSET).into()
    }

    pub fn fifo_enabled(&self) -> bool {
        self.0 & flags::FIFO_ENABLED != 0
    }

    pub fn interrupt_event_type(&self) -> flags::InterrupEventType {
        ((self.0 & flags::INTERRUPT_EVENT_TYPE) >> flags::INTERUPT_EVENT_TYPE_OFFSET).into()
    }

    pub fn interrupt_pending(&self) -> bool {
        self.0 & flags::INTERRUPT_PENDING != 0
    }
}
