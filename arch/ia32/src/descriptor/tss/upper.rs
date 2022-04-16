/// The collection of all segment descriptor's upper part field offsets.
pub mod offsets {
    /// Base address middle part offset (16:23)
    pub const BASE_MID: u8 = 0;
    /// Segment type offset
    pub const TYPE: u8 = 8;
    /// Descriptor privilege level offset
    pub const DPL: u8 = 13;
    /// Segment present bit offset
    pub const P: u8 = 15;
    /// Limit high part offset (16:19)
    pub const LIMIT_HIGH: u8 = 16;
    /// Available system bit offset
    pub const AVL: u8 = 20;
    /// Granularity bit offset
    pub const G: u8 = 23;
    /// Base address high part offset (24:31)
    pub const BASE_HIGH: u8 = 24;
}

/// The collection of all segment descriptor's upper part bits representation.
pub mod bits {
    pub const BASE_MID: u32 = 0xff;
    pub const TYPE: u32 = 0xf;
    pub const DPL: u32 = 0x3;
    pub const P: u32 = 0x1;
    pub const LIMIT_HIGH: u32 = 0xf;
    pub const AVL: u32 = 0x1;
    pub const G: u32 = 0x1;
    pub const BASE_HIGH: u32 = 0xff;
}

/// The collection of all segment descriptor's upper part flags.
pub mod flags {
    use super::{bits, offsets};
    /// Base address middle part bitmask (16:23)
    pub const BASE_MID: u32 = offset!(BASE_MID);
    /// Segment type bitmask
    pub const TYPE: u32 = offset!(TYPE);
    /// Descriptor privilege level bitmask
    pub const DPL: u32 = offset!(DPL);
    /// Segment present bitmask
    pub const P: u32 = offset!(P);
    /// Limit high part bitmask (16:19)
    pub const LIMIT_HIGH: u32 = offset!(LIMIT_HIGH);
    /// Available system bit bitmask
    pub const AVL: u32 = offset!(AVL);
    /// Granularity bitmask
    pub const G: u32 = offset!(G);
    /// Base address high part bitmask (24:31)
    pub const BASE_HIGH: u32 = offset!(BASE_HIGH);
}

#[repr(C, packed)]
#[derive(Default, Copy, Clone, Debug)]
pub struct Upper(pub u32);

impl Upper {
    pub fn busy(self, value: bool) -> Self {
        Self(setbits!(self.0, if value { 0b1011 } else { 1001 }, TYPE))
    }

    pub fn privilege_level(self, value: u32) -> Self {
        Self(setbits!(self.0, value, DPL))
    }

    pub fn present(self, value: u32) -> Self {
        Self(setbits!(self.0, value, P))
    }

    pub fn available(self, value: u32) -> Self {
        Self(setbits!(self.0, value, AVL))
    }

    pub fn granularity(self, value: u32) -> Self {
        Self(setbits!(self.0, value, G))
    }

    pub fn base_mid(self, value: u32) -> Self {
        Self(setbits!(self.0, value, BASE_MID))
    }

    pub fn base_high(self, value: u32) -> Self {
        Self(setbits!(self.0, value, BASE_HIGH))
    }

    pub fn limit_high(self, value: u32) -> Self {
        Self(setbits!(self.0, value, LIMIT_HIGH))
    }
}
