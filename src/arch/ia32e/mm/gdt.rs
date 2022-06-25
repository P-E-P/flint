use crate::arch::ia32::descriptor::segment::{
    DefaultOperationSize, DescriptorType, Granularity, PrivilegeLevel, SegmentDescriptor,
    SegmentType,
};
use crate::arch::ia32::mm::gdt::GlobalDescriptorTable;
use log::{debug, trace};

pub fn setup_gdt() {
    trace!("Setting up 64bits gdt...");
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
