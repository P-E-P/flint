use crate::arch::ia32::address::VirtualAddress;
use crate::arch::ia32::interrupts::pic;
use crate::arch::ia32::interrupts::pic::*;
use crate::arch::ia32::interrupts::pit8254;
use crate::arch::ia32::interrupts::pit8254::*;
use crate::arch::ia32e::{
    descriptor::gate::Gate,
    interrupts::frame::InterruptStackFrame,
    selector::{SegmentSelector, TableIndicator},
    PrivilegeLevel,
};
use crate::arch::in_byte;
use crate::utils::bitfield::*;

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
    IDT.entries[0] = Gate::interrupt(VirtualAddress::from_handler(div_by_zero), kernel_segment);
    // Debug exception
    IDT.entries[1] = Gate::interrupt(
        VirtualAddress::from_handler(debug_exception),
        kernel_segment,
    );
    // NMI interrupt
    IDT.entries[2] = Gate::interrupt(VirtualAddress::from_handler(nmi), kernel_segment);
    // Breakpoint
    IDT.entries[3] = Gate::interrupt(VirtualAddress::from_handler(breakpoint), kernel_segment);
    // Overflow
    IDT.entries[4] = Gate::interrupt(VirtualAddress::from_handler(overflow), kernel_segment);
    // Bound range exceeded
    IDT.entries[5] = Gate::interrupt(VirtualAddress::from_handler(bound_range), kernel_segment);
    // Invalid OP code
    IDT.entries[6] = Gate::interrupt(VirtualAddress::from_handler(invalid_op), kernel_segment);
    // Device not available
    IDT.entries[7] = Gate::interrupt(VirtualAddress::from_handler(device_na), kernel_segment);
    // Double fault
    IDT.entries[8] = Gate::interrupt(
        VirtualAddress::from_handler_with_err(double_fault),
        kernel_segment,
    );
    // Coprocessor segment overrun
    IDT.entries[9] = Gate::interrupt(VirtualAddress::from_handler(coproc_overrun), kernel_segment);
    // Invalid TSS
    IDT.entries[10] = Gate::interrupt(
        VirtualAddress::from_handler_with_err(invalid_tss),
        kernel_segment,
    );
    // Segment not present
    IDT.entries[11] = Gate::interrupt(
        VirtualAddress::from_handler_with_err(segment_not_present),
        kernel_segment,
    );
    // Stack segment fault
    IDT.entries[12] = Gate::interrupt(
        VirtualAddress::from_handler_with_err(stack_fault),
        kernel_segment,
    );
    // General protection
    IDT.entries[13] = Gate::interrupt(
        VirtualAddress::from_handler_with_err(general_fault),
        kernel_segment,
    );
    // Page fault
    IDT.entries[14] = Gate::interrupt(
        VirtualAddress::from_handler_with_err(page_fault),
        kernel_segment,
    );
    // x87 fpu floating point error
    IDT.entries[16] = Gate::interrupt(VirtualAddress::from_handler(x87_fpe), kernel_segment);
    // Alignment check
    IDT.entries[17] = Gate::interrupt(
        VirtualAddress::from_handler_with_err(alignment_check),
        kernel_segment,
    );
    // Machine check
    IDT.entries[18] = Gate::interrupt(VirtualAddress::from_handler(machine_check), kernel_segment);
    // SIMD floating point Exception
    IDT.entries[19] = Gate::interrupt(VirtualAddress::from_handler(simd_fpe), kernel_segment);
    // Virtualization exception
    IDT.entries[20] = Gate::interrupt(VirtualAddress::from_handler(virt_exception), kernel_segment);
    // Control protection exception
    IDT.entries[21] = Gate::interrupt(
        VirtualAddress::from_handler_with_err(control_protection),
        kernel_segment,
    );

    // PIT interrupt
    IDT.entries[PIC_OFFSET + PIT_IRQ] =
        Gate::interrupt(VirtualAddress::from_handler(pit), kernel_segment);
    // Keyboard interrupt
    IDT.entries[PIC_OFFSET + KEYBOARD_IRQ] =
        Gate::interrupt(VirtualAddress::from_handler(keyboard), kernel_segment);
}

