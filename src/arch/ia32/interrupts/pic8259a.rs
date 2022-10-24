//! A module for the 8259 PIC.
use crate::arch::io::port::Port;
use crate::arch::io::register::WriteRegister;
use crate::utils::bitfield::*;
use log::trace;

/// Master and Slave PICs ports on IBM PC
const MASTER_PIC_COM: u16 = 0x20;
const SLAVE_PIC_COM: u16 = 0xA0;

/// PIC IRQs offset in IDT
pub const PIC_OFFSET: usize = 32;
// PIC IRQs mapping
pub const PIT_IRQ: usize = 0;
pub const KEYBOARD_IRQ: usize = 1;

/// An enum defining how interrupts are triggered.
#[derive(PartialEq)]
enum TriggerMode {
    /// Level triggered interrupts
    LEVEL,
    /// Edge triggered interrupts
    EDGE,
}

impl From<TriggerMode> for bool {
    fn from(value: TriggerMode) -> Self {
        value == TriggerMode::LEVEL
    }
}

/// An enum defining the CALL Address Interval.
#[derive(PartialEq)]
enum AddressInterval {
    FOUR,
    EIGHT
}

impl From<AddressInterval> for bool {
    fn from(value: AddressInterval) -> Self {
        value == AddressInterval::FOUR
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
    SINGLE,
    /// Slave in cascading.
    SLAVE,
    /// Master in cascading.
    MASTER,
}

/// Struct representing a 8259 PIC
struct PIC8259a {
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
            offset: offset,
            role: PICRole::SINGLE,
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
            offset: offset,
            role: PICRole::MASTER,
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
            offset: offset,
            role: PICRole::SLAVE,
        }
    }

    /// Sends the Initialisation Command Word 1.
    ///
    /// # Arguments
    ///
    /// * `has_icw4` - If true, ICW4 will be expected.
    /// * `adi` - CALL Address Inverval
    /// * `ltim` - Level Triggered mode or Edge Triggered mode.
    pub unsafe fn send_icw1(&self, has_icw4: bool, adi: AddressInterval, ltim: TriggerMode) {
        let com = ICW1::new(has_icw4, self.role == PICRole::SINGLE, adi, ltim);
        self.com_port.write(com.into());
    }

    /// Sends the Initialisation Command Word 2.
    pub unsafe fn send_icw2(&self) {
        self.data_port.write(self.offset);
    }

    /// Sends the Initialisation Command Word 3.
    pub unsafe fn send_icw3(&self) {
        match self.role {
            PICRole::SINGLE => panic!("ICW3: Nothing to be sent in single mode."),
            PICRole::MASTER => self
                .data_port
                .write(0_u8.set_bit(self.pin.unwrap().into(), true)),
            PICRole::SLAVE => self.data_port.write(self.pin.unwrap()),
        };
    }

    /// Sends the Initialisation Command Word 4.
    ///
    /// # Arguments
    ///
    /// * `aeoi` - Auto EOI mode.
    /// * `buffered` - Buffered mode.
    /// * `fully_nested` - Special fully nested mode.
    pub unsafe fn send_icw4(&self, aeoi: bool, buffered: bool, fully_nested: bool) {
        let com = 0_u8
            .set_bit(0, true)
            .set_bit(1, aeoi)
            .set_bit(2, self.role == PICRole::MASTER)
            .set_bit(3, buffered)
            .set_bit(4, fully_nested);
        self.data_port.write(com);
    }

    /// Send Operational Command Word 1.
    ///
    /// # Arguments
    ///
    /// * `mask` - IRQs bitmask (1 = IRQ disabled).
    pub unsafe fn send_ocw1(&self, mask: u8) {
        self.data_port.write(mask);
    }

    /// Acknowledge End Of Interrupt on PIC.
    pub unsafe fn ack_eoi(&self) {
        self.com_port.write(0x20_u8);
    }
}

static MASTER_PIC: PIC8259a = PIC8259a::master(MASTER_PIC_COM, 2, PIC_OFFSET as u8);
static SLAVE_PIC: PIC8259a = PIC8259a::slave(SLAVE_PIC_COM, 2, (PIC_OFFSET + 8) as u8);

/// Setup both Master and Slave PICs. Sends Initialisation Commands and IRQs masks.
///
/// # Arguments
///
/// * `master_mask` - Master PIC IRQs bitmask (1 = IRQ disabled).
/// * `slave_mask` - Slave PIC IRQs bitmask (1 = IRQ disabled).
pub fn setup(master_mask: u8, slave_mask: u8) {
    trace!("Setting up 8259a PICs...");

    unsafe {
        MASTER_PIC.send_icw1(true, AddressInterval::EIGHT, TriggerMode::EDGE);
        SLAVE_PIC.send_icw1(true, AddressInterval::EIGHT, TriggerMode::EDGE);

        MASTER_PIC.send_icw2();
        SLAVE_PIC.send_icw2();

        MASTER_PIC.send_icw3();
        SLAVE_PIC.send_icw3();

        MASTER_PIC.send_icw4(false, false, false);
        SLAVE_PIC.send_icw4(false, false, false);

        MASTER_PIC.send_ocw1(master_mask);
        SLAVE_PIC.send_ocw1(slave_mask);
    }
}

/// Acknowledge End Of Interrupt on Master PIC or both Master and Slave PICs
/// depending on the IRQ index.
///
/// # Arguments
///
/// * `irq` - IRQ index of the EOI to acknowledge.
pub unsafe fn ack_eoi(irq: u8) {
    if irq >= 8 {
        SLAVE_PIC.ack_eoi();
    }
    MASTER_PIC.ack_eoi();
}
