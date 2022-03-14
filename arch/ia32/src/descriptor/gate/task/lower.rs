pub mod offsets {
    /// TSS Segment selector
    pub const TSS_SEGMENT_SELECTOR: u8 = 16;
}

pub mod bits {
    pub const TSS_SEGMENT_SELECTOR: u32 = 0xffff;
}

pub mod flags {
    use super::{bits, offsets};
    pub const TSS_SEGMENT_SELECTOR: u32 = offset!(TSS_SEGMENT_SELECTOR);
}

#[repr(C, packed)]
#[derive(Default, Copy, Clone, Debug)]
pub struct Lower(pub u32);

impl Lower {
    pub fn tss_segment_selector(self, value: u32) -> Self {
        Self(setbits!(self.0, value, TSS_SEGMENT_SELECTOR))
    }
}
