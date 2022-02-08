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

impl TryFrom<u8> for FifoStatus {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FifoStatus::NoFifo),
            1 => Ok(FifoStatus::Reserved),
            2 => Ok(FifoStatus::EnabledNotFunctioning),
            3 => Ok(FifoStatus::Enabled),
            _ => Err("Invalid value for fifo status"),
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

impl TryFrom<u8> for InterrupEventType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(InterrupEventType::ModemStatus),
            1 => Ok(InterrupEventType::TransmitterHoldingRegisterEmpty),
            2 => Ok(InterrupEventType::ReceivedDataAvailable),
            3 => Ok(InterrupEventType::ReceiverLineStatus),
            4 => Ok(InterrupEventType::Reserved1),
            5 => Ok(InterrupEventType::Reserved2),
            6 => Ok(InterrupEventType::TimeoutInterruptPending),
            7 => Ok(InterrupEventType::Reserved3),
            _ => Err("Invalid value for interrupt event type"),
        }
    }
}

impl InterruptIdentification {
    pub fn fifo_status(&self) -> FifoStatus {
        ((self.0 & flags::FIFO_STATUS) >> 6)
            .try_into()
            .expect("Invalid fifo status value")
    }

    pub fn fifo_enabled(&self) -> bool {
        self.0 & flags::FIFO_ENABLED != 0
    }

    pub fn interrupt_event_type(&self) -> InterrupEventType {
        ((self.0 & flags::INTERRUPT_EVENT_TYPE) >> 1)
            .try_into()
            .expect("Invalid interrupt event value")
    }

    pub fn interrupt_pending(&self) -> bool {
        self.0 & flags::INTERRUPT_PENDING != 0
    }
}
