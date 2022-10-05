use crate::arch::ia32e::{
    descriptor::gate::Gate,
    selector::{SegmentSelector, TableIndicator},
    PrivilegeLevel,
    descriptor::gate::Kind
};
use core::arch::asm;
use core::ptr::addr_of;
use core::{fmt, mem};
use log::trace;

const IDT_LEN: usize = 256;
const GDT_KERNEL_CODE: u16 = 1;

pub struct InterruptDescriptorTable {
    pub entries: [Gate; IDT_LEN],
}

impl InterruptDescriptorTable {
    pub const fn const_default() -> Self {
        Self {
            entries: [Gate::const_default(); IDT_LEN],
        }
    }
}

impl Default for InterruptDescriptorTable {
    fn default() -> Self {
        Self::const_default()
    }
}

impl InterruptDescriptorTable {
    pub fn load(&'static self) {
        trace!("Loading interrupt descriptor table...");

        let idtr = InterruptDescriptorTableRegister::new(
            (IDT_LEN * mem::size_of::<Gate>())
                .try_into()
                .expect("Idt length does not fit in a u16, cannot set IDTR"),
            self,
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

#[repr(C, packed)]
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

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::const_default();

unsafe fn setup_predefined() {
    let kernel_segment =
        SegmentSelector::new(GDT_KERNEL_CODE, TableIndicator::GDT, PrivilegeLevel::Kernel);

    // Divide error
    IDT.entries[0] = Gate::new(div_by_zero as *const () as u64, kernel_segment).present(true).kind(Kind::Interrupt);
    // Debug exception
    IDT.entries[1] = Gate::new(0, kernel_segment).present(false);
    // NMI interrupt
    IDT.entries[2] = Gate::new(0, kernel_segment).present(false);
    // Breakpoint
    IDT.entries[3] = Gate::new(breakpoint as *const () as u64, kernel_segment).present(true).kind(Kind::Interrupt);
    // Overflow
    IDT.entries[4] = Gate::new(0, kernel_segment).present(false);
    // Bound range exceeded
    IDT.entries[5] = Gate::new(0, kernel_segment).present(false);
    // Invalid OP code
    IDT.entries[6] = Gate::new(0, kernel_segment).present(false);
    // Device not available
    IDT.entries[7] = Gate::new(0, kernel_segment).present(false);
    // Double fault
    IDT.entries[8] = Gate::new(double_fault as *const () as u64, kernel_segment).present(true).kind(Kind::Interrupt);
    // Coprocessor segment overrun
    IDT.entries[9] = Gate::new(0, kernel_segment).present(false);
    // Invalid TSS
    IDT.entries[10] = Gate::new(0, kernel_segment).present(false);
    // Segment not present
    IDT.entries[11] = Gate::new(0, kernel_segment).present(false);
    // Stack segment fault
    IDT.entries[12] = Gate::new(0, kernel_segment).present(false);
    // General protection
    IDT.entries[13] = Gate::new(0, kernel_segment).present(false);
    // Page fault
    IDT.entries[14] = Gate::new(0, kernel_segment).present(false);
    // x87 fpu floating point error
    IDT.entries[16] = Gate::new(0, kernel_segment).present(false);
    // Alignment check
    IDT.entries[17] = Gate::new(0, kernel_segment).present(false);
    // Machine check
    IDT.entries[18] = Gate::new(0, kernel_segment).present(false);
    // SIMD floating point Exception
    IDT.entries[19] = Gate::new(0, kernel_segment).present(false);
    // Virtualization exception
    IDT.entries[20] = Gate::new(0, kernel_segment).present(false);
    // Control protection exception
    IDT.entries[21] = Gate::new(0, kernel_segment).present(false);
}

pub fn setup_idt() {
    unsafe {
        trace!("Setting up idt...");
        setup_predefined();
        trace!("Loading idt...");
        IDT.load();
    }

    /*unsafe {
        asm!("int3", options(nomem, nostack));
    }*/

    loop {

    }
}

extern "x86-interrupt" fn div_by_zero() {
    panic!("Division by zero");
}

extern "x86-interrupt" fn breakpoint() {
    panic!("Breakpoint");
}

extern "x86-interrupt" fn double_fault() {
    panic!("Double fault");
}
