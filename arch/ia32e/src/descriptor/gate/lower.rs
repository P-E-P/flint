pub mod offsets {
    pub const OFFSET_LOW: u8 = 0;
    pub const SEGMENT_SELECTOR: u8 = 16;
}

pub mod bits {
    pub const OFFSET_LOW: u32 = 0xffff;
    pub const SEGMENT_SELECTOR: u32 = 0xffff;
}

pub mod flags {
    use super::{bits, offsets};
    pub const OFFSET_LOW: u32 = offset!(OFFSET_LOW);
    pub const SEGMENT_SELECTOR: u32 = offset!(SEGMENT_SELECTOR);
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Lower(u32);

impl Lower {
    pub fn offset_low(self, value: u32) -> Self {
        Self(setbits!(self.0, value, OFFSET_LOW))
    }

    pub fn segment_selector(self, value: u32) -> Self {
        Self(setbits!(self.0, value, SEGMENT_SELECTOR))
    }
}
