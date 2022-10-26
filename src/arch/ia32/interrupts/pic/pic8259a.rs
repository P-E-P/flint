//! A module for the 8259 PIC.
use crate::arch::io::port::Port;
use crate::arch::io::register::WriteRegister;
use crate::utils::bitfield::*;

/// Master and Slave PICs ports on IBM PC
pub const MASTER_PIC_COM: u16 = 0x20;
pub const SLAVE_PIC_COM: u16 = 0xA0;

// PIC IRQs mapping
pub const PIT_IRQ: usize = 0;
pub const KEYBOARD_IRQ: usize = 1;

/// An enum defining how interrupts are triggered.
#[derive(PartialEq, Eq)]
pub enum TriggerMode {
    /// Level triggered interrupts
    Level,
    /// Edge triggered interrupts
    Edge,
}

impl From<TriggerMode> for bool {
    fn from(value: TriggerMode) -> Self {
        value == TriggerMode::Level
    }
}

/// An enum defining the CALL Address Interval.
#[derive(PartialEq, Eq)]
pub enum AddressInterval {
    Four,
    Eight,
}

impl From<AddressInterval> for bool {
    fn from(value: AddressInterval) -> Self {
        value == AddressInterval::Four
    }
}

/// A structure representing the first initialisation command for a 8259 PIC.
#[derive(Clone, Copy)]
struct ICW1(u8);

impl ICW1 {
    /// Creates a [`ICW1`] byte from parameters.
    ///
    /// # Arguments
    ///
    /// * `has_icw4` - If true, ICW4 will be expected in initialisation phase.
    /// * `single` - Single mode (true) or Cascaded mode (false).
    /// * `adi` - CALL Address Inverval
    /// * `ltim` - Level Triggered mode or Edge Triggered mode.
    pub fn new(has_icw4: bool, single: bool, adi: AddressInterval, ltim: TriggerMode) -> Self {
        Self(
            0_u8.set_bit(0, has_icw4)
                .set_bit(1, single)
                .set_bit(2, adi.into())
                .set_bit(3, ltim.into())
                .set_bit(4, true),
        )
    }
}

impl From<ICW1> for u8 {
    fn from(value: ICW1) -> Self {
        value.0
    }
}

/// Enum to describe the cascading role of the 8259 PIC.
#[derive(PartialEq)]
enum PICRole {
    /// No cascading.
    Single,
    /// Slave in cascading.
    Slave,
    /// Master in cascading.
    Master,
}

/// Struct representing a 8259 PIC
pub struct PIC8259a {
    /// Command port of the chip.
    com_port: Port<u8>,
    /// Data port of the chip.
    data_port: Port<u8>,
    /// Pin connected to Master/Slave. None if single mode.
    pin: Option<u8>,
    /// PIC IRQs offset in Interrupt Descriptor Table.
    offset: u8,
    /// PIC Role.
    role: PICRole,
}

impl PIC8259a {
    /// Creates a [`PIC8259a`]'s representation in Single mode.
    ///
    /// # Arguments
    ///
    /// * `addr` - PIC Command port address. Data port address will be deduced.
    /// * `offset` - PIC IRQs offset in Interrupt Descriptor Table.
    const fn single(addr: u16, offset: u8) -> Self {
        if offset % 8 != 0 {
            panic!("8259a: offset must be a multiple of 8.");
        }

        Self {
            com_port: Port::new(addr),
            data_port: Port::new(addr + 1),
            pin: None,
            offset,
            role: PICRole::Single,
        }
    }

    /// Creates a [`PIC8259a`]'s representation in Master mode.
    ///
    /// # Arguments
    ///
    /// * `addr` - PIC Command port address. Data port address will be deduced.
    /// * `slave_pin` - The pin on which the PIC slave is connected to.
    /// * `offset` - PIC IRQs offset in Interrupt Descriptor Table.
    pub const fn master(addr: u16, slave_pin: u8, offset: u8) -> Self {
        if slave_pin >= 8 {
            panic!("8259a: pin must be between 0 and 7");
        }

        if offset % 8 != 0 {
            panic!("8259a: offset must be a multiple of 8.");
        }

        Self {
            com_port: Port::new(addr),
            data_port: Port::new(addr + 1),
            pin: Some(slave_pin),
            offset,
            role: PICRole::Master,
        }
    }

