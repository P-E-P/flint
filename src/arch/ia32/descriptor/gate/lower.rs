//! Common lower part for trap gate and interrupt gate.
//! This module provides the common structure for the lower part of the trap
//! gate descriptor and the interrupt gate descriptor.

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

#[must_use]
#[repr(C, packed)]
#[derive(Default, Copy, Clone, Debug)]
pub struct Lower(pub u32);

impl Lower {
    pub fn segment_selector(self, value: u32) -> Self {
        Self(setbits!(self.0, value, SEGMENT_SELECTOR))
    }

    pub fn offset_low(self, value: u32) -> Self {
        Self(setbits!(self.0, value, OFFSET_LOW))
    }
}
