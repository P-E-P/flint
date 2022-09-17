use crate::arch::ia32::address::VirtualAddress;
use crate::arch::ia32e::{
    descriptor::gate::Gate,
    selector::{SegmentSelector, TableIndicator},
    PrivilegeLevel,
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

fn setup_predefined(idt: &mut InterruptDescriptorTable) {
    let kernel_segment =
        SegmentSelector::new(GDT_KERNEL_CODE, TableIndicator::GDT, PrivilegeLevel::Kernel);

    // Divide error
    idt.entries[0] = Gate::new(VirtualAddress::from_fn(div_by_zero), kernel_segment).present(true);
    // Debug exception
    idt.entries[1] = Gate::new(VirtualAddress::from_fn(debug_exception), kernel_segment).present(true);
    // NMI interrupt
    idt.entries[2] = Gate::new(VirtualAddress::from_fn(nmi), kernel_segment).present(true);
    // Breakpoint
    idt.entries[3] = Gate::new(VirtualAddress::from_fn(breakpoint), kernel_segment).present(true);
    // Overflow
    idt.entries[4] = Gate::new(VirtualAddress::from_fn(overflow), kernel_segment).present(true);
    // Bound range exceeded
    idt.entries[5] = Gate::new(VirtualAddress::from_fn(bound_range), kernel_segment).present(true);
    // Invalid OP code
    idt.entries[6] = Gate::new(VirtualAddress::from_fn(invalid_op), kernel_segment).present(true);
    // Device not available
    idt.entries[7] = Gate::new(VirtualAddress::from_fn(device_na), kernel_segment).present(true);
    // Double fault
    idt.entries[8] = Gate::new(VirtualAddress::from_fn(double_fault), kernel_segment).present(true);
    // Coprocessor segment overrun
    idt.entries[9] = Gate::new(VirtualAddress::from_fn(coproc_overrun), kernel_segment).present(true);
    // Invalid TSS
    idt.entries[10] = Gate::new(VirtualAddress::from_fn(invalid_tss), kernel_segment).present(true);
    // Segment not present
    idt.entries[11] = Gate::new(VirtualAddress::from_fn(segment_not_present), kernel_segment).present(true);
    // Stack segment fault
    idt.entries[12] = Gate::new(VirtualAddress::from_fn(stack_fault), kernel_segment).present(true);
    // General protection
    idt.entries[13] = Gate::new(VirtualAddress::from_fn(general_fault), kernel_segment).present(true);
    // Page fault
    idt.entries[14] = Gate::new(VirtualAddress::from_fn(page_fault), kernel_segment).present(true);
    // x87 fpu floating point error
    idt.entries[16] = Gate::new(VirtualAddress::from_fn(x87_fpe), kernel_segment).present(true);
    // Alignment check
    idt.entries[17] = Gate::new(VirtualAddress::from_fn(alignment_check), kernel_segment).present(true);
    // Machine check
    idt.entries[18] = Gate::new(VirtualAddress::from_fn(machine_check), kernel_segment).present(true);
    // SIMD floating point Exception
    idt.entries[19] = Gate::new(VirtualAddress::from_fn(simd_fpe), kernel_segment).present(true);
    // Virtualization exception
    idt.entries[20] = Gate::new(VirtualAddress::from_fn(virt_exception), kernel_segment).present(true);
    // Control protection exception
    idt.entries[21] = Gate::new(VirtualAddress::from_fn(control_protection), kernel_segment).present(true);
    // Hypervisor injection exception
    idt.entries[28] = Gate::new(VirtualAddress::from_fn(hypervisor_injection), kernel_segment).present(true);
    // VMM communication exception
    idt.entries[29] = Gate::new(VirtualAddress::from_fn(vmm_communication), kernel_segment).present(true);
    // Security exception
    idt.entries[30] = Gate::new(VirtualAddress::from_fn(security_exception), kernel_segment).present(true);
}

pub fn setup_idt() {
    trace!("Setting up idt...");
    let mut idt = InterruptDescriptorTable::default();
    setup_predefined(&mut idt);

    todo!("Add idt loading");
    //idt.load();
}

extern "x86-interrupt" fn div_by_zero() {
    panic!("Division by zero");
}

extern "x86-interrupt" fn debug_exception() {
    panic!("Debug exception");
}

extern "x86-interrupt" fn nmi() {
    panic!("Non-Maskable Interrupt");
}

extern "x86-interrupt" fn breakpoint() {
    panic!("Breakpoint reached");
}

extern "x86-interrupt" fn overflow() {
    panic!("Overflow occured");
}

extern "x86-interrupt" fn bound_range() {
    panic!("Bound range exceeded");
}

extern "x86-interrupt" fn invalid_op() {
    panic!("Invalid opcode");
}

extern "x86-interrupt" fn device_na() {
    panic!("Device not available");
}

extern "x86-interrupt" fn double_fault() {
    panic!("Double fault");
}

extern "x86-interrupt" fn coproc_overrun() {
    panic!("Coprocessor segment overrun");
}

extern "x86-interrupt" fn invalid_tss() {
    panic!("Invalid TSS");
}

extern "x86-interrupt" fn segment_not_present() {
    panic!("Segment not present");
}

extern "x86-interrupt" fn stack_fault() {
    panic!("Stack segment fault");
}

extern "x86-interrupt" fn general_fault() {
    panic!("General protection fault");
}

extern "x86-interrupt" fn page_fault() {
    panic!("Page fault");
}

extern "x86-interrupt" fn x87_fpe() {
    panic!("Floating point exception");
}

extern "x86-interrupt" fn alignment_check() {
    panic!("Unaligned memory data reference");
}

extern "x86-interrupt" fn machine_check() {
    panic!("Machine check exception");
}

extern "x86-interrupt" fn simd_fpe() {
    panic!("Floating point exception");
}

extern "x86-interrupt" fn virt_exception() {
    panic!("Virtualization exception");
}

extern "x86-interrupt" fn control_protection() {
    panic!("Control protection exception");
}

extern "x86-interrupt" fn hypervisor_injection() {
    panic!("Hypervisor injection exception");
}

extern "x86-interrupt" fn vmm_communication() {
    panic!("VMM communication exception");
}

extern "x86-interrupt" fn security_exception() {
    panic!("Security exception");
}
