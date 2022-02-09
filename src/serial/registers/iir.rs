use super::Register;
use crate::io::inb;

pub struct InterruptIdentificationRegister {
    pub address: u16,
}

impl Register for InterruptIdentificationRegister {
    type Value = InterruptIdentification;

    fn read(&self) -> Self::Value {
        unsafe { inb(self.address).into() }
    }

    fn write(&self, value: Self::Value) {
        unimplemented!("This register is read only!");
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
}

#[repr(u8)]
pub enum FifoStatus {
    NoFifo = 0,
    Reserved = 1,
    EnabledNotFunctioning = 2,
    Enabled = 3,
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
    ModemStatus = 0,
    TransmitterHoldingRegisterEmpty = 1,
    ReceivedDataAvailable = 2,
    ReceiverLineStatus = 3,
    Reserved1 = 4,
    Reserved2 = 5,
    TimeoutInterruptPending = 6,
    Reserved3 = 7,
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

impl InterruptIdentification {
    pub fn fifo_status(&self) -> FifoStatus {
        ((self.0 & flags::FIFO_STATUS) >> 6).into()
    }

    pub fn fifo_enabled(&self) -> bool {
        self.0 & flags::FIFO_ENABLED != 0
    }

    pub fn interrupt_event_type(&self) -> InterrupEventType {
        ((self.0 & flags::INTERRUPT_EVENT_TYPE) >> 1).into()
    }

    pub fn interrupt_pending(&self) -> bool {
        self.0 & flags::INTERRUPT_PENDING != 0
    }
}
