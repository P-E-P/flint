use crate::descriptor::gate::Gate;
use crate::selector::{SegmentSelector, TableIndicator};
use crate::PrivilegeLevel;
use core::arch::asm;
use core::ptr::addr_of;
use core::{fmt, mem};
use log::{debug, trace};

const IDT_LEN: usize = 256;
const GDT_KERNEL_CODE: u16 = 1;

pub struct InterruptDescriptorTable {
    pub entries: [Gate; IDT_LEN],
}

impl Default for InterruptDescriptorTable {
    fn default() -> Self {
        Self {
            entries: [Gate::default(); IDT_LEN],
        }
    }
}

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
        for entry in self.entries {
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

fn setup_predefined(idt: &mut InterruptDescriptorTable) {
    let kernel_segment =
        SegmentSelector::new(GDT_KERNEL_CODE, TableIndicator::GDT, PrivilegeLevel::Kernel);

    // Divide error
    idt.entries[0] = Gate::new(0, kernel_segment).present(true);
    // Debug exception
    idt.entries[1] = Gate::new(0, kernel_segment).present(true);
    // NMI interrupt
    idt.entries[2] = Gate::new(0, kernel_segment).present(true);
    // Breakpoint
    idt.entries[3] = Gate::new(0, kernel_segment).present(true);
    // Overflow
    idt.entries[4] = Gate::new(0, kernel_segment).present(true);
    // Bound range exceeded
    idt.entries[5] = Gate::new(0, kernel_segment).present(true);
    // Invalid OP code
    idt.entries[6] = Gate::new(0, kernel_segment).present(true);
    // Device not available
    idt.entries[7] = Gate::new(0, kernel_segment).present(true);
    // Double fault
    idt.entries[8] = Gate::new(0, kernel_segment).present(true);
    // Coprocessor segment overrun
    idt.entries[9] = Gate::new(0, kernel_segment).present(true);
    // Invalid TSS
    idt.entries[10] = Gate::new(0, kernel_segment).present(true);
    // Segment not present
    idt.entries[11] = Gate::new(0, kernel_segment).present(true);
    // Stack segment fault
    idt.entries[12] = Gate::new(0, kernel_segment).present(true);
    // General protection
    idt.entries[13] = Gate::new(0, kernel_segment).present(true);
    // Page fault
    idt.entries[14] = Gate::new(0, kernel_segment).present(true);
    // x87 fpu floating point error
    idt.entries[16] = Gate::new(0, kernel_segment).present(true);
    // Alignment check
    idt.entries[17] = Gate::new(0, kernel_segment).present(true);
    // Machine check
    idt.entries[18] = Gate::new(0, kernel_segment).present(true);
    // SIMD floating point Exception
    idt.entries[19] = Gate::new(0, kernel_segment).present(true);
    // Virtualization exception
    idt.entries[20] = Gate::new(0, kernel_segment).present(true);
    // Control protection exception
    idt.entries[21] = Gate::new(0, kernel_segment).present(true);
}

pub fn setup_idt() {
    trace!("Setting up idt...");

    let mut idt = InterruptDescriptorTable::default();
    setup_predefined(&mut idt);

    idt.load();

    todo!("Set up 64 bit idt");
}
