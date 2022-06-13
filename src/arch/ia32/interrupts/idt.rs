use crate::arch::ia32::descriptor::gate::Gate;
use core::arch::asm;
use core::ptr::addr_of;
use core::{fmt, mem};
use log::trace;

const IDT_LEN: usize = 256;

pub struct InterruptDescriptorTable(pub [Gate; IDT_LEN]);

impl InterruptDescriptorTable {
    pub fn load(self) {
        trace!("Loading interrupt descriptor table...");

        let idtr = InterruptDescriptorTableRegister::new(
            (IDT_LEN * mem::size_of::<Gate>())
                .try_into()
                .expect("Idt length does not fit in a u16, cannot set IDTR"),
            addr_of!(self),
        );

        unsafe {
            asm!("lidt [{}]", in(reg) addr_of!(idtr));
        }
    }
}

impl fmt::Display for InterruptDescriptorTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for entry in self.0 {
            if let Err(why) = writeln!(f, "{}", entry) {
                return Err(why);
            }
        }
        Ok(())
    }
}

struct InterruptDescriptorTableRegister {
    size: u16,
    offset: *const InterruptDescriptorTable,
}

impl InterruptDescriptorTableRegister {
    pub fn new(size: u16, address: *const InterruptDescriptorTable) -> Self {
        InterruptDescriptorTableRegister {
            size,
            offset: address,
        }
    }
}

pub fn setup_idt() {
    trace!("Setting up idt...");
    todo!("Implement idt setup");
}
