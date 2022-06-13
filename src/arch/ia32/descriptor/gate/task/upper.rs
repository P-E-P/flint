pub mod offsets {
    /// Descriptor privilege level offset.
    pub const DPL: u8 = 13;
    /// Segment present bit offset.
    pub const P: u8 = 15;
}

/// The collection of all task gate descriptor's upper part bits representation.
pub mod bits {
    pub const DPL: u32 = 0x3;
    pub const P: u32 = 0x1;
}

pub mod flags {
    use super::{bits, offsets};
    pub const DPL: u32 = offset!(DPL);
    pub const P: u32 = offset!(P);
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct Upper(pub u32);

impl Default for Upper {
    fn default() -> Self {
        // Set task gate's "101" bits in the proper field.
        Self(0x5 << 8)
    }
}

impl Upper {
    pub fn present(self, value: u32) -> Self {
        Self(setbits!(self.0, value, P))
    }

    pub fn privilege_level(self, value: u32) -> Self {
        Self(setbits!(self.0, value, DPL))
    }
}
