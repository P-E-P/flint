use descriptor::{ SegmentDescriptor, DescriptorType, PrivilegeLevel, DefaultOperationSize, Granularity, SegmentType};

pub mod descriptor;

const GDT_LEN: usize = 5;

/// Global descriptor table register (Intel III 2.4.1).
#[repr(C, packed)]
struct GlobalDescriptorTableRegister {
    /// Table limit specifying the number of bytes in the table.
    size: u16,
    /// Linear base address. Should be 32 bits in protected mode and 64 bits
    /// in IA-32e mode.
    offset: usize,
}

fn setup_gdt() {
    let gdt: [SegmentDescriptor; GDT_LEN] = [
        // Null segment
        SegmentDescriptor::default(),
        // Kernel code
        SegmentDescriptor::new(0, 0)
            .present(true)
            .segment_type(SegmentType::Code {
                accessed: false,
                read: true,
                conforming: false,
            })
            .descriptor_type(DescriptorType::CodeOrData)
            .ia32e_mode(true)
            .privilege_level(PrivilegeLevel::Kernel)
            .default_operation_size(DefaultOperationSize::Segment32Bits)
            .granularity(Granularity::FourKByte),
        // Kernel data
        SegmentDescriptor::new(0, 0)
            .present(true)
            .segment_type(SegmentType::Data {
                accessed: false,
                write: true,
                expand_down: false,
            })
            .descriptor_type(DescriptorType::CodeOrData)
            .ia32e_mode(true)
            .privilege_level(PrivilegeLevel::Kernel)
            .default_operation_size(DefaultOperationSize::Segment32Bits)
            .granularity(Granularity::FourKByte),
        // User code
        SegmentDescriptor::new(0, 0).present(true),
        // User data
        SegmentDescriptor::new(0, 0).present(true),
    ];
}
