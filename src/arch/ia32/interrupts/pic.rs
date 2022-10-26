use log::trace;
use pic8259a::{AddressInterval, PIC8259a, TriggerMode, MASTER_PIC_COM, SLAVE_PIC_COM};
pub use pic8259a::{KEYBOARD_IRQ, PIT_IRQ};

mod pic8259a;

/// PIC IRQs offset in IDT
pub const PIC_OFFSET: usize = 32;

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
        MASTER_PIC.send_icw1(true, AddressInterval::Eight, TriggerMode::Edge);
        SLAVE_PIC.send_icw1(true, AddressInterval::Eight, TriggerMode::Edge);

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
pub fn ack_eoi(irq: u8) {
    unsafe {
        if irq >= 8 {
            SLAVE_PIC.ack_eoi();
        }
        MASTER_PIC.ack_eoi();
    }
}
