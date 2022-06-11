pub mod offsets {
    pub const D: u8 = 11;
    pub const DPL: u8 = 13;
    pub const P: u8 = 15;
    pub const OFFSET_HIGH: u8 = 16;
}

pub mod bits {
    pub const D: u32 = 0x1;
    pub const DPL: u32 = 0x3;
    pub const P: u32 = 0x1;
    pub const OFFSET_HIGH: u32 = 0xffff;
}

pub mod flags {
    use super::{bits, offsets};
    pub const D: u32 = offset!(D);
    pub const DPL: u32 = offset!(DPL);
    pub const P: u32 = offset!(P);
    pub const OFFSET_HIGH: u32 = offset!(OFFSET_HIGH);
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct Upper(pub u32);

impl Default for Upper {
    fn default() -> Self {
        // Set interrupt gate's "111" bits in the proper field.
        Self(0x7 << 8)
    }
}

impl Upper {
    pub fn present(self, value: u32) -> Self {
        Self(setbits!(self.0, value, P))
    }

    pub fn privilege_level(self, value: u32) -> Self {
        Self(setbits!(self.0, value, DPL))
    }

    pub fn size(self, value: u32) -> Self {
        Self(setbits!(self.0, value, D))
    }

    pub fn offset_high(self, value: u32) -> Self {
        Self(setbits!(self.0, value, OFFSET_HIGH))
    }
}
