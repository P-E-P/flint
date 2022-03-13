use super::descriptor::{
    DefaultOperationSize, DescriptorType, Granularity, PrivilegeLevel, SegmentDescriptor,
    SegmentType,
};
use core::arch::asm;
use core::ptr::addr_of;
use core::{fmt, mem};
use log::{debug, trace};

const GDT_LEN: usize = 5;

pub struct GlobalDescriptorTable([SegmentDescriptor; GDT_LEN]);

impl GlobalDescriptorTable {
    fn load(self) {
        trace!("Loading global descriptor table...");

        let gdtr = GlobalDescriptorTableRegister::new(
            (GDT_LEN * mem::size_of::<SegmentDescriptor>())
                .try_into()
                .expect("Gdt length does not fit in a u16, cannot set GDTR"),
            addr_of!(self),
        );

        unsafe {
            asm!("lgdt [{}]", in(reg) addr_of!(gdtr));
        }
    }
}

impl fmt::Display for GlobalDescriptorTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for entry in self.0 {
            if let Err(why) = write!(f, "{}\n", entry) {
                return Err(why);
            }
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
    trace!("Setting up gdt...");
    let gdt = GlobalDescriptorTable([
        // Null segment
        SegmentDescriptor::default(),
        // Kernel code
        SegmentDescriptor::new(0, 0xFFFF)
            .segment_type(SegmentType::Code {
                accessed: false,
                read: true,
                conforming: false,
            })
            .descriptor_type(DescriptorType::CodeOrData)
            .ia32e_mode(true)
            .privilege_level(PrivilegeLevel::Kernel)
            .granularity(Granularity::FourKByte),
        // Kernel data
        SegmentDescriptor::new(0, 0xFFFF)
            .segment_type(SegmentType::Data {
                accessed: false,
                write: true,
                expand_down: false,
            })
            .descriptor_type(DescriptorType::CodeOrData)
            .privilege_level(PrivilegeLevel::Kernel)
            .default_operation_size(DefaultOperationSize::Segment32Bits)
            .granularity(Granularity::FourKByte),
        // User code
        SegmentDescriptor::new(0, 0xFFFF)
            .segment_type(SegmentType::Code {
                accessed: false,
                read: true,
                conforming: false,
            })
            .descriptor_type(DescriptorType::CodeOrData)
            .ia32e_mode(true)
            .privilege_level(PrivilegeLevel::Userland)
            .granularity(Granularity::FourKByte),
        // User data
        SegmentDescriptor::new(0, 0xFFFF)
            .segment_type(SegmentType::Data {
                accessed: false,
                write: true,
                expand_down: false,
            })
            .descriptor_type(DescriptorType::CodeOrData)
            .privilege_level(PrivilegeLevel::Userland)
            .default_operation_size(DefaultOperationSize::Segment32Bits)
            .granularity(Granularity::FourKByte),
    ]);
    debug!("GDT:\n{}", gdt);
    gdt.load();
}
