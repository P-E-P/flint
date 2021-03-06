use crate::arch::ia32::selector::SegmentSelector;

#[repr(C, packed)]
pub struct TaskStateSegment {
    previous_task: u16,
    reserved_1: u16,

    esp0: u32,

    ss0: u16,
    reserved_2: u16,

    esp1: u32,

    ss1: u16,
    reserved_3: u16,

    esp2: u32,

    ss2: u16,
    reserved_4: u16,

    cr3: u32,
    eip: u32,
    eflags: u32,
    eax: u32,
    ecx: u32,
    edx: u32,
    ebx: u32,
    esp: u32,

    ebp: u32,
    esi: u32,
    edi: u32,

    es: u16,
    reserved_5: u16,

    cs: u16,
    reserved_6: u16,

    ss: u16,
    reserved_7: u16,

    ds: u16,
    reserved_8: u16,

    fs: u16,
    reserved_9: u16,

    gs: u16,
    reserved_10: u16,

    ldt_segment_selector: SegmentSelector,
    reserved_11: u16,

    reserved_12: u16,
    io_map_base_address: u16,

    ssp: u32,
}
