use crate::arch::io::port::Port;
use crate::arch::io::register::WriteRegister;
use crate::utils::bitfield::*;
use log::trace;

const MASTER_PIC_COM: u16 = 0x20;
const SLAVE_PIC_COM: u16 = 0xA0;

// PIC offset in IDT
pub const PIC_OFFSET: usize = 32;
pub const PIT_IRQ: usize = 0;
pub const KEYBOARD_IRQ: usize = 1;

#[derive(Clone, Copy)]
pub struct ICW1(u8);

impl ICW1 {
    pub fn new(has_icw4: bool, single: bool, adi: bool, ltim: bool) -> Self {
        Self(
            0_u8.set_bit(0, has_icw4)
                .set_bit(1, single)
                .set_bit(2, adi)
                .set_bit(3, ltim)
                .set_bit(4, true),
        )
    }
}

impl From<ICW1> for u8 {
    fn from(value: ICW1) -> Self {
        value.0
    }
}

#[derive(PartialEq)]
enum PicRole {
    SINGLE,
    SLAVE,
    MASTER,
}

pub struct PIC8259a {
    com_port: Port<u8>,
    data_port: Port<u8>,
    pin: Option<u8>,
    offset: u8,
    role: PicRole,
}

impl PIC8259a {
    const fn single(addr: u16, offset: u8) -> Self {
        if offset % 8 != 0 {
            panic!("8259a: offset must be a multiple of 8.");
        }

        Self {
            com_port: Port::new(addr),
            data_port: Port::new(addr + 1),
            pin: None,
            offset: offset,
            role: PicRole::SINGLE,
        }
    }

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
            role: PicRole::MASTER,
        }
    }

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
            role: PicRole::SLAVE,
        }
    }

    pub unsafe fn send_icw1(&self, has_icw4: bool, adi: bool, ltim: bool) {
        let com = ICW1::new(has_icw4, self.role == PicRole::SINGLE, adi, ltim);
        self.com_port.write(com.into());
    }

    pub unsafe fn send_icw2(&self) {
        self.data_port.write(self.offset);
    }

    pub unsafe fn send_icw3(&self) {
        match self.role {
            PicRole::SINGLE => panic!("ICW3: Nothing to be sent in single mode."),
            PicRole::MASTER => self
                .data_port
                .write(0_u8.set_bit(self.pin.unwrap().into(), true)),
            PicRole::SLAVE => self.data_port.write(self.pin.unwrap()),
        };
    }

    pub unsafe fn send_icw4(&self, aeoi: bool, buffered: bool, fully_nested: bool) {
        let com = 0_u8
            .set_bit(0, true)
            .set_bit(1, aeoi)
            .set_bit(2, self.role == PicRole::MASTER)
            .set_bit(3, buffered)
            .set_bit(4, fully_nested);
        self.data_port.write(com);
    }

    pub unsafe fn send_ocw1(&self, mask: u8) {
        self.data_port.write(mask);
    }

    pub unsafe fn ack_eoi(&self) {
        self.com_port.write(0x20_u8);
    }
}

static MASTER_PIC: PIC8259a = PIC8259a::master(MASTER_PIC_COM, 2, PIC_OFFSET as u8);
static SLAVE_PIC: PIC8259a = PIC8259a::slave(SLAVE_PIC_COM, 2, (PIC_OFFSET + 8) as u8);

pub fn setup_pic(master_mask: u8, slave_mask: u8) {
    if PIC_OFFSET % 8 != 0 {
        panic!("PIC offset must be a multiple of 8.");
    }

    trace!("Setting up 8259a PICs...");

    unsafe {
        MASTER_PIC.send_icw1(true, false, false);
        SLAVE_PIC.send_icw1(true, false, false);

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

pub unsafe fn ack_eoi(irq: u8) {
    if irq >= 8 {
        SLAVE_PIC.ack_eoi();
    }
    MASTER_PIC.ack_eoi();
}
