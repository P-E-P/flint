use crate::arch::ia32::descriptor::segment::SegmentDescriptor;
use core::arch::asm;
use core::ptr::addr_of;
use core::{fmt, mem};
use log::trace;

const GDT_LEN: usize = 5;

pub struct GlobalDescriptorTable(pub [SegmentDescriptor; GDT_LEN]);

impl GlobalDescriptorTable {
    pub fn load(&'static self) {
        trace!("Loading global descriptor table...");

        let gdtr = GlobalDescriptorTableRegister::new(
            (GDT_LEN * mem::size_of::<SegmentDescriptor>())
                .try_into()
                .expect("Gdt length does not fit in a u16, cannot set GDTR"),
            self,
        );

        unsafe {
            asm!("lgdt [{}]", in(reg) addr_of!(gdtr));
        }
    }
}

impl fmt::Display for GlobalDescriptorTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for entry in self.0 {
            writeln!(f, "{}", entry)?;
        }
        Ok(())
    }
}

/// Global descriptor table register (Intel III 2.4.1).
#[repr(C, packed)]
struct GlobalDescriptorTableRegister {
    /// Table limit specifying the number of bytes in the table.
    size: u16,
    /// Linear base address. Should be 32 bits in protected mode and 64 bits
    /// in IA-32e mode.
    offset: *const GlobalDescriptorTable,
}

impl GlobalDescriptorTableRegister {
    pub fn new(size: u16, address: *const GlobalDescriptorTable) -> Self {
        GlobalDescriptorTableRegister {
            size,
            offset: address,
        }
    }
}

pub fn setup_gdt() {
    trace!("Setting up 32bits gdt...");
    todo!("Implement 32 bit GDT");
}
