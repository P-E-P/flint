pub mod offsets {
    pub const IST: u8 = 0;
    /// Gate type (interrupt, trap...)
    pub const TYPE: u8 = 8;
    /// Descriptor privilege level offset.
    pub const DPL: u8 = 13;
    /// Segment present bit offset.
    pub const P: u8 = 15;
    pub const OFFSET_MID: u8 = 16;
}

pub mod bits {
    pub const IST: u32 = 0x7;
    pub const TYPE: u32 = 0xf;
    pub const DPL: u32 = 0x3;
    pub const P: u32 = 0x1;
    pub const OFFSET_MID: u32 = 0xffff;
}

pub mod flags {
    use super::{bits, offsets};
    pub const IST: u32 = offset!(IST);
    pub const TYPE: u32 = offset!(TYPE);
    pub const DPL: u32 = offset!(DPL);
    pub const P: u32 = offset!(P);
    pub const OFFSET_MID: u32 = offset!(OFFSET_MID);
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Upper(u32);

impl Upper {
    pub fn kind(self, value: u32) -> Self {
        Self(setbits!(self.0, value, TYPE))
    }

    pub fn present(self, value: u32) -> Self {
        Self(setbits!(self.0, value, P))
    }

    pub fn privilege_level(self, value: u32) -> Self {
        Self(setbits!(self.0, value, DPL))
    }

    pub fn offset_mid(self, value: u32) -> Self {
        Self(setbits!(self.0, value, OFFSET_MID))
    }

    pub fn interrupt_stack_table(self, value: u32) -> Self {
        Self(setbits!(self.0, value, IST))
    }
}
