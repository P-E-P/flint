//! A module containing the operations, internal fields and flags aaccessible
//! for an [`InterruptIdentificationRegister`].
use crate::arch::io::{
    port::Port,
    register::{ReadRegister, Register},
};
use crate::serial::ComPort;

/// The offset of the [`InterruptIdentificationRegister`] relatively to the
/// UART's base port address.
const IIR_OFFSET: u16 = 2;

/// A structure containing the informations to identify an
/// [`InterruptIdentificationRegister`].
pub struct InterruptIdentificationRegister {
    port: Port<u8>,
}

impl InterruptIdentificationRegister {
    pub fn new(address: u16) -> Self {
        InterruptIdentificationRegister {
            port: Port::new(address),
        }
    }
}

impl Register for InterruptIdentificationRegister {
    type Value = InterruptIdentification;
}

impl ReadRegister for InterruptIdentificationRegister {
    unsafe fn read(&self) -> Self::Value {
        self.port.read().into()
    }
}

impl From<ComPort> for InterruptIdentificationRegister {
    fn from(port: ComPort) -> Self {
        InterruptIdentificationRegister::new(port as u16 + IIR_OFFSET)
    }
}

/// Internal value of the [`InterruptIdentificationRegister`].
pub struct InterruptIdentification(u8);

impl From<u8> for InterruptIdentification {
    fn from(value: u8) -> Self {
        InterruptIdentification(value)
    }
}

/// Flags values and bitmasks for the InterruptIdentificationRegister's internal
/// value.
pub mod flags {
    /// Fifo status bitmask.
    pub const FIFO_STATUS: u8 = 0b11000000;
    /// 64 byte fifo enabled bitmask.
    pub const FIFO_ENABLED: u8 = 0b00100000;
    pub const RESERVED: u8 = 0b00010000;
    /// Interrupt even type bitmask.
    pub const INTERRUPT_EVENT_TYPE: u8 = 0b00001110;
    /// Interrupt pending bitmask.
    pub const INTERRUPT_PENDING: u8 = 0b00000001;

    /// [`FifoStatus`] field's offset.
    pub const FIFO_STATUS_OFFSET: u8 = 6;
    /// [`InterruptEventType`] field's offset.
    pub const INTERUPT_EVENT_TYPE_OFFSET: u8 = 1;

    /// An enumeration of all possible fifo status values.
    ///
    /// # Note
    ///
    /// The `u8` representation of this enum has already been shifted to the
    /// field's position.
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

    /// An enumeration of all possible interrupt event type values.
    ///
    /// # Note
    ///
    /// The `u8` representation of this enum has already been shifted to the
    /// field's position.
    #[repr(u8)]
    pub enum InterruptEventType {
        ModemStatus = 0 << INTERUPT_EVENT_TYPE_OFFSET,
        TransmitterHoldingRegisterEmpty = 1 << INTERUPT_EVENT_TYPE_OFFSET,
        ReceivedDataAvailable = 2 << INTERUPT_EVENT_TYPE_OFFSET,
        ReceiverLineStatus = 3 << INTERUPT_EVENT_TYPE_OFFSET,
        Reserved1 = 4 << INTERUPT_EVENT_TYPE_OFFSET,
        Reserved2 = 5 << INTERUPT_EVENT_TYPE_OFFSET,
        TimeoutInterruptPending = 6 << INTERUPT_EVENT_TYPE_OFFSET,
        Reserved3 = 7 << INTERUPT_EVENT_TYPE_OFFSET,
    }

    impl From<u8> for InterruptEventType {
        fn from(value: u8) -> Self {
            match value {
                0 => InterruptEventType::ModemStatus,
                1 => InterruptEventType::TransmitterHoldingRegisterEmpty,
                2 => InterruptEventType::ReceivedDataAvailable,
                3 => InterruptEventType::ReceiverLineStatus,
                4 => InterruptEventType::Reserved1,
                5 => InterruptEventType::Reserved2,
                6 => InterruptEventType::TimeoutInterruptPending,
                7 => InterruptEventType::Reserved3,
                _ => panic!("Invalid value for interrupt event type"),
            }
        }
    }
}

impl InterruptIdentification {
    /// Get the [`flags::FifoStatus`]'s field from the register value.
    pub fn fifo_status(&self) -> flags::FifoStatus {
        ((self.0 & flags::FIFO_STATUS) >> flags::FIFO_STATUS_OFFSET).into()
    }

    /// Whether the fifo has been enabled.
    pub fn fifo_enabled(&self) -> bool {
        self.0 & flags::FIFO_ENABLED != 0
    }

    /// Get the [`flags::InterruptEventType`]'s field from the register value.
    pub fn interrupt_event_type(&self) -> flags::InterruptEventType {
        ((self.0 & flags::INTERRUPT_EVENT_TYPE) >> flags::INTERUPT_EVENT_TYPE_OFFSET).into()
    }

    /// Whether an interrupt is pending.
    pub fn interrupt_pending(&self) -> bool {
        self.0 & flags::INTERRUPT_PENDING != 0
    }
}