pub fn setup_idt() {
    unsafe {
        trace!("Setting up idt...");
        pic::setup(
            0b11111111_u8
                .set_bit(KEYBOARD_IRQ, false)
                .set_bit(PIT_IRQ, false),
            0b11111111,
        );
        pit8254::setup();
        setup_predefined();
        trace!("Loading idt...");
        IDT.load();
    }
}

extern "x86-interrupt" fn div_by_zero(_frame: InterruptStackFrame) {
    panic!("Division by zero!");
}

extern "x86-interrupt" fn debug_exception(_frame: InterruptStackFrame) {
    panic!("Debug exception");
}

extern "x86-interrupt" fn nmi(_frame: InterruptStackFrame) {
    panic!("Non-Maskable Interrupt");
}

extern "x86-interrupt" fn breakpoint(_frame: InterruptStackFrame) {
    panic!("Breakpoint!");
}

extern "x86-interrupt" fn overflow(_frame: InterruptStackFrame) {
    panic!("Overflow occured");
}

extern "x86-interrupt" fn bound_range(_frame: InterruptStackFrame) {
    panic!("Bound range exceeded");
}

extern "x86-interrupt" fn invalid_op(_frame: InterruptStackFrame) {
    panic!("Invalid opcode");
}

extern "x86-interrupt" fn device_na(_frame: InterruptStackFrame) {
    panic!("Device not available");
}

extern "x86-interrupt" fn double_fault(frame: InterruptStackFrame, _err: u64) {
    panic!("Double fault!\n{}", frame);
}

extern "x86-interrupt" fn coproc_overrun(_frame: InterruptStackFrame) {
    panic!("Coprocessor segment overrun");
}

extern "x86-interrupt" fn invalid_tss(_frame: InterruptStackFrame, _err: u64) {
    panic!("Invalid TSS");
}

extern "x86-interrupt" fn segment_not_present(_frame: InterruptStackFrame, _err: u64) {
    panic!("Segment not present");
}

extern "x86-interrupt" fn stack_fault(_frame: InterruptStackFrame, _err: u64) {
    panic!("Stack segment fault");
}

extern "x86-interrupt" fn general_fault(_frame: InterruptStackFrame, _err: u64) {
    panic!("General protection fault");
}

extern "x86-interrupt" fn page_fault(_frame: InterruptStackFrame, _err: u64) {
    panic!("Page fault");
}

extern "x86-interrupt" fn x87_fpe(_frame: InterruptStackFrame) {
    panic!("Floating point exception");
}

extern "x86-interrupt" fn alignment_check(_frame: InterruptStackFrame, _err: u64) {
    panic!("Unaligned memory data reference");
}

extern "x86-interrupt" fn machine_check(_frame: InterruptStackFrame) {
    panic!("Machine check exception");
}

extern "x86-interrupt" fn simd_fpe(_frame: InterruptStackFrame) {
    panic!("Floating point exception");
}

extern "x86-interrupt" fn virt_exception(_frame: InterruptStackFrame) {
    panic!("Virtualization exception");
}

extern "x86-interrupt" fn control_protection(_frame: InterruptStackFrame, _err: u64) {
    panic!("Control protection exception");
}

extern "x86-interrupt" fn keyboard(_frame: InterruptStackFrame) {
    unsafe {
        let scancode: u8 = in_byte(0x60_u16);
        if scancode.get_bit(7) {
            println!("Key pressed!");
        }
        ack_eoi(KEYBOARD_IRQ as u8);
    }
}

extern "x86-interrupt" fn pit(_frame: InterruptStackFrame) {
    unsafe {
        TICK_COUNTER.increment();
        ack_eoi(PIT_IRQ as u8);
    }
}