    /// Creates a [`PIC8259a`]'s representation in Slave mode.
    ///
    /// # Arguments
    ///
    /// * `addr` - PIC Command port address. Data port address will be deduced.
    /// * `slave_pin` - The pin on which the master PIC is connected to.
    /// * `offset` - PIC IRQs offset in Interrupt Descriptor Table.
    pub const fn slave(addr: u16, slave_pin: u8, offset: u8) -> Self {
        if slave_pin >= 8 {
            panic!("8259a: pin must be between 0 and 7");
        }

        if offset % 8 != 0 {
            panic!("8259a: offset must be a multiple of 8.");
        }

        Self {
            com_port: Port::new(addr),
            data_port: Port::new(addr + 1),
            pin: Some(slave_pin),
            offset,
            role: PICRole::Slave,
        }
    }

    /// Sends the Initialisation Command Word 1.
    ///
    /// # Arguments
    ///
    /// * `has_icw4` - If true, ICW4 will be expected.
    /// * `adi` - CALL Address Inverval
    /// * `ltim` - Level Triggered mode or Edge Triggered mode.
    ///
    /// # Safety
    ///
    /// The arguments you are providing should be consistent with other commands you are sending.
    pub unsafe fn send_icw1(&self, has_icw4: bool, adi: AddressInterval, ltim: TriggerMode) {
        let com = ICW1::new(has_icw4, self.role == PICRole::Single, adi, ltim);
        self.com_port.write(com.into());
    }

    /// Sends the Initialisation Command Word 2.
    ///
    /// # Safety
    ///
    /// The arguments you are providing should be consistent with other commands you are sending.
    pub unsafe fn send_icw2(&self) {
        self.data_port.write(self.offset);
    }

    /// Sends the Initialisation Command Word 3.
    ///
    /// # Safety
    ///
    /// The arguments you are providing should be consistent with other commands you are sending.
    pub unsafe fn send_icw3(&self) {
        match self.role {
            PICRole::Single => panic!("ICW3: Nothing to be sent in single mode."),
            PICRole::Master => self
                .data_port
                .write(0_u8.set_bit(self.pin.unwrap().into(), true)),
            PICRole::Slave => self.data_port.write(self.pin.unwrap()),
        };
    }

    /// Sends the Initialisation Command Word 4.
    ///
    /// # Arguments
    ///
    /// * `aeoi` - Auto EOI mode.
    /// * `buffered` - Buffered mode.
    /// * `fully_nested` - Special fully nested mode.
    ///
    /// # Safety
    ///
    /// The arguments you are providing should be consistent with other commands you are sending.
    pub unsafe fn send_icw4(&self, aeoi: bool, buffered: bool, fully_nested: bool) {
        let com = 0_u8
            .set_bit(0, true)
            .set_bit(1, aeoi)
            .set_bit(2, self.role == PICRole::Master)
            .set_bit(3, buffered)
            .set_bit(4, fully_nested);
        self.data_port.write(com);
    }

    /// Send Operational Command Word 1.
    ///
    /// # Arguments
    ///
    /// * `mask` - IRQs bitmask (1 = IRQ disabled).
    ///
    /// # Safety
    ///
    /// The arguments you are providing should be consistent with other commands you are sending.
    pub unsafe fn send_ocw1(&self, mask: u8) {
        self.data_port.write(mask);
    }

    /// Acknowledge End Of Interrupt on PIC.
    ///
    /// # Safety
    ///
    /// Interrupt source should be verified.
    pub unsafe fn ack_eoi(&self) {
        self.com_port.write(0x20_u8);
    }
}
